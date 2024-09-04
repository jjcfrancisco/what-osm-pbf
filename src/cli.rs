use crate::bbox;
use crate::index_v1::IndexV1;
use crate::validate;
use crate::Result;

use clap::{ArgAction, Parser};

/// Get the necessary osm pbf files within a bounding box
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Bounding box in the format: min_lon,min_lat,max_lon,max_lat
    #[arg(short, long)]
    pub bbox: String,

    #[arg(short, long, default_value = "3")]
    pub level: usize,

    /// Download the resulting osm.pbf files. Optional.
    #[arg(short, long, action(ArgAction::SetTrue))]
    pub download: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    let validated_bbox = validate::bbox(&args.bbox)?;
    validate::level(&args.level)?;

    let index = IndexV1::new()?;
    let grand_parents = index.get_grandparents();
    #[allow(unused_variables)]
    let islands = index.get_islands();
    let osmpbfs = bbox::get_intersecting(grand_parents, islands, &validated_bbox, &args);
    for osmpbf in osmpbfs {
        println!("Name: {:?}, Link: {:?}", osmpbf.name, osmpbf.link);
    }

    Ok(())
}
