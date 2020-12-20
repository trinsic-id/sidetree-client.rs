use crate::{
    encoder::encode,
    multihash::{canonicalize, hash, HashAlgorithm},
    SuffixData,
};

pub(crate) fn compute_unique_suffix(suffix_data: &SuffixData) -> String {
    let suffix_data_buffer = canonicalize(suffix_data).unwrap();
    let multihash = hash(&suffix_data_buffer, HashAlgorithm::Sha256);

    encode(multihash)
}
