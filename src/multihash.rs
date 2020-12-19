use std::todo;

use ::multihash::{Code, MultihashDigest};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn canonicalize<T>(value: &T) -> Result<Vec<u8>, String>
where
    T: Serialize + ?Sized,
{
    serde_jcs::to_vec(value).map_err(|err| format!("{}", err))
}

pub fn canonicalize_then_double_hash_then_encode<T>(value: &T) -> Result<String, String>
where
    T: Serialize + ?Sized,
{
    let content_buffer = match canonicalize(value) {
        Ok(x) => x,
        Err(_) => return Err("failed to canonicalize".to_string()),
    };

    // Double hash.
    // Default to SHA256.
    let intermediate_hash_buffer = hash_as_non_multihash_buffer(content_buffer.as_slice(), HashAlgorithm::Sha256);
    let multihash_encoded_string = hash_then_encode(intermediate_hash_buffer.as_slice(), HashAlgorithm::Sha256);
    return Ok(multihash_encoded_string);
}

pub(crate) fn hash(buffer: &[u8], algorithm: HashAlgorithm) -> Vec<u8> {
    match algorithm {
        HashAlgorithm::Sha256 => Code::Sha2_256.digest(buffer).to_bytes(),
        HashAlgorithm::Sha3_256 => Code::Sha3_256.digest(buffer).to_bytes(),
    }
}

pub(crate) fn hash_as_non_multihash_buffer(buffer: &[u8], algorithm: HashAlgorithm) -> Vec<u8> {
    match algorithm {
        HashAlgorithm::Sha256 => Sha256::digest(buffer).to_vec(),
        HashAlgorithm::Sha3_256 => todo!(),
    }
}

pub(crate) fn hash_then_encode(buffer: &[u8], algorithm: HashAlgorithm) -> String {
    let multihash_buffer = hash(buffer, algorithm);
    return base64::encode_config(multihash_buffer, base64::URL_SAFE);
}

pub(crate) enum HashAlgorithm {
    Sha256,
    Sha3_256,
}

#[cfg(test)]
pub mod test {
    use ::multihash::{Code, MultihashDigest};

    #[test]
    pub fn test() {
        let digest = Code::Sha2_256.digest(b"hello");

        println!("{:?}", digest);
        println!("{:?}", digest.to_bytes())
    }
}
