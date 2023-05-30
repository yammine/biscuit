mod biscuit;

pub use biscuit::*;
use magnus::Error;

#[magnus::init]
fn init() -> Result<(), Error> {
    biscuit::init()?;

    Ok(())
}
