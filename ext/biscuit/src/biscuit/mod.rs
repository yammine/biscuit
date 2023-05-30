use magnus::{define_module, memoize, Error, RModule};

mod builder;
mod key_pair;
mod token;

pub use key_pair::KeyPairRb;

/// The `Biscuit` module.
pub fn root() -> RModule {
    *memoize!(RModule: define_module("Biscuit").unwrap())
}

pub struct Biscuit;
impl Biscuit {
    // Junk right now, ignore this
}

pub fn init() -> Result<(), Error> {
    let _biscuit = root();

    key_pair::init()?;
    builder::init()?;
    token::init()?;

    Ok(())
}
