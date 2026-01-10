//! Key Generation (Trusted Setup)
//!
//! Generates proving and verifying keys for zk-SNARK circuits.
//! In production, this would use a trusted setup ceremony.

use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_std::rand::RngCore;
use crate::privacy::circuit::PrivateTransferCircuit;

/// Generate proving and verifying keys for private transfer circuit
pub fn generate_keys<R: RngCore>(
    rng: &mut R,
) -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), String> {
    // Create a dummy circuit for key generation
    // The circuit structure must match the actual circuit
    let circuit = PrivateTransferCircuit {
        old_balance: None, // No witness values needed for key generation
        amount: None,
        new_balance: None,
        nullifier: Fr::zero(),
        commitment: None,
    };

    // Generate keys using Groth16
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)
        .map_err(|e| format!("Key generation failed: {:?}", e))?;

    Ok((pk, vk))
}

/// Load keys from bytes (for production use with trusted setup)
pub fn load_keys_from_bytes(
    pk_bytes: &[u8],
    vk_bytes: &[u8],
) -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), String> {
    let pk = ProvingKey::<Bn254>::deserialize(pk_bytes)
        .map_err(|e| format!("Failed to deserialize proving key: {:?}", e))?;
    
    let vk = VerifyingKey::<Bn254>::deserialize(vk_bytes)
        .map_err(|e| format!("Failed to deserialize verifying key: {:?}", e))?;

    Ok((pk, vk))
}

/// Serialize keys to bytes
pub fn serialize_keys(
    pk: &ProvingKey<Bn254>,
    vk: &VerifyingKey<Bn254>,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut pk_bytes = Vec::new();
    pk.serialize(&mut pk_bytes)
        .map_err(|e| format!("Failed to serialize proving key: {:?}", e))?;

    let mut vk_bytes = Vec::new();
    vk.serialize(&mut vk_bytes)
        .map_err(|e| format!("Failed to serialize verifying key: {:?}", e))?;

    Ok((pk_bytes, vk_bytes))
}
