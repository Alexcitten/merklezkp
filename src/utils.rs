pub fn compute_hash(input: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    use hex::encode;
    let mut hasher = Sha256::new();
    hasher.update(input);
    encode(hasher.finalize())
}