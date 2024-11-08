#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use {
    crate::processor::Processor,
    miraland_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

miraland_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}
