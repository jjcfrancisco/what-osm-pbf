use crate::index_v1::find_osm_pbf_link;
use geojson::FeatureCollection;

#[derive(Debug)]
pub struct Island {
    pub name: String,
    id: String,
    pub link: String,
    pub geom: geojson::Geometry,
}

const ISLAND_LIST: [&str; 5] = [
    "canary-islands",
    "azores",
    "comores",
    "isle-of-man",
    "guernsey-jersey",
];

pub fn get(data: &FeatureCollection) -> Option<Vec<Island>> {
    let mut islands: Vec<Island> = Vec::new();
    let empty_urls = serde_json::Map::new();
    data.features.iter().for_each(|feature| {
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
        if ISLAND_LIST.contains(&id) {
            islands.push(Island {
                name: name.to_string(),
                id: id.to_string(),
                link: link.expect("No link found"),
                geom: feature.geometry.clone().expect("No geometry found"),
            });
        }
    });
    if islands.is_empty() {
        return None;
    }
    Some(islands)
}
