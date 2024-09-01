use crate::Result;

use crate::index_v1::IndexV1;
use crate::validate;
use clap::{ArgAction, Parser};

/// Get the necessary osm pbf files within a bounding box
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Bounding box in the format: min_lon,min_lat,max_lon,max_lat
    #[arg(short, long)]
    pub bbox: String,

    /// Download the resulting osm.pbf files. Optional.
    #[arg(short, long, action(ArgAction::SetTrue))]
    pub download: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    validate::bbox(&args.bbox)?;
    if args.download.is_some() {
        let index = IndexV1::new()?;
        let grand_parents = index.get_grandparents()?;
    }

    Ok(())
}
