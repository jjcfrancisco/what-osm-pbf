use crate::{Error, Result};
use geo_types::{coord, Polygon, Rect};

pub fn bbox(bbox: &str) -> Result<Polygon> {
    let bbox_str: Vec<&str> = bbox.split(',').collect();
    if bbox_str.len() != 4 {
        return Err(Error::InvalidBbox);
    }

    let bbox_nums: Vec<f64> = bbox_str
        .iter()
        .map(|x| x.parse().expect("Invalid bbox values"))
        .collect();

    // Transform bbox to Polygon
    let rect = Rect::new(
        coord! { x: bbox_nums[0], y: bbox_nums[1]},
        coord! { x: bbox_nums[2], y: bbox_nums[3]},
    );

    let bbox = rect.to_polygon();

    Ok(bbox)
}

pub fn level(level: &usize) -> Result<usize> {
    // must be between 0 and 3
    if *level > 3 {
        return Err(Error::InvalidLevel);
    } else {
        Ok(*level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox() {
        let bbox = bbox("-180,-90,180,90").unwrap();
        assert_eq!(bbox.exterior()[0], coord! { x: -180.0, y: -90.0 });
        assert_eq!(bbox.exterior()[2], coord! { x: 180.0, y: 90.0 });
    }

    #[test]
    fn test_level() {
        let level = level(&3).unwrap();
        assert_eq!(level, 3);
    }
}
