use crate::{Result, Error};

pub fn bbox(bbox: &str) -> Result<()> {
    let bbox: Vec<&str> = bbox.split(',').collect();
    if bbox.len() != 4 {
        return Err(Error::InvalidBbox);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox() {
        assert!(bbox("1,2,3,4").is_ok());
        assert!(bbox("1,2,3").is_err());
    }
}
