use crate::index_v1::{parents, find_osm_pbf_link};
use geojson::FeatureCollection;

#[derive(Debug)]
pub struct GrandParent {
    pub name: String,
    id: String,
    pub link: String,
    pub parents: Option<Vec<parents::Parent>>,
    pub geom: geojson::Geometry,
}

pub fn get(data: &FeatureCollection) -> Option<Vec<GrandParent>> {
    let mut grand_parents: Vec<GrandParent> = Vec::new();
    let empty_urls = serde_json::Map::new();
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
        let urls = feature
            .property("urls")
            .unwrap_or(&serde_json::Value::Null)
            .as_object()
            .unwrap_or(&empty_urls);
        let link = find_osm_pbf_link(urls);
        if feature
            .property("parent")
            .unwrap_or(&serde_json::Value::Null)
            .is_null()
        {
            grand_parents.push(GrandParent {
                name: name.to_string(),
                id: id.to_string(),
                link: link.expect("No link found"),
                parents: parents::get(data, id),
                geom: feature.geometry.clone().expect("No geometry found"),
            });
        }
    });
    if grand_parents.is_empty() {
        return None;
    }
    Some(grand_parents)
}
