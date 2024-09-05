use crate::bbox::intersecting::OsmPbf;
use crate::Result;

use geojson::{FeatureCollection, GeoJson};
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::io::Write;

pub fn download(url: &str, file_path: &PathBuf) -> Result<()> {
    // Send an HTTP GET request to the URL
    let mut response = reqwest::blocking::get(url)?;

    let mut dest = File::create(file_path)?;

    if cfg!(debug_assertions) {
        println!("Downloading file: {:?}", file_path);
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

    // Get the current working directory
    let current_dir = std::env::current_dir()?;

    // Define the output file path
    let file_path = current_dir.join("what-osm-pbf.json");

    // Create and write to the file
    let mut file = File::create(&file_path)?;
    serde_json::to_writer_pretty(&file, &osmpbfs_json)?;

    // Optionally, flush the file to ensure it's written
    file.flush()?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_download() {
        let url = "https://download.geofabrik.de/index-v1.json";
        let tmp_dir = tempdir().unwrap();
        let tmp_full_path = tmp_dir.path().join("index-v1.json");
        download(url, &tmp_full_path).unwrap();
        assert_eq!(tmp_full_path.exists(), true);
    }

    #[test]
    fn test_read_geojson() {
        let url = "https://download.geofabrik.de/index-v1.json";
        let tmp_dir = tempdir().unwrap();
        let tmp_full_path = tmp_dir.path().join("index-v1.json");
        download(url, &tmp_full_path).unwrap();
        let geojson = read_geojson(&tmp_full_path).unwrap();
        assert_eq!(geojson.features.is_empty(), false);
    }

    #[test]
    fn test_to_json() {
        let osmpbfs = vec![
            OsmPbf {
                name: "Africa".to_string(),
                link: "https://download.geofabrik.de/africa-latest.osm.pbf".to_string(),
            },
            OsmPbf {
                name: "Antarctica".to_string(),
                link: "https://download.geofabrik.de/antarctica-latest.osm.pbf".to_string(),
            },
        ];
        to_json(&osmpbfs).unwrap();
    }
}
