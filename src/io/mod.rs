use crate::Result;
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use reqwest;
use std::fs::File;
use std::io::copy;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn download(url: &str, file_path: &PathBuf) -> Result<()> {
    // Send an HTTP GET request to the URL
    let mut response = reqwest::blocking::get(url)?;

    let mut dest = File::create(&file_path)?;

    if cfg!(debug_assertions) {
        println!("Downloading index-v1.json");
    }

    // Copy the contents of the response to the file
    copy(&mut response, &mut dest)?;

    Ok(())
}

pub fn read_geojson(file_path: &PathBuf) -> Result<GeometryCollection<f64>> {
    let mut file = File::open(&file_path)?;
    let mut file_contents = String::new();
    let _ = file.read_to_string(&mut file_contents);

    let data = file_contents.parse::<GeoJson>()?;
    let geometry_collection = quick_collection(&data)?;

    Ok(geometry_collection)
}
