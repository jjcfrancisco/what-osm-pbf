use crate::io::{download, read_geojson};
use crate::{Error, Result};
use geojson::FeatureCollection;
use serde_json::Map;
use std::path::Path;
use tempfile::tempdir;

const URL: &str = "https://download.geofabrik.de/index-v1.json";

// type Index struct {
// 	GrandParents *[]GrandParent
// 	Islands      *[]Island
// }

#[derive(Debug)]
pub struct IndexV1 {
    geojson: FeatureCollection,
    pub grand_parents: Option<Vec<GrandParent>>,
    pub islands: Option<Vec<Island>>,
}

#[derive(Debug)]
pub struct GrandParent {
    name: String,
    id: String,
    parents: Option<Vec<Parent>>,
    geom: Option<geojson::Geometry>,
}

#[derive(Debug)]
pub struct Parent {
    name: String,
    id: String,
    iso31661alpha2: Option<String>,
    iso31662: Option<String>,
    link: Option<String>,
    children: Option<Vec<Child>>,
    geom: Option<geojson::Geometry>,
}

#[derive(Debug)]
pub struct Child {
    name: String,
    id: String,
    iso31661alpha2: Option<String>,
    iso31662: Option<String>,
    link: String,
    // grand_children: Option<Vec<GrandChild>>,
    geom: Option<geojson::Geometry>,
}

#[derive(Debug)]
pub struct Island {
    name: String,
    id: String,
    link: String,
    geom: Option<geojson::Geometry>,
}

// type GrandChild struct {
// 	Name           string
// 	Id             string
// 	Iso31661alpha2 string
// 	Iso31662       string
// 	Link           string
// 	Geom           orb.Geometry
// }

fn get_index_v1() -> Result<FeatureCollection> {
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
    pub fn get_grandparents(&self) -> Result<Option<Vec<GrandParent>>> {
        let grand_parents = get_grandparents(&self.geojson);
        if grand_parents.is_none() {
            return Err(Error::NoGrandParents);
        }
        Ok(grand_parents)
    }
}

fn get_grandparents(data: &FeatureCollection) -> Option<Vec<GrandParent>> {
    let mut grand_parents: Vec<GrandParent> = Vec::new();
    data.features.iter().for_each(|feature| {
        // Get name or ignore
        let name = feature
            .property("name")
            .unwrap_or(&serde_json::Value::Null)
            .as_str()
            .unwrap_or("No name found");
        let id = feature
            .property("id")
            .unwrap_or(&serde_json::Value::Null)
            .as_str()
            .unwrap_or("No id found");
        if feature
            .property("parent")
            .unwrap_or(&serde_json::Value::Null)
            .is_null()
        {
            grand_parents.push(GrandParent {
                name: name.to_string(),
                id: id.to_string(),
                parents: get_parents(data, id),
                geom: feature.geometry.clone(),
            });
        }
    });
    if grand_parents.is_empty() {
        return None;
    }
    Some(grand_parents)
}

pub fn get_parents(data: &FeatureCollection, grand_parent_id: &str) -> Option<Vec<Parent>> {
    let parents: Vec<Parent> = Vec::new();
    let empty_urls = serde_json::Map::new();
    let empty_array: Vec<serde_json::Value> = Vec::new();
    data.features.iter().for_each(|feature| {
        // If 'parent' is not null
        if !feature
            .property("parent")
            .unwrap_or(&serde_json::Value::Null)
            .is_null()
        {
            let parent_id = feature
                .property("parent")
                .unwrap_or(&serde_json::Value::Null)
                .as_str()
                .unwrap_or("No parent id found");
            let name = feature
                .property("name")
                .unwrap_or(&serde_json::Value::Null)
                .as_str()
                .unwrap_or("No name found");
            let id = feature
                .property("id")
                .unwrap_or(&serde_json::Value::Null)
                .as_str()
                .unwrap_or("No id found");
            let urls = feature
                .property("urls")
                .unwrap_or(&serde_json::Value::Null)
                .as_object()
                .unwrap_or(&empty_urls);
            let link = find_osm_pbf_link(urls);
            let iso31661alpha2 = feature
                .property("iso3166-1:alpha2")
                .unwrap_or(&serde_json::Value::Null)
                .as_array()
                .unwrap_or(&empty_array)
                .first()
                .unwrap_or(&serde_json::Value::Null)
                .as_str();
            if iso31661alpha2.is_some() && parent_id == grand_parent_id {
                let mut parent = Parent {
                    name: name.to_string(),
                    id: id.to_string(),
                    iso31661alpha2: None,
                    iso31662: None,
                    link: link.clone(),
                    children: None,
                    geom: feature.geometry.clone(),
                };
                if let Some(iso31661alpha2) = iso31661alpha2 {
                    parent.iso31661alpha2 = Some(iso31661alpha2.to_string());
                }
                parent.children = get_children();
            }
            // Russian territories are processed differently
            if parent_id == "russia" && grand_parent_id == "russia" {
                let mut parent = Parent {
                    name: name.to_string(),
                    id: id.to_string(),
                    iso31661alpha2: None,
                    iso31662: None,
                    link,
                    children: None,
                    geom: feature.geometry.clone(),
                };
                if iso31661alpha2.is_some() {
                    parent.iso31661alpha2 = Some(iso31661alpha2.unwrap().to_string())
                };
                // Russia does not have children territories but
                // here for consistency and future proofing
                parent.children = get_children();
            }
        }
    });
    Some(parents)
}

fn get_children() -> Option<Vec<Child>> {
    None
}

fn find_osm_pbf_link(urls: &Map<String, serde_json::Value>) -> Option<String> {
    for (key, value) in urls {
        if key.contains("pbf") {
            return Some(value.as_str().unwrap_or("No link found").to_string())
        }
    }
    None
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

    #[test]
    fn test_get_grandparents() {
        let index = IndexV1::new().unwrap();
        let grand_parents = get_grandparents(&index.geojson);
        assert_eq!(grand_parents.is_none(), false);
    }

    #[test]
    fn test_get_parents() {
        let index = IndexV1::new().unwrap();
        let grand_parents = get_grandparents(&index.geojson);
        let parents = get_parents(&index.geojson, &grand_parents.unwrap()[0].id);
        assert_eq!(parents.is_none(), false);
    }

    #[test]
    fn test_get_children() {
        let children = get_children();
        assert_eq!(children.is_none(), true);
    }
}
