mod error;
mod cli;
mod io;
mod validate;
mod index_v1;
mod bbox;

pub use self::error::{Error, Result};

fn main() -> Result<()> {
    cli::run()?;
    Ok(())
}
