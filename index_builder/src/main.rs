use std::error::Error;

use clap::Parser;
use serde::{Deserialize, Serialize};

use common::usnob::USNOBFile;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
struct CatalogObject {
    #[serde(rename = "USNOB_ID")]
    usnob_id: u32,
    #[serde(rename = "RA")]
    ra: f64,
    #[serde(rename = "SIGMA_RA")]
    sigma_ra: f32,
    #[serde(rename = "SIGMA_RA_FIT")]
    sigma_ra_fit: f32,
    #[serde(rename = "PM_RA")]
    pm_ra: f32,
    #[serde(rename = "DEC")]
    dec: f64,
    #[serde(rename = "SIGMA_DEC")]
    sigma_dec: f32,
    #[serde(rename = "SIGMA_DEC_FIT")]
    sigma_dec_fit: f32,
    #[serde(rename = "PM_DEC")]
    pm_dec: f32,
    #[serde(rename = "MAGNITUDE_0")]
    mag0: f32,
    #[serde(rename = "MAGNITUDE_1")]
    mag1: f32,
    #[serde(rename = "MAGNITUDE_2")]
    mag2: f32,
    #[serde(rename = "MAGNITUDE_3")]
    mag3: f32,
    #[serde(rename = "MAGNITUDE_4")]
    mag4: f32,
    #[serde(rename = "EPOCH")]
    epoch: f32,
    #[serde(rename = "NUM_DETECTIONS")]
    num_detections: i32,
    #[serde(rename = "FLAGS")]
    flags: u8,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file } = Args::parse();

    let file = USNOBFile::open(&file)?;

    for object in file.iter() {
        println!("{}", serde_json::to_string_pretty(&object)?);
    }

    Ok(())
}
