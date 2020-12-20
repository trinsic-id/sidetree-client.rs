use crate::{
    did::compute_unique_suffix,
    multihash::{canonicalize_then_double_hash_then_encode, canonicalize_then_hash_then_encode},
    secp256k1::KeyPair,
    Delta, Document, JsonWebKey, Patch, PublicKey, SuffixData,
};
use serde::Serialize;

pub enum OperationType {
    Create,
    Update,
    Recover,
    Deactivate,
}

pub struct OperationRequest {
    did_unique_suffix: Option<String>,
    suffix_data: Option<String>,
    operation_type: OperationType,
    delta: Delta,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateOperation {
    suffix_data: SuffixData,
    delta: Delta,
}

pub fn create_operation() -> CreateOperationResult {
    let update_key = KeyPair::random();
    let recovery_key = KeyPair::random();

    let signing_key = KeyPair::random();
    let signing_key_public = signing_key.to_public_key(
        "key-1".into(),
        Some(vec![
            "authentication".into(),
            "assertionMethod".into(),
            "capabilityInvocation".into(),
            "capabilityDelegation".into(),
            "keyAgreement".into(),
        ]),
    );

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

    let operation = CreateOperation {
        suffix_data: suffix_data.clone(),
        delta,
    };

    let did_suffix = compute_unique_suffix(&suffix_data);

    CreateOperationResult {
        update_key: (&update_key).into(),
        recovery_key: (&recovery_key).into(),
        signing_key: signing_key_public,
        operation,
        did_suffix,
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct CreateOperationResult {
    operation: CreateOperation,
    did_suffix: String,
    update_key: JsonWebKey,
    recovery_key: JsonWebKey,
    signing_key: PublicKey,
}

#[cfg(test)]
mod test {
    use super::create_operation;

    #[test]
    fn generate_create_operation() {
        let result = create_operation();
        let json = serde_json::to_string_pretty(&result);

        println!("{}", json.unwrap());
    }
}
