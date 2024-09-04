use crate::index_v1::find_osm_pbf_link;
use geojson::FeatureCollection;
use crate::index_v1::grandchildren;

#[derive(Debug)]
pub struct Child {
    pub name: String,
    id: String,
    iso31661alpha2: Option<String>,
    iso31662: Option<String>,
    pub link: String,
    pub grandchildren: Option<Vec<grandchildren::GrandChild>>,
    pub geom: geojson::Geometry,
}

pub fn get(data: &FeatureCollection, parent_id: &str) -> Option<Vec<Child>> {
    let mut children: Vec<Child> = Vec::new();
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
        if feature
            .property("parent")
            .unwrap_or(&serde_json::Value::Null)
            == parent_id
        {
            let mut child = Child {
                name: name.to_string(),
                id: id.to_string(),
                iso31661alpha2: None,
                iso31662: None,
                link: link.expect("No link found"),
                grandchildren: None,
                geom: feature.geometry.clone().expect("No geometry found"),
            };
            child.grandchildren = grandchildren::get(data, &child.id);
            children.push(child);
        }
    });
    if children.is_empty() {
        return None;
    }
    Some(children)
}

