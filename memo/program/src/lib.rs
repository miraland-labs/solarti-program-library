#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it parses,
//! while verifying and logging signers. Currently handles UTF-8 characters.

mod entrypoint;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk version
pub use miraland_program;
use miraland_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// Legacy symbols from Memo v1
pub mod v1 {
    miraland_program::declare_id!("MemojWWmbFiRdUEQtRpMkeeyNB181Mr9uWEzdrgHUnc");
}

miraland_program::declare_id!("MemoE5FsL5zWDjihivRfHTpkR8RdviRbziKsfGS9Ntd");

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will be processed as a
///     normal, unsigned solarti-memo
///
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_memo() {
        let signer_pubkey = Pubkey::new_unique();
        let memo = "🐆".as_bytes();
        let instruction = build_memo(memo, &[&signer_pubkey]);
        assert_eq!(memo, instruction.data);
        assert_eq!(instruction.accounts.len(), 1);
        assert_eq!(instruction.accounts[0].pubkey, signer_pubkey);
    }
}
