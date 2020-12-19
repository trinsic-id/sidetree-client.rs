use secp256k1::{Message, PublicKey, RecoveryId, SecretKey, Signature};

pub struct KeyPair {
    public_key: PublicKey,
    secret_key: Option<SecretKey>,
}

impl KeyPair {
    pub fn sign(&self, message: &[u8]) -> (Signature, RecoveryId) {
        secp256k1::sign(
            &Message::parse_slice(message).unwrap(),
            &self.secret_key.as_ref().unwrap(),
        )
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        secp256k1::verify(
            &Message::parse_slice(message).unwrap(),
            &Signature::parse_slice(signature).unwrap(),
            &self.public_key,
        )
    }
}

pub fn generate() -> KeyPair {
    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed).expect("couldn't generate random seed");

    // TODO: Add rng support
    let secret_key = SecretKey::parse(&seed).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);

    KeyPair {
        public_key,
        secret_key: Some(secret_key),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_random_key() {
        let keypair = super::generate();

        assert!(true)
    }
}
