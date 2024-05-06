use std::path::{Path, PathBuf};

use anyhow::Result;
use common::usnob::{Observation, USNOBFile};

use common::usnob::USNOBObject;
use sqlx::{Connection, SqliteConnection};

use itertools::Itertools;

use crate::object::Object;

const INSERT_BATCH_SIZE: usize = 1000;

fn to_db_schema(obj: &USNOBObject, filename: &str) -> Object {
    let bmag = {
        let mut n = 0;
        let mut sum = 0.0;

        if let Some(Observation { mag, .. }) = obj.observations.blue1 {
            n += 1;
            sum += mag;
        }

        if let Some(Observation { mag, .. }) = obj.observations.blue2 {
            n += 1;
            sum += mag;
        }

        if n == 0 {
            None
        } else {
            Some(sum / n as f32)
        }
    };

    let rmag = {
        let mut n = 0;
        let mut sum = 0.0;
        if let Some(Observation { mag, .. }) = obj.observations.red1 {
            n += 1;
            sum += mag;
        }
        if let Some(Observation { mag, .. }) = obj.observations.red2 {
            n += 1;
            sum += mag;
        }

        if n == 0 {
            None
        } else {
            Some(sum / n as f32)
        }
    };

    let imag = obj.observations.infrared.as_ref().map(|o| o.mag);

    Object {
        usnob_id: obj.usnob_id.clone(),
        ra: obj.ra,
        sigma_ra: obj.sigma_ra,
        sigma_ra_fit: obj.sigma_ra_fit,
        pm_ra: obj.pm_ra,
        dec: obj.dec,
        sigma_dec: obj.sigma_dec,
        sigma_dec_fit: obj.sigma_dec_fit,
        pm_dec: obj.pm_dec,
        rmag,
        bmag,
        imag,
        epoch: obj.epoch,
        num_detections: obj.n_detections as i32,
        origin_file: filename.to_string(),
    }
}

pub fn get_files(paths: &[impl AsRef<Path>]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for path in paths {
        let path = path.as_ref();
        if path.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;
                if entry.path().is_file() && entry.path().extension() == Some("cat".as_ref()) {
                    files.push(entry.path());
                }
            }
        } else {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

pub async fn ingest_files(paths: &[impl AsRef<Path>]) -> Result<()> {
    let files = get_files(paths)?;

    let mut connection = SqliteConnection::connect(&dotenvy::var("DATABASE_URL")?).await?;

    for path in files {
        let file = USNOBFile::open(&path)?;
        let objects = file
            .iter()
            .map(|o| to_db_schema(&o, path.to_str().unwrap()));

        let n_objects = file.len().unwrap();

        println!("Ingesting file {:?} ({} objects)", path, n_objects);

        let start = std::time::Instant::now();

        for (idx, chunk) in objects.chunks(INSERT_BATCH_SIZE).into_iter().enumerate() {
            Object::insert_many(chunk, &mut connection).await?;
            let progress = ((idx * 1000) as f32 / n_objects as f32) * 100.0;
            print!("\r    Progress: {:5.2}% ", progress);
        }

        println!("\r    Done ({}s)        ", start.elapsed().as_secs());
    }

    Ok(())
}
