//! Proof Verification
//!
//! Verifies zk-SNARK proofs for privacy operations.

use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey};
use crate::privacy::circuit::PrivacyCircuit;

/// Privacy Verifier
pub struct PrivacyVerifier {
    /// Verifying key (public, from trusted setup)
    verifying_key: VerifyingKey<Bn254>,
}

impl PrivacyVerifier {
    /// Create new verifier with verifying key
    pub fn new(verifying_key: VerifyingKey<Bn254>) -> Self {
        Self { verifying_key }
    }

    /// Verify a proof
    pub fn verify<C: PrivacyCircuit>(
        &self,
        proof: &Proof<Bn254>,
        circuit: &C,
    ) -> bool {
        let public_inputs = circuit.public_inputs();
        Groth16::<Bn254>::verify(&self.verifying_key, &public_inputs, proof)
            .unwrap_or(false)
    }

    /// Deserialize proof from bytes
    pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof<Bn254>, String> {
        Proof::<Bn254>::deserialize(bytes)
            .map_err(|e| format!("Proof deserialization failed: {:?}", e))
    }
}
