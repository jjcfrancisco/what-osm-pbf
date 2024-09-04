use crate::io::{download, read_geojson};
use crate::{Error, Result};
use geojson::FeatureCollection;
use serde_json::Map;
use std::path::Path;
use tempfile::tempdir;

pub mod children;
pub mod grandchildren;
pub mod grandparents;
pub mod islands;
pub mod parents;

use grandparents::GrandParent;
use islands::Island;

const URL: &str = "https://download.geofabrik.de/index-v1.json";

#[derive(Debug)]
pub struct IndexV1 {
    geojson: FeatureCollection,
    pub grand_parents: Option<Vec<GrandParent>>,
    pub islands: Option<Vec<Island>>,
}

fn get_index_v1() -> Result<FeatureCollection> {
    // Use the same filename as the URL
    let filename = Path::new(URL)
        .file_name()
        .expect("No filename found in URL")
        .to_str()
        .unwrap();

    let tmp_dir =tempdir()?;
    let tmp_full_path = tmp_dir.path().join(filename);

    if cfg!(debug_assertions) {
        println!("Temporal file downloaded here: {:?}", tmp_full_path);
    }

    download(URL, &tmp_full_path)?;
    read_geojson(&tmp_full_path)
}

impl IndexV1 {
    pub fn new() -> Result<Self> {
        let index_v1 = get_index_v1()?;
        Ok(Self {
            geojson: index_v1,
            grand_parents: None,
            islands: None,
        })
    }
    pub fn get_grandparents(&self) -> Option<Vec<GrandParent>> {
        let grand_parents = grandparents::get(&self.geojson);
        if grand_parents.is_none() {
            return None
        }
        grand_parents
    }
    pub fn get_islands(&self) -> Option<Vec<Island>> {
        let islands = islands::get(&self.geojson);
        if islands.is_none() {
            return None
        }
        islands
    }
}

fn find_osm_pbf_link(urls: &Map<String, serde_json::Value>) -> Option<String> {
    for (key, value) in urls {
        if key.contains("pbf") {
            return Some(value.as_str().unwrap_or("No link found").to_string());
        }
    }
    None
}

// Only for debugging purposes
fn display_list(grand_parents: Vec<GrandParent>, islands: Vec<Island>) {
    for grand_parent in grand_parents {
        println!("Grandparent: {}", grand_parent.name);
        if grand_parent.parents.is_some() {
            for parent in grand_parent.parents.unwrap() {
                println!("Parent: {}", parent.name);
                if parent.children.is_some() {
                    for child in parent.children.unwrap() {
                        println!("Child: {}", child.name);
                        if child.grandchildren.is_some() {
                            for grandchild in child.grandchildren.unwrap() {
                                println!("Grandchild: {}", grandchild.name);
                            }
                        }
                    }
                }
            }
        }
    }
    for island in islands {
        println!("Island: {}", island.name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index_v1() {
        let index_v1 = get_index_v1().unwrap();
        assert_eq!(index_v1.features.is_empty(), false);
    }

    #[test]
    fn test_index_v1() {
        let index = IndexV1::new().unwrap();
        assert_eq!(index.geojson.features.is_empty(), false);
    }
}
