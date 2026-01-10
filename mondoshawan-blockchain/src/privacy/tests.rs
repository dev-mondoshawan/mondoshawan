#[cfg(test)]
mod tests {
    use super::*;
    use crate::privacy::{Commitment, Nullifier, PedersenCommitment};
    use crate::types::Address;

    #[test]
    fn test_commitment_creation() {
        let amount = 1000u128;
        let blinding = [42u8; 32];
        
        let commitment = PedersenCommitment::commit(amount, &blinding);
        
        // Commitment should be non-zero
        assert_ne!(commitment.point, ark_bn254::G1Projective::zero());
    }

    #[test]
    fn test_nullifier_generation() {
        let receiver = [1u8; 20];
        let blinding = [42u8; 32];
        
        let nullifier = Nullifier::generate(&receiver, &blinding);
        
        // Nullifier should be non-zero
        assert_ne!(nullifier.hash, [0u8; 32]);
    }

    #[test]
    fn test_privacy_note_creation() {
        let amount = 1000u128;
        let receiver = [1u8; 20];
        let blinding = [42u8; 32];
        
        let note = PrivacyNote::new(amount, receiver, blinding);
        
        assert_eq!(note.amount, amount);
        assert_eq!(note.receiver, receiver);
    }
}
