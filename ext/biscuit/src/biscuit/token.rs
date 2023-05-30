use super::{builder, key_pair, root};
use builder::BuilderRb;
use key_pair::PublicKeyRb;

use biscuit_auth::Biscuit;
use magnus::{class, function, method, prelude::*, DataTypeFunctions, Error, TypedData};

#[derive(TypedData)]
#[magnus(class = "Biscuit::Token", free_immediately, size)]
pub struct TokenRb {
    pub token: Biscuit,
}

impl DataTypeFunctions for TokenRb {}

impl TokenRb {
    fn builder() -> builder::BuilderRb {
        BuilderRb::new()
    }

    fn from_base64(s: String, kp: &PublicKeyRb) -> Result<Self, Error> {
        let token = Biscuit::from_base64(s.as_bytes(), kp.key).unwrap();
        Ok(TokenRb { token })
    }

    fn to_base64(&self) -> Result<String, Error> {
        Ok(self.token.to_base64().unwrap())
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Token", class::object())?;
    class.define_singleton_method("builder", function!(TokenRb::builder, 0))?;
    class.define_singleton_method("from_base64", function!(TokenRb::from_base64, 2))?;

    class.define_method("to_base64", method!(TokenRb::to_base64, 0))?;

    Ok(())
}
