//! Program entrypoint

use {
    crate::processor,
    miraland_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

miraland_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    processor::process(program_id, accounts, input)
}
