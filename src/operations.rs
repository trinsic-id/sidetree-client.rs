use crate::{
    did::*,
    multihash::{canonicalize_then_double_hash_then_encode, canonicalize_then_hash_then_encode},
    secp256k1::KeyPair,
    Delta, Patch, SuffixData,
};
use serde::{ser::SerializeMap, Serialize};

#[derive(Debug, Clone)]
pub enum Operation {
    Create(SuffixData, Delta),
}

impl Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        match self {
            Operation::Create(suffix_data, delta) => {
                map.serialize_entry("type", "create")?;
                map.serialize_entry("suffixData", suffix_data)?;
                map.serialize_entry("delta", delta)?;
            }
        }

        map.end()
    }
}

pub fn create() -> OperationResult {
    let update_key = KeyPair::random();
    let recovery_key = KeyPair::random();

    let signing_key = KeyPair::random();
    let signing_key_public = signing_key.to_public_key("key-1".into(), Some(Purpose::all()));

    let document = Document {
        public_keys: vec![signing_key_public.clone()],
        services: vec![],
    };

    let patches = vec![Patch::Replace(document)];

    let mut update_key_public: JsonWebKey = (&update_key).into();
    update_key_public.d = None;

    let delta = Delta {
        update_commitment: canonicalize_then_double_hash_then_encode(&update_key_public).unwrap(),
        patches,
    };

    let delta_hash = canonicalize_then_hash_then_encode(&delta, crate::multihash::HashAlgorithm::Sha256);

    let mut recovery_key_public: JsonWebKey = (&recovery_key).into();
    recovery_key_public.d = None;

    let suffix_data = SuffixData {
        delta_hash,
        recovery_commitment: canonicalize_then_double_hash_then_encode(&recovery_key_public).unwrap(),
        data_type: None,
    };

    let did_suffix = compute_unique_suffix(&suffix_data);
    let operation = Operation::Create(suffix_data, delta);

    OperationResult {
        update_key: (&update_key).into(),
        recovery_key: (&recovery_key).into(),
        signing_key: signing_key_public,
        operation,
        did_suffix,
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct OperationResult {
    operation: Operation,
    did_suffix: String,
    update_key: JsonWebKey,
    recovery_key: JsonWebKey,
    signing_key: PublicKey,
}

#[cfg(test)]
mod test {
    use super::{create, Operation};

    #[test]
    fn generate_create_operation() {
        let result = create();
        let json = serde_json::to_string_pretty(&result.operation);

        assert!(matches!(json, Result::Ok(_)));
        assert!(matches!(result.operation, Operation::Create(_, _)));
        assert!(result.did_suffix.len() > 0);

        println!("{}", json.unwrap());
    }
}
