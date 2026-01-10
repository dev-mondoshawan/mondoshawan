//! Performance Benchmarks for Privacy Layer
//!
//! Measures proof generation time, verification time, and proof size.

#[cfg(test)]
mod benchmarks {
    use crate::privacy::*;
    use crate::privacy::{
        generate_keys, PrivacyProver, PrivacyVerifier, PrivateTransferCircuit,
        PedersenCommitment, Nullifier,
    };
    use ark_bn254::Fr;
    use ark_std::test_rng;
    use std::time::Instant;

    #[test]
    fn benchmark_key_generation() {
        let mut rng = test_rng();
        let start = Instant::now();
        
        let (pk, vk) = generate_keys(&mut rng).unwrap();
        
        let duration = start.elapsed();
        println!("Key generation time: {:?}", duration);
        
        // Keys should be generated successfully
        assert!(pk.alpha_g1 != ark_bn254::G1Projective::zero());
        assert!(vk.alpha_g1_beta_g2 != ark_bn254::G2Projective::zero());
        
        // Key generation should complete in reasonable time (< 10 seconds for test)
        assert!(duration.as_secs() < 10);
    }

    #[test]
    fn benchmark_proof_generation() {
        let mut rng = test_rng();
        
        // Generate keys
        let (pk, _vk) = generate_keys(&mut rng).unwrap();
        let prover = PrivacyProver::new(pk);
        
        // Create circuit with witness values
        let old_balance = 1000u128;
        let amount = 100u128;
        let new_balance = 900u128;
        let nullifier = Fr::from(42u64);
        let commitment = Fr::from(123u64);
        
        // Benchmark proof generation
        let start = Instant::now();
        let proof = prover.prove_private_transfer(
            old_balance,
            amount,
            new_balance,
            nullifier,
            commitment,
            &mut rng,
        ).unwrap();
        let duration = start.elapsed();
        
        println!("Proof generation time: {:?}", duration);
        println!("Proof size: {} bytes", PrivacyProver::serialize_proof(&proof).unwrap().len());
        
        // Proof generation should complete in reasonable time
        assert!(duration.as_secs() < 30);
    }

    #[test]
    fn benchmark_proof_verification() {
        let mut rng = test_rng();
        
        // Generate keys
        let (pk, vk) = generate_keys(&mut rng).unwrap();
        let prover = PrivacyProver::new(pk.clone());
        let verifier = PrivacyVerifier::new(vk);
        
        // Create circuit with witness values
        let old_balance = 1000u128;
        let amount = 100u128;
        let new_balance = 900u128;
        let nullifier = Fr::from(42u64);
        let commitment = Fr::from(123u64);
        
        // Generate proof
        let proof = prover.prove_private_transfer(
            old_balance,
            amount,
            new_balance,
            nullifier,
            commitment,
            &mut rng,
        ).unwrap();
        
        // Benchmark verification
        let circuit = PrivateTransferCircuit {
            old_balance: Some(old_balance),
            amount: Some(amount),
            new_balance: Some(new_balance),
            nullifier,
            commitment: Some(commitment),
        };
        
        let public_inputs = circuit.public_inputs();
        let start = Instant::now();
        let verified = verifier.verify_with_inputs(&proof, &public_inputs);
        let duration = start.elapsed();
        
        println!("Proof verification time: {:?}", duration);
        
        assert!(verified, "Proof verification should succeed");
        // Verification should be very fast (< 100ms)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn benchmark_commitment_creation() {
        let amount = 1000u128;
        let blinding = [42u8; 32];
        
        let start = Instant::now();
        let commitment = PedersenCommitment::commit(amount, &blinding);
        let duration = start.elapsed();
        
        println!("Commitment creation time: {:?}", duration);
        
        assert_ne!(commitment.point, ark_bn254::G1Projective::zero());
        // Commitment creation should be very fast (< 1ms)
        assert!(duration.as_micros() < 1000);
    }

    #[test]
    fn benchmark_nullifier_generation() {
        let receiver = [1u8; 20];
        let blinding = [42u8; 32];
        
        let start = Instant::now();
        let nullifier = Nullifier::generate(&receiver, &blinding);
        let duration = start.elapsed();
        
        println!("Nullifier generation time: {:?}", duration);
        
        assert_ne!(nullifier.hash, [0u8; 32]);
        // Nullifier generation should be very fast (< 1ms)
        assert!(duration.as_micros() < 1000);
    }
}
