#[derive(Debug)]
pub struct Island {
    name: String,
    id: String,
    link: String,
    geom: Option<geojson::Geometry>,
}


