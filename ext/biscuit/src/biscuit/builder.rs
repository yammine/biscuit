use super::root;
use magnus::{class, function, prelude::*, DataTypeFunctions, Error, TypedData};

#[derive(TypedData)]
#[magnus(class = "Biscuit::Builder", free_immediately, size)]
pub struct BuilderRb;

impl DataTypeFunctions for BuilderRb {}

impl BuilderRb {
    pub fn new() -> Self {
        BuilderRb
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Builder", class::object())?;
    class.define_singleton_method("new", function!(BuilderRb::new, 0))?;

    Ok(())
}
