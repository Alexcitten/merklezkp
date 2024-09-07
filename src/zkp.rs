use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof, VerifyingKey};
use bellman::{Circuit};
use bls12_381::{Bls12, Scalar as Fr};
use rand::rngs::OsRng;
use std::error::Error;

pub struct ZeroKnowledgeProof {
    proof: Proof<Bls12>,
    verifying_key: VerifyingKey<Bls12>,
}

impl ZeroKnowledgeProof {
    pub fn create(circuit: impl Circuit<Fr> + Clone) -> Result<Self, Box<dyn Error>> {
        let mut rng = OsRng;
        let params = generate_random_parameters::<Bls12, _, _>(circuit.clone(), &mut rng)?;
        let vk = params.vk.clone();
        let proof = create_random_proof(circuit, &params, &mut rng)?;
        Ok(ZeroKnowledgeProof { proof, verifying_key: vk })
    }

    pub fn validate(&self, inputs: &[Fr]) -> Result<bool, Box<dyn Error>> {
        let prepared_vk = prepare_verifying_key(&self.verifying_key);
        match verify_proof(&prepared_vk, &self.proof, inputs) {
            Ok(_) => Ok(true),
            Err(e) => Err(Box::new(e)),
        }
    }
}