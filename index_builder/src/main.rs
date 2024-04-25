use std::error::Error;

use clap::Parser;
use serde::Deserialize;

use common::fits::FitsTable;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct CatalogObject {
    #[serde(rename = "USNOB_ID")]
    usnob_id: i32,
    #[serde(rename = "RA")]
    ra: f64,
    #[serde(rename = "DEC")]
    dec: f64,
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
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file } = Args::parse();

    let table = FitsTable::open(&file, 1)?;

    println!("{}", table.len());

    for row in table.iter::<CatalogObject>() {
        match row {
            Ok(row) => println!("RA {}\t\tDEC {}", row.ra, row.dec),
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
