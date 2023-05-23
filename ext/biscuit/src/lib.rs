use biscuit_auth::{KeyPair, PrivateKey};
use magnus::{define_module, function, method, prelude::*, DataTypeFunctions, Error, TypedData};

#[derive(TypedData)]
#[magnus(class = "Biscuit::KeyPair", free_immediately, size)]
struct KeyPairWrapper {
    key_pair: KeyPair,
}

impl DataTypeFunctions for KeyPairWrapper {}

impl KeyPairWrapper {
    fn new() -> Self {
        KeyPairWrapper {
            key_pair: KeyPair::new(),
        }
    }

    fn from(private: String) -> Result<Self, Error> {
        let private_key = PrivateKey::from_bytes_hex(&private).unwrap();
        let key_pair = KeyPair::from(&private_key);
        Ok(KeyPairWrapper { key_pair })
    }

    fn public(&self) -> Result<String, Error> {
        Ok(self.key_pair.public().to_bytes_hex())
    }

    fn private(&self) -> Result<String, Error> {
        Ok(self.key_pair.private().to_bytes_hex())
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Biscuit")?;
    let keypair_class = module.define_class("KeyPair", Default::default())?;
    keypair_class.define_singleton_method("new", function!(KeyPairWrapper::new, 0))?;
    keypair_class.define_singleton_method("from", function!(KeyPairWrapper::from, 1))?;
    keypair_class.define_method("public", method!(KeyPairWrapper::public, 0))?;
    keypair_class.define_method("private", method!(KeyPairWrapper::private, 0))?;
    Ok(())
}
