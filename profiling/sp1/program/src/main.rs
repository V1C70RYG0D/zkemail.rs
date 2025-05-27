//! SP1 zkVM Guest Program for Email Verification
//! 
//! This program runs inside the SP1 zkVM and verifies email signatures
//! while providing zero-knowledge proofs of the verification process.

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use zkemail_core::*;

#[derive(Serialize, Deserialize)]
pub struct EmailVerificationInput {
    pub email: Email,
}

#[derive(Serialize, Deserialize)]
pub struct EmailVerificationOutput {
    pub verification_result: EmailVerifierOutput,
    pub is_valid: bool,
    pub cycle_count: u64,
}

pub fn main() {
    // Read input from the host
    let input = sp1_zkvm::io::read::<EmailVerificationInput>();
    
    // Get cycle count at start for benchmarking
    let start_cycles = sp1_zkvm::io::cycle_count();
    
    // Perform email verification
    let verification_result = match verify_email(&input.email) {
        Ok(result) => {
            let end_cycles = sp1_zkvm::io::cycle_count();
            EmailVerificationOutput {
                verification_result: result,
                is_valid: true,
                cycle_count: end_cycles - start_cycles,
            }
        }
        Err(_) => {
            let end_cycles = sp1_zkvm::io::cycle_count();
            EmailVerificationOutput {
                verification_result: EmailVerifierOutput {
                    from_domain_hash: vec![],
                    public_key_hash: vec![],
                    external_inputs: vec![],
                },
                is_valid: false,
                cycle_count: end_cycles - start_cycles,
            }
        }
    };
    
    // Commit the result
    sp1_zkvm::io::commit(&verification_result);
}
