use crate::bbox::intersecting::OsmPbf;
use crate::Result;
use geojson::{FeatureCollection, GeoJson};
use reqwest;
use std::env::current_dir;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use serde_json;
use std::io::Write;

pub fn download(url: &str, file_path: &PathBuf) -> Result<()> {
    // Send an HTTP GET request to the URL
    let mut response = reqwest::blocking::get(url)?;

    let mut dest = File::create(file_path)?;

    if cfg!(debug_assertions) {
        println!("Downloading index-v1.json");
    }

    // Copy the contents of the response to the file
    copy(&mut response, &mut dest)?;

    Ok(())
}

pub fn read_geojson(file_path: &PathBuf) -> Result<FeatureCollection> {
    let geojson_str = std::fs::read_to_string(file_path)?;
    let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
    let feature_collection = FeatureCollection::try_from(geojson).unwrap();

    Ok(feature_collection)
}

pub fn to_json(osmpbfs: &Vec<OsmPbf>) -> Result<()> {
    let mut osmpbfs_json = Vec::new();

    // Convert each OsmPbf into JSON and push to osmpbfs_json
    for osmpbf in osmpbfs {
        let osmpbf_json = serde_json::json!({
            "name": osmpbf.name,
            "link": osmpbf.link,
        });
        osmpbfs_json.push(osmpbf_json);
    }

    let osmpbfs_json = serde_json::json!(osmpbfs_json);

    if cfg!(debug_assertions) {
        println!("{}", serde_json::to_string_pretty(&osmpbfs_json)?);
    }

    // Get the current working directory
    let current_dir = std::env::current_dir()?;

    // Define the output file path
    let file_path = current_dir.join("osmpbfs.json");

    // Create and write to the file
    let mut file = File::create(&file_path)?;
    serde_json::to_writer_pretty(&file, &osmpbfs_json)?;

    // Optionally, flush the file to ensure it's written
    file.flush()?;

    println!("File saved to: {:?}", file_path);

    Ok(())
}
