use super::root;

use biscuit_auth::{KeyPair, PrivateKey, PublicKey};
use magnus::{class, function, method, prelude::*, DataTypeFunctions, Error, TypedData};

#[derive(TypedData)]
#[magnus(class = "Biscuit::KeyPair", free_immediately, size)]
pub struct KeyPairRb {
    pub key_pair: KeyPair,
}

impl DataTypeFunctions for KeyPairRb {}

#[derive(TypedData)]
#[magnus(class = "Biscuit::PublicKey", free_immediately, size)]
pub struct PublicKeyRb {
    pub key: biscuit_auth::PublicKey,
}

impl DataTypeFunctions for PublicKeyRb {}

impl PublicKeyRb {
    fn to_hex(&self) -> Result<String, Error> {
        Ok(self.key.to_bytes_hex())
    }

    fn from_hex(s: String) -> Result<Self, Error> {
        let key = PublicKey::from_bytes_hex(&s).unwrap();
        Ok(PublicKeyRb { key })
    }
}

#[derive(TypedData)]
#[magnus(class = "Biscuit::PrivateKey", free_immediately, size)]
pub struct PrivateKeyRb {
    pub key: biscuit_auth::PrivateKey,
}

impl DataTypeFunctions for PrivateKeyRb {}

impl PrivateKeyRb {
    fn from_hex(s: String) -> Result<Self, Error> {
        let key = PrivateKey::from_bytes_hex(&s).unwrap();
        Ok(PrivateKeyRb { key })
    }

    fn to_hex(&self) -> Result<String, Error> {
        Ok(self.key.to_bytes_hex())
    }
}

impl KeyPairRb {
    fn new() -> Self {
        KeyPairRb {
            key_pair: KeyPair::new(),
        }
    }

    fn from(private: &PrivateKeyRb) -> Result<Self, Error> {
        let key_pair = KeyPair::from(&private.key);
        Ok(KeyPairRb { key_pair })
    }

    fn public(&self) -> Result<PublicKeyRb, Error> {
        Ok(PublicKeyRb {
            key: self.key_pair.public(),
        })
    }

    fn private(&self) -> Result<PrivateKeyRb, Error> {
        Ok(PrivateKeyRb {
            key: self.key_pair.private(),
        })
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("KeyPair", class::object())?;
    class.define_singleton_method("new", function!(KeyPairRb::new, 0))?;
    class.define_singleton_method("from", function!(KeyPairRb::from, 1))?;
    class.define_method("public", method!(KeyPairRb::public, 0))?;
    class.define_method("private", method!(KeyPairRb::private, 0))?;

    let private_key = root().define_class("PrivateKey", class::object())?;
    private_key.define_singleton_method("from_hex", function!(PrivateKeyRb::from_hex, 1))?;
    private_key.define_method("to_hex", method!(PrivateKeyRb::to_hex, 0))?;

    let public_key = root().define_class("PublicKey", class::object())?;
    public_key.define_singleton_method("from_hex", function!(PublicKeyRb::from_hex, 1))?;
    public_key.define_method("to_hex", method!(PublicKeyRb::to_hex, 0))?;

    Ok(())
}
