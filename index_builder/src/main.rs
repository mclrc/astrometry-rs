use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;

use clap::Parser;
use serde::{Deserialize, Serialize};
use vizier_adql::Client;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    order: u32,
    #[clap(long = "sphpx")]
    stars_per_healpix: u32,
    #[clap(short, long)]
    output: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CatalogueObject {
    #[serde(rename = "USNO-B1_0")]
    designation: String,
    #[serde(rename = "RAJ2000")]
    raj2000: f64,
    #[serde(rename = "DEJ2000")]
    dej2000: f64,
    #[serde(rename = "R1mag")]
    rmag: f64,
    #[serde(rename = "B1mag")]
    bmag: f64,
    #[serde(rename = "Imag")]
    imag: f64,
}

const CATALOGUE_URL: &str = "I/284/out";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = Arc::new(Client::default());

    let nside: u64 = u32::pow(2, args.order).into();
    let n_healpix = 12 * nside * nside;

    println!("Querying catalogue for {} HEALPix cells", n_healpix);

    let mut healpix_map = HashMap::new();

    let requests = (0..n_healpix)
        .map(|healpix_idx| {
            let client = Arc::clone(&client);

            tokio::spawn(async move {
                let query = format!(
                    "SELECT TOP {} *
                    FROM \"{}\"
                    WHERE IVO_HEALPIX_INDEX({}, RAJ2000, DEJ2000) = {}
                    ORDER BY B1mag ASC",
                    args.stars_per_healpix, CATALOGUE_URL, args.order, healpix_idx
                );

                let response = client
                    .query::<CatalogueObject>(&query)
                    .await
                    .map_err(|e| eprintln!("Error querying catalogue: {:?}", e));

                println!(
                    "[{}] HEALPix {}",
                    if response.is_ok() { "OK" } else { "ERR" },
                    healpix_idx,
                );

                (healpix_idx, response)
            })
        })
        .collect::<Vec<_>>();

    let results = join_all(requests).await;

    for result in results {
        match result {
            Ok((healpix_idx, Ok(stars))) => {
                healpix_map.insert(healpix_idx, stars);
            }
            Ok((_, Err(e))) => println!("Error: {:?}", e),
            Err(e) => println!("Task failed: {:?}", e),
        }
    }

    // Write the output to a file
    let output = std::fs::File::create(args.output).unwrap();

    serde_json::to_writer_pretty(output, &healpix_map).unwrap();
}
