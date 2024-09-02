use geojson::FeatureCollection;
use crate::index_v1::parents;

#[derive(Debug)]
pub struct GrandParent {
    pub name: String,
    id: String,
    pub parents: Option<Vec<parents::Parent>>,
    geom: Option<geojson::Geometry>,
}

pub fn get(data: &FeatureCollection) -> Option<Vec<GrandParent>> {
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
                parents: parents::get(data, id),
                geom: feature.geometry.clone(),
            });
        }
    });
    if grand_parents.is_empty() {
        return None;
    }
    Some(grand_parents)
}
