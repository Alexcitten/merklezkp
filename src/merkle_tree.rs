use sha2::{Sha256, Digest};
use hex::encode;
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root_hash: String,
    leaf_nodes: Vec<String>,
    hash_function: fn(&[u8]) -> String,
}

impl MerkleTree {
    pub fn new(data: &[&[u8]], hash_function: fn(&[u8]) -> String) -> Self {
        let leaf_nodes: Vec<String> = data.iter().map(|d| hash_function(d)).collect();
        let root_hash = MerkleTree::construct_root(&leaf_nodes, hash_function);
        MerkleTree { root_hash, leaf_nodes, hash_function }
    }

    fn construct_root(leaves: &[String], hash_function: fn(&[u8]) -> String) -> String {
        if leaves.len() == 1 {
            return leaves[0].clone()
        }

        let mut parent_nodes = vec![];
        for i in (0..leaves.len()).step_by(2) {
            let left = &leaves[i];
            let right = if i + 1 < leaves.len() { &leaves[i + 1] } else { left };
            let parent = hash_function(&[left.as_bytes(), right.as_bytes()].concat());
            parent_nodes.push(parent)
        }

        MerkleTree::construct_root(&parent_nodes, hash_function)
    }
    pub fn root(&self) -> &String {&self.root_hash}

    pub fn generate_proof(&self, index: usize) -> Vec<(String, bool)> {
        let mut proof = vec![];
        let mut idx = index;
        let mut current_level = self.leaf_nodes.clone();
        while current_level.len() > 1 {
            let mut parent_nodes = vec![];
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() { &current_level[i + 1] } else { left };
                let parent = (self.hash_function)(&[left.as_bytes(), right.as_bytes()].concat());
                parent_nodes.push(parent);
                if i == idx {proof.push((right.clone(), true));} else if i + 1 == idx {proof.push((left.clone(), false))}
            } idx /= 2; current_level = parent_nodes
        } proof
    }

    pub fn validate_proof(proof: &[(String, bool)], root: &String, leaf: &String, hash_function: fn(&[u8]) -> String) -> bool {
        let mut current_hash = leaf.clone();
        for (sibling, is_right_sibling) in proof {
            if *is_right_sibling {current_hash = hash_function(&[current_hash.as_bytes(), sibling.as_bytes()].concat())
            } else {current_hash = hash_function(&[sibling.as_bytes(), current_hash.as_bytes()].concat())}
        } current_hash == *root
    }
}

impl Default for MerkleTree {
    fn default() -> Self {Self::new(&[], default_hash_function)}
}
pub fn default_hash_function(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    encode(hasher.finalize())
}
