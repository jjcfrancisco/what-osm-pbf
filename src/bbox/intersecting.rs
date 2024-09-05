use crate::cli::Cli;
use crate::index_v1::{
    children::Child, grandchildren::GrandChild, grandparents::GrandParent, islands::Island,
    parents::Parent,
};
use geo::{self, Intersects};
use geo_types::Polygon;

pub trait GrandParentTrait {
    fn intersect(&self, other: &Polygon) -> bool;
}

impl GrandParentTrait for GrandParent {
    fn intersect(&self, other: &Polygon) -> bool {
        let geojson_geom = &self.geom;
        let geo_types_geom: geo_types::Geometry<f64> = geojson_geom
            .try_into()
            .expect("Failed to convert GeoJSON geometry to geo-types geometry");
        geo_types_geom.intersects(other)
    }
}

pub trait ParentTrait {
    fn intersect(&self, other: &Polygon) -> bool;
}

impl ParentTrait for Parent {
    fn intersect(&self, other: &Polygon) -> bool {
        let geojson_geom = &self.geom;
        let geo_types_geom: geo_types::Geometry<f64> = geojson_geom
            .try_into()
            .expect("Failed to convert GeoJSON geometry to geo-types geometry");
        geo_types_geom.intersects(other)
    }
}

pub trait ChildTrait {
    fn intersect(&self, other: &Polygon) -> bool;
}

impl ChildTrait for Child {
    fn intersect(&self, other: &Polygon) -> bool {
        let geojson_geom = &self.geom;
        let geo_types_geom: geo_types::Geometry<f64> = geojson_geom
            .try_into()
            .expect("Failed to convert GeoJSON geometry to geo-types geometry");
        geo_types_geom.intersects(other)
    }
}

pub trait GrandChildTrait {
    fn intersect(&self, other: &Polygon) -> bool;
}

impl GrandChildTrait for GrandChild {
    fn intersect(&self, other: &Polygon) -> bool {
        let geojson_geom = &self.geom;
        let geo_types_geom: geo_types::Geometry<f64> = geojson_geom
            .try_into()
            .expect("Failed to convert GeoJSON geometry to geo-types geometry");
        geo_types_geom.intersects(other)
    }
}

pub trait IslandTrait {
    fn intersect(&self, other: &Polygon) -> bool;
}

impl IslandTrait for Island {
    fn intersect(&self, other: &Polygon) -> bool {
        let geojson_geom = &self.geom;
        let geo_types_geom: geo_types::Geometry<f64> = geojson_geom
            .try_into()
            .expect("Failed to convert GeoJSON geometry to geo-types geometry");
        geo_types_geom.intersects(other)
    }
}

pub struct OsmPbf {
    pub name: String,
    pub link: String,
}

impl OsmPbf {
    pub fn new() -> Vec<OsmPbf> {
        Vec::new()
    }
}

pub trait OsmPbfTrait {
    fn add(&mut self, name: String, link: String);
}

impl OsmPbfTrait for Vec<OsmPbf> {
    fn add(&mut self, name: String, link: String) {
        self.push(OsmPbf { name, link });
    }
}

pub fn get_all(
    grand_parents: Option<Vec<GrandParent>>,
    islands: Option<Vec<Island>>,
    validated_bbox: &Polygon,
    args: &Cli,
) -> Vec<OsmPbf> {
    let mut osmpbfs = OsmPbf::new();
    let mut does_intersect: bool;
    for grand_parent in grand_parents.unwrap() {
        does_intersect = grand_parent.intersect(validated_bbox);

        ////// Grandparent
        if does_intersect && args.level == 3 {
            // Example: user wants level=3 only
            osmpbfs.add(grand_parent.name, grand_parent.link);
        } else if does_intersect && args.level < 3 && grand_parent.parents.is_some() {
            // Example: user wants level=2 or lower and there are parents
            for parent in grand_parent.parents.unwrap() {
                does_intersect = parent.intersect(validated_bbox);

                ////// Parent
                if does_intersect && args.level == 2 {
                    // Example: user wants level=2 only
                    osmpbfs.add(parent.name, parent.link);
                } else if does_intersect && args.level < 2 && parent.children.is_some() {
                    // Example: user wants level=1 or lower and there are children
                    for child in parent.children.unwrap() {
                        does_intersect = child.intersect(validated_bbox);

                        ////// Child
                        if does_intersect && args.level == 1 {
                            // Example: user wants level=1 only
                            osmpbfs.add(child.name, child.link);
                        } else if does_intersect && args.level < 1 && child.grandchildren.is_some()
                        {
                            // Example: user wants level=0 or lower and there are grandchildren
                            for grandchild in child.grandchildren.unwrap() {
                                ////// Grandchild
                                does_intersect = grandchild.intersect(validated_bbox);
                                if does_intersect && args.level == 0 {
                                    // Example: user wants level=0 only
                                    osmpbfs.add(grandchild.name, grandchild.link);
                                }
                            }
                        } else if does_intersect && args.level < 1 && child.grandchildren.is_none()
                        {
                            // Example: user wants level=0 but there are no grandchildren
                            osmpbfs.add(child.name, child.link);
                        }
                    }
                } else if does_intersect && args.level < 2 && parent.children.is_none() {
                    // Example: user wants level=1 or lower but there are no children
                    osmpbfs.add(parent.name, parent.link);
                }
            }
        } else if does_intersect && args.level < 3 && grand_parent.parents.is_none() {
            // Example: user wants level=2 or lower but there are no parents
            osmpbfs.add(grand_parent.name, grand_parent.link);
        }
    }
    // Islands
    for island in islands.unwrap() {
        does_intersect = island.intersect(validated_bbox);
        if does_intersect {
            osmpbfs.add(island.name, island.link);
        }
    }
    osmpbfs
}
