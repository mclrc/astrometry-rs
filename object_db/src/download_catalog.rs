use std::{env, path::Path, process::Stdio};

use anyhow::Result;
use common::error::AstroError;
use futures::future;
use tokio::process::Command;

const CATALOG_TORRENT_URL: &str = "http://www.ap-i.net/pub/skychart/usnob/usno-b1.0.torrent";

pub async fn download_catalog(destination: impl AsRef<Path>) -> Result<()> {
    let destination = destination.as_ref();

    let temp_dir = env::temp_dir();

    let torrent_path = temp_dir.join("usnob1.torrent");
    if !torrent_path.exists() {
        print!("Downloading catalog torrent... ");
        let response = reqwest::get(CATALOG_TORRENT_URL).await?;
        let bytes = response.bytes().await?;
        tokio::fs::write(&torrent_path, bytes).await?;
        println!("done");
    }

    if !destination.exists() {
        tokio::fs::create_dir_all(destination).await?;
    }

    which::which("aria2c")
        .map_err(|_| AstroError::new("Could not find aria2 in PATH. Make sure it is installed."))?;

    println!("Downloading catalog... ");
    let mut child = Command::new("aria2c")
        .arg("--summary-interval=10")
        .arg("--seed-time=0")
        .arg(format!("--dir={}", destination.display()))
        .arg(torrent_path)
        .stdout(Stdio::inherit())
        .spawn()?;

    let status = child.wait().await?;

    if !status.success() {
        Err(AstroError::new("Failed to download catalog"))?;
    }

    println!("Done");

    let mut unzip_tasks = Vec::with_capacity(1800);

    let datadir = destination.join("usno-b1.0");
    let mut dir = tokio::fs::read_dir(&datadir).await?;

    while let Some(file) = dir.next_entry().await? {
        let path = file.path();
        let ext = path.extension().unwrap_or_default();
        let filename = path.file_name().unwrap();

        if ext != "zip" {
            continue;
        }

        let destination = destination.to_path_buf();

        let task = tokio::spawn(async move {
            Command::new("unzip")
                .arg(&path)
                .arg("*.cat")
                .arg("-d")
                .arg(destination)
                .stdout(Stdio::inherit())
                .spawn()?
                .wait()
                .await?;

            tokio::fs::remove_file(path).await?;

            Ok::<(), anyhow::Error>(())
        });

        unzip_tasks.push(task);
    }

    future::join_all(unzip_tasks).await;

    Ok(())
}
