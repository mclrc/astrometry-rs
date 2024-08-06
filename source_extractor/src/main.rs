use anyhow::Result;
use clap::Parser;
use source_extractor::{draw_objects, extract_sources};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let objects = extract_sources(&args.input)?;

    let mut stdout = std::io::stdout();

    for object in objects.iter() {
        object.write_as_string(&mut stdout)?;
    }

    if let Some(output) = args.output {
        let img = draw_objects(&args.input, &objects)?;
        img.save(output)?;
    }

    Ok(())
}
