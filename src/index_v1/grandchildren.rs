use crate::index_v1::find_osm_pbf_link;
use geojson::FeatureCollection;

#[derive(Debug)]
pub struct GrandChild {
    name: String,
    id: String,
    iso31661alpha2: Option<String>,
    iso31662: Option<String>,
    link: Option<String>,
    geom: Option<geojson::Geometry>,
}

pub fn get(data: &FeatureCollection, child_id: &str) -> Option<Vec<GrandChild>> {
    let mut grand_children: Vec<GrandChild> = Vec::new();
    let empty_urls = serde_json::Map::new();
    data.features.iter().for_each(|feature| {
        if feature
            .property("parent")
            .unwrap_or(&serde_json::Value::Null)
            == child_id
        {
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
            grand_children.push(GrandChild {
                name: name.to_string(),
                id: id.to_string(),
                iso31661alpha2: None,
                iso31662: None,
                link,
                geom: feature.geometry.clone(),
            });
        }
    });
    None
}
