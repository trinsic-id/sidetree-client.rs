use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
struct Delta {
    patches: Vec<Patch>,
    update_commitment: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Document {
    #[serde(rename = "publicKeys")]
    public_keys: Vec<PublicKey>,
    services: Vec<Service>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Service {
    id: String,
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct SuffixData {
    #[serde(rename = "deltaHash")]
    delta_hash: String,
    #[serde(rename = "recoveryCommitment")]
    recovery_commitment: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    data_type: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub enum Patch {
    AddPublicKeys(Vec<PublicKey>),
    RemovePublicKeys(Vec<String>),
    AddServices(Vec<Service>),
    RemoveServices(Vec<String>),
    Replace(Document),
    IetfJsonPatch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct JsonWebKey {
    #[serde(rename = "kty")]
    key_type: String,
    #[serde(rename = "crv")]
    curve: String,
    x: String,
    y: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    d: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicKey {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    purposes: Option<Vec<String>>,
    #[serde(rename = "publicKeyJwk", skip_serializing_if = "Option::is_none")]
    jwk: Option<JsonWebKey>,
}

mod did;
mod encoder;
mod multihash;
pub mod operations;
pub mod secp256k1;
