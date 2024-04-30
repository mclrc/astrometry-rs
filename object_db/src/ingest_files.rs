use diesel::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::Result;
use common::usnob::{Observation, USNOBFile};
use diesel::PgConnection;

use common::usnob::USNOBObject;

use crate::{object::Object, schema};

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

pub fn ingest_files(db: &mut PgConnection, paths: &[impl AsRef<Path>]) -> Result<()> {
    let files = paths
        .iter()
        .map(<_>::as_ref)
        .flat_map(|p| {
            if p.is_dir() {
                let mut files = Vec::new();
                for entry in p
                    .read_dir()
                    .unwrap_or_else(|_| panic!("Failed to read directory {:?}", p))
                {
                    let entry = entry.ok().unwrap_or_else(|| {
                        panic!("Failed to read file {:?}", p);
                    });
                    if entry.path().is_file() && entry.path().extension() == Some("cat".as_ref()) {
                        files.push(entry.path());
                    }
                }

                files
            } else {
                vec![p.to_path_buf()]
            }
        })
        .collect::<Vec<PathBuf>>();

    for path in files {
        let file = USNOBFile::open(&path)?;
        let objects = file
            .iter()
            .map(|o| to_db_schema(&o, path.to_str().unwrap()));

        let n_objects = file.len().unwrap();

        println!("Ingesting file {:?} ({} objects)", path, n_objects);

        let start = std::time::Instant::now();

        let mut batch = Vec::with_capacity(1000);

        for (idx, object) in objects.enumerate() {
            batch.push(object);

            if batch.len() < 1000 && idx + 1 < n_objects {
                continue;
            }

            diesel::insert_into(schema::object::table)
                .values(&batch)
                .on_conflict(schema::object::usnob_id)
                .do_nothing()
                .execute(db)?;

            batch.clear();
            let progress = (idx as f32 / n_objects as f32) * 100.0;
            print!("\r    Progress: {:5.2}% ", progress);
        }

        println!("\r    Done ({}s)        ", start.elapsed().as_secs());
    }

    Ok(())
}
