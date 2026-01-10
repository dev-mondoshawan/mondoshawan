//! Proof Verification
//!
//! Verifies zk-SNARK proofs for privacy operations.

use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey};
use crate::privacy::circuit::PrivacyCircuit;
use std::io::Cursor;

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
        let mut cursor = Cursor::new(bytes);
        Proof::<Bn254>::deserialize(&mut cursor)
            .map_err(|e| format!("Proof deserialization failed: {:?}", e))
    }

    /// Verify proof with public inputs
    pub fn verify_with_inputs(
        &self,
        proof: &Proof<Bn254>,
        public_inputs: &[Fr],
    ) -> bool {
        Groth16::<Bn254>::verify(&self.verifying_key, public_inputs, proof)
            .unwrap_or(false)
    }
}
