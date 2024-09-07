pub mod merkle_tree;
pub mod zkp;
pub mod utils;
pub use crate::merkle_tree::MerkleTree;
pub use crate::zkp::ZeroKnowledgeProof;
pub fn default_hash_function(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    use hex::encode;
    let mut hasher = Sha256::new();
    hasher.update(data);
    encode(hasher.finalize())
}
