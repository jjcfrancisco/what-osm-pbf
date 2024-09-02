use geojson::FeatureCollection;
use crate::index_v1::find_osm_pbf_link;
use crate::index_v1::children;

#[derive(Debug)]
pub struct Parent {
    pub name: String,
    id: String,
    iso31661alpha2: Option<String>,
    iso31662: Option<String>,
    link: Option<String>,
    pub children: Option<Vec<children::Child>>,
    geom: Option<geojson::Geometry>,
}
pub fn get(data: &FeatureCollection, grand_parent_id: &str) -> Option<Vec<Parent>> {
    let mut parents: Vec<Parent> = Vec::new();
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
                parent.children = children::get(&data, id);
                parents.push(parent);
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
                parent.children = children::get(&data, id);
                parents.push(parent);
            }
        }
    });
    if parents.is_empty() {
        return None;
    }
    Some(parents)
}

