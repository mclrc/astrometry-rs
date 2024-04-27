use std::path::Path;

use anyhow::Result;

use common::usnob::USNOBFile;
use diesel::PgConnection;

pub fn ingest_files(_db: &PgConnection, vec: &[impl AsRef<Path>]) -> Result<()> {
    for path in vec {
        let file = USNOBFile::open(path)?;
        let objects = file.iter();

        for object in objects {
            println!("{:?}", object);
        }
    }
    Ok(())
}
