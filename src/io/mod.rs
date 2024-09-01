use crate::Result;
use geojson::{FeatureCollection, GeoJson};
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

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
