use crate::io::{download, read_geojson};
use crate::{Error, Result};
use geo_types::GeometryCollection;
use std::path::Path;
use tempfile::tempdir;

const URL: &str = "https://download.geofabrik.de/index-v1.json";

fn get_index_v1() -> Result<GeometryCollection<f64>> {
    // Use the same filename as the URL
    let filename = Path::new(URL)
        .file_name()
        .expect("No filename found in URL")
        .to_str()
        .unwrap();

    let tmp_dir = tempdir()?;
    let tmp_full_path = tmp_dir.path().join(filename);

    if cfg!(debug_assertions) {
        println!("Temporal file downloaded here: {:?}", tmp_full_path);
    }

    download(URL, &tmp_full_path)?;
    Ok(read_geojson(&tmp_full_path)?)
}

#[derive(Debug)]
pub struct Index_v1 {
    data: GeometryCollection<f64>,
}

impl Index_v1{
    pub fn new() -> Result<Self> {
        let index_v1 = get_index_v1()?;
        Ok(Self {
            data: index_v1,
        })
    }
}

// fn get_grandparents(data: serde_json::Value) -> Result<Vec<String>> {
//     let mut grandparents = Vec::new();
//     for country in data.as_object().unwrap().keys() {
//         for region in data[country].as_object().unwrap().keys() {
//             grandparents.push(format!("{}/{}", country, region));
//         }
//     }
//     Ok(grandparents)
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index_v1() {
        let index_v1 = get_index_v1().unwrap();
        assert_eq!(index_v1.is_empty(), false);
    }

    #[test]
    fn test_index_v1() {
        let index = Index_v1::new().unwrap();
        assert_eq!(index.data.is_empty(), false);
    }
}
