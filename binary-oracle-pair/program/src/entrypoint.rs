//! Program entrypoint definitions

#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use {
    crate::{error::PoolError, processor},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
};

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) =
        processor::Processor::process_instruction(program_id, accounts, instruction_data)
    {
        // catch the error so we can print it
        error.print::<PoolError>();
        return Err(error);
    }
    Ok(())
}
