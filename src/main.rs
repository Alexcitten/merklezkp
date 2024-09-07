use merkle_zkp_project::{MerkleTree, default_hash_function};

fn main() {
    let data = vec!["a".as_bytes(), "b".as_bytes(), "c".as_bytes(), "d".as_bytes()];
    let tree = MerkleTree::new(&data, default_hash_function);
    
    println!("Merkle Root: {}", tree.root());
    let proof = tree.generate_proof(0);
    println!("Proof for first element: {:?}", proof);
    let leaf_hash = default_hash_function(data[0]);
    println!("Leaf hash: {}", leaf_hash);
    let is_valid = MerkleTree::validate_proof(&proof, tree.root(), &leaf_hash, default_hash_function);
    println!("Is proof valid? {}", is_valid)
}