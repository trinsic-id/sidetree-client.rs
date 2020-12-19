use serde::Serialize;

pub enum OperationType {
    Create,
    Update,
    Recover,
    Deactivate,
}

struct Operation<T> {
    did_unique_suffix: String,
    operation_type: OperationType,
    operation: T,
}

struct CreateOperation {}

#[derive(Debug, Serialize)]
struct Delta<T>
where
    T: Serialize + Sized,
{
    patches: Vec<T>,
    update_commitment: String,
}

#[derive(Debug, Serialize)]
struct SuffixData {
    #[serde(rename = "deltaHash")]
    delta_hash: String,
    #[serde(rename = "recoveryCommitment")]
    recovery_commitment: String,
    #[serde(rename = "type")]
    data_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct AddPublicKey {
    action: String,
    public_keys: Vec<PublicKey>,
}

#[derive(Debug, Serialize)]
struct PublicKey {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    purposes: Option<Vec<u8>>,
}

fn canonicalize<T>(value: &T) -> Result<Vec<u8>, String>
where
    T: Serialize + ?Sized,
{
    let commitment = Delta::<AddPublicKey> {
        patches: vec![],
        update_commitment: String::default(),
    };
    serde_jcs::to_vec(value).map_err(|err| format!("{}", err))
}

pub mod secp256k1;
