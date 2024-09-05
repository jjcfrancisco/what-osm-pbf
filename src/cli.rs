use crate::bbox;
use crate::index_v1::IndexV1;
use crate::io::to_json;
use crate::validate;
use crate::Result;

use clap::Parser;

/// Get the necessary osm pbf files within a bounding box
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Bounding box in the format: min_lon,min_lat,max_lon,max_lat
    #[arg(short, long)]
    pub bbox: String,

    /// Level of detail. Optional. Default is 0 (the smallest level).
    #[arg(short, long, default_value = "3")]
    pub level: usize,

    /// Save output to JSON. Optional.
    #[arg(short, long, default_value = "false")]
    pub savejson: String,

    /// Download the resulting osm.pbf files. Optional.
    #[arg(short, long, default_value = "false")]
    pub download: String,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    let validated_bbox = validate::bbox(&args.bbox)?;
    validate::level(&args.level)?;

    let index = IndexV1::new()?;
    let grand_parents = index.get_grandparents();
    let islands = index.get_islands();
    let osmpbfs = bbox::intersecting::get_all(grand_parents, islands, &validated_bbox, &args);

    if osmpbfs.is_empty() {
        println!("\n❌ No osm.pbf files found that intersect\n");
        return Ok(());
    } else {
        println!(
            "\n✅ Found {} osm.pbf files that intersect\n",
            osmpbfs.len()
        );
        for osmpbf in &osmpbfs {
            println!("Name: {:?}, Link: {:?}", osmpbf.name, osmpbf.link);
        }
        println!();
        if args.download == "true" {
            for osmpbf in &osmpbfs {
                println!("Downloading: {:?}", osmpbf.link);
                let file_name = osmpbf.link.split('/').last().unwrap();
                let file_path = std::env::current_dir()?.join(&file_name);
                crate::io::download(&osmpbf.link, &file_path)?;
                println!("✓ Downloaded: {:?}\n", file_path);
            }
        } else if args.savejson == "true" {
            to_json(&osmpbfs)?;
            println!("🎉 Results saved to what-osm-pbf.json\n");
        }
    }

    Ok(())
}
