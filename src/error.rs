use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Validate
    InvalidBbox,

    // -- Externals
    #[from]
    Io(std::io::Error),
    #[from]
    Reqwest(reqwest::Error),
    #[from]
    JSON(serde_json::Error),
    #[from]
    GeoJSON(geojson::Error),
}

// region: -- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: Error Boilerplate
