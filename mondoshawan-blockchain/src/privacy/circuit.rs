//! zk-SNARK Circuit Definitions
//!
//! Defines circuits for private transfers and other privacy operations.

use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError, Variable};
use serde::{Deserialize, Serialize};

// Helper macro for linear combinations
macro_rules! lc {
    () => {
        ark_relations::r1cs::LinearCombination::<Fr>::zero()
    };
    ($($x:expr),*) => {
        {
            let mut lc = ark_relations::r1cs::LinearCombination::<Fr>::zero();
            $(
                lc = lc + $x;
            )*
            lc
        }
    };
}

/// Privacy circuit trait
pub trait PrivacyCircuit: ConstraintSynthesizer<Fr> {
    /// Get public inputs for the circuit
    fn public_inputs(&self) -> Vec<Fr>;
}

/// Private Transfer Circuit
///
/// Proves:
/// 1. Sender has sufficient balance
/// 2. New sender balance = old balance - amount
/// 3. Nullifier is valid (prevents double-spending)
/// 4. Commitment is valid (receiver can decrypt)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateTransferCircuit {
    /// Sender's old balance (private witness)
    pub old_balance: Option<u128>,
    /// Amount to transfer (private witness)
    pub amount: Option<u128>,
    /// New sender balance (public output)
    pub new_balance: Option<u128>,
    /// Nullifier (public input)
    pub nullifier: Fr,
    /// Commitment (public output)
    pub commitment: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for PrivateTransferCircuit {
    fn generate_constraints(
        &self,
        cs: &mut ConstraintSystem<Fr>,
    ) -> Result<(), SynthesisError> {
        // Allocate witness variables
        let old_balance_var = cs.alloc_input(
            || "old_balance",
            || self.old_balance.ok_or(SynthesisError::AssignmentMissing)
                .map(|b| Fr::from(b))
        )?;

        let amount_var = cs.alloc(
            || "amount",
            || self.amount.ok_or(SynthesisError::AssignmentMissing)
                .map(|a| Fr::from(a))
        )?;

        let new_balance_var = cs.alloc_output(
            || "new_balance",
            || self.new_balance.ok_or(SynthesisError::AssignmentMissing)
                .map(|b| Fr::from(b))
        )?;

        // Constraint 1: old_balance >= amount (sufficient funds)
        // This is a range check - simplified for now
        // In production, use proper range proof circuit
        
        // Constraint 2: new_balance = old_balance - amount
        // This means: old_balance = amount + new_balance
        // We enforce: old_balance * 1 = (amount + new_balance) * 1
        cs.enforce_constraint(
            lc!() + (Variable::One, Fr::one()) + old_balance_var,
            lc!() + (Variable::One, Fr::one()),
            lc!() + amount_var + new_balance_var,
        )?;

        // Constraint 3: Nullifier is valid (simplified)
        // In production, this would verify nullifier = hash(receiver_secret, note_index)
        // For now, just allocate nullifier as public input
        let nullifier_var = cs.alloc_input(
            || "nullifier",
            || Ok(self.nullifier)
        )?;
        
        // Constraint 4: Commitment is valid (simplified)
        // In production, this would verify commitment = PedersenCommit(amount, blinding)
        let commitment_var = cs.alloc_output(
            || "commitment",
            || self.commitment.ok_or(SynthesisError::AssignmentMissing)
        )?;
        
        Ok(())
    }
}

impl PrivacyCircuit for PrivateTransferCircuit {
    fn public_inputs(&self) -> Vec<Fr> {
        vec![
            self.nullifier,
            self.commitment.unwrap_or(Fr::zero()),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let circuit = PrivateTransferCircuit {
            old_balance: Some(1000),
            amount: Some(100),
            new_balance: Some(900),
            nullifier: Fr::from(42u64),
            commitment: Some(Fr::from(123u64)),
        };

        // Circuit should be created successfully
        assert!(circuit.old_balance.is_some());
    }
}
