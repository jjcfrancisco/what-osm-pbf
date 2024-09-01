use crate::Result;

use crate::validate;
use clap::{Parser, ArgAction};
use crate::index_v1::Index_v1;

/// Get the necessary osm pbf files within a bounding box
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Bounding box in the format: min_lon,min_lat,max_lon,max_lat
    #[arg(short, long)]
    pub bbox: String,

    /// Download the resulting osm.pbf files. Optional.
    #[arg(
        short,
        long,
        action(ArgAction::SetTrue),
    )]
    pub download: Option<String>,
}

pub fn run() -> Result<()> {
    let mut args = Cli::parse();
    validate::bbox(&mut args.bbox)?;
    if args.download.is_some() {
        let index = Index_v1::new()?;
        println!("{:?}", index);
    }

    Ok(())
}
