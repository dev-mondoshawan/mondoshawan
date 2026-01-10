//! Proof Generation
//!
//! Generates zk-SNARK proofs for privacy operations.

use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_std::rand::RngCore;
use crate::privacy::circuit::PrivacyCircuit;

/// Privacy Prover
pub struct PrivacyProver {
    /// Proving key (from trusted setup)
    proving_key: ProvingKey<Bn254>,
}

impl PrivacyProver {
    /// Create new prover with proving key
    pub fn new(proving_key: ProvingKey<Bn254>) -> Self {
        Self { proving_key }
    }

    /// Generate proof for a circuit
    pub fn prove<C: ConstraintSynthesizer<Fr>>(
        &self,
        circuit: C,
        rng: &mut dyn RngCore,
    ) -> Result<Proof<Bn254>, String> {
        Groth16::<Bn254>::prove(&self.proving_key, circuit, rng)
            .map_err(|e| format!("Proof generation failed: {:?}", e))
    }

    /// Serialize proof to bytes
    pub fn serialize_proof(proof: &Proof<Bn254>) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        proof.serialize(&mut bytes)
            .map_err(|e| format!("Proof serialization failed: {:?}", e))?;
        Ok(bytes)
    }
}
