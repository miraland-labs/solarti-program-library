//! Program processor

mod process_add_required_signatory;
mod process_add_signatory;
mod process_cancel_proposal;
mod process_cast_vote;
mod process_complete_proposal;
mod process_create_governance;
mod process_create_mint_governance;
mod process_create_native_treasury;
mod process_create_program_governance;
mod process_create_proposal;
mod process_create_realm;
mod process_create_token_governance;
mod process_create_token_owner_record;
mod process_deposit_governing_tokens;
mod process_execute_transaction;
mod process_finalize_vote;
mod process_flag_transaction_error;
mod process_insert_transaction;
mod process_refund_proposal_deposit;
mod process_relinquish_vote;
mod process_remove_required_signatory;
mod process_remove_transaction;
mod process_revoke_governing_tokens;
mod process_set_governance_config;
mod process_set_governance_delegate;
mod process_set_realm_authority;
mod process_set_realm_config;
mod process_sign_off_proposal;
mod process_update_program_metadata;
mod process_withdraw_governing_tokens;

use {
    crate::{error::GovernanceError, instruction::GovernanceInstruction},
    process_add_required_signatory::*,
    process_add_signatory::*,
    process_cancel_proposal::*,
    process_cast_vote::*,
    process_complete_proposal::*,
    process_create_governance::*,
    process_create_mint_governance::*,
    process_create_native_treasury::*,
    process_create_program_governance::*,
    process_create_proposal::*,
    process_create_realm::*,
    process_create_token_governance::*,
    process_create_token_owner_record::*,
    process_deposit_governing_tokens::*,
    process_execute_transaction::*,
    process_finalize_vote::*,
    process_flag_transaction_error::*,
    process_insert_transaction::*,
    process_refund_proposal_deposit::*,
    process_relinquish_vote::*,
    process_remove_required_signatory::*,
    process_remove_transaction::*,
    process_revoke_governing_tokens::*,
    process_set_governance_config::*,
    process_set_governance_delegate::*,
    process_set_realm_authority::*,
    process_set_realm_config::*,
    process_sign_off_proposal::*,
    process_update_program_metadata::*,
    process_withdraw_governing_tokens::*,
    solana_program::{
        account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult,
        msg, program_error::ProgramError, pubkey::Pubkey,
    },
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
    // Use try_from_slice_unchecked to support forward compatibility of newer UI
    // with older program
    let instruction: GovernanceInstruction =
        try_from_slice_unchecked(input).map_err(|_| ProgramError::InvalidInstructionData)?;

    if let GovernanceInstruction::InsertTransaction {
        option_index,
        index,
        hold_up_time,
        instructions: _,
    } = instruction
    {
        // Do not dump instruction data into logs
        msg!(
            "GOVERNANCE-INSTRUCTION: InsertInstruction {{option_index: {:?}, index: {:?}, hold_up_time: {:?} }}",
            option_index,
            index,
            hold_up_time
        );
    } else {
        msg!("GOVERNANCE-INSTRUCTION: {:?}", instruction);
    }

    match instruction {
        GovernanceInstruction::CreateRealm { name, config_args } => {
            process_create_realm(program_id, accounts, name, config_args)
        }

        GovernanceInstruction::DepositGoverningTokens { amount } => {
            process_deposit_governing_tokens(program_id, accounts, amount)
        }

        GovernanceInstruction::WithdrawGoverningTokens {} => {
            process_withdraw_governing_tokens(program_id, accounts)
        }

        GovernanceInstruction::SetGovernanceDelegate {
            new_governance_delegate,
        } => process_set_governance_delegate(program_id, accounts, &new_governance_delegate),

        GovernanceInstruction::CreateProgramGovernance {
            config,
            transfer_upgrade_authority,
        } => process_create_program_governance(
            program_id,
            accounts,
            config,
            transfer_upgrade_authority,
        ),

        GovernanceInstruction::CreateMintGovernance {
            config,
            transfer_mint_authorities,
        } => {
            process_create_mint_governance(program_id, accounts, config, transfer_mint_authorities)
        }

        GovernanceInstruction::CreateTokenGovernance {
            config,
            transfer_account_authorities,
        } => process_create_token_governance(
            program_id,
            accounts,
            config,
            transfer_account_authorities,
        ),

        GovernanceInstruction::CreateGovernance { config } => {
            process_create_governance(program_id, accounts, config)
        }

        GovernanceInstruction::CreateProposal {
            name,
            description_link,
            vote_type: proposal_type,
            options,
            use_deny_option,
            proposal_seed,
        } => process_create_proposal(
            program_id,
            accounts,
            name,
            description_link,
            proposal_type,
            options,
            use_deny_option,
            proposal_seed,
        ),
        GovernanceInstruction::AddSignatory { signatory } => {
            process_add_signatory(program_id, accounts, signatory)
        }
        GovernanceInstruction::Legacy1 => {
            Err(GovernanceError::InstructionDeprecated.into()) // No-op
        }
        GovernanceInstruction::SignOffProposal {} => {
            process_sign_off_proposal(program_id, accounts)
        }
        GovernanceInstruction::CastVote { vote } => process_cast_vote(program_id, accounts, vote),

        GovernanceInstruction::FinalizeVote {} => process_finalize_vote(program_id, accounts),

        GovernanceInstruction::RelinquishVote {} => process_relinquish_vote(program_id, accounts),

        GovernanceInstruction::CancelProposal {} => process_cancel_proposal(program_id, accounts),

        GovernanceInstruction::InsertTransaction {
            option_index,
            index,
            hold_up_time,
            instructions,
        } => process_insert_transaction(
            program_id,
            accounts,
            option_index,
            index,
            hold_up_time,
            instructions,
        ),

        GovernanceInstruction::RemoveTransaction {} => {
            process_remove_transaction(program_id, accounts)
        }
        GovernanceInstruction::ExecuteTransaction {} => {
            process_execute_transaction(program_id, accounts)
        }

        GovernanceInstruction::SetGovernanceConfig { config } => {
            process_set_governance_config(program_id, accounts, config)
        }

        GovernanceInstruction::FlagTransactionError {} => {
            process_flag_transaction_error(program_id, accounts)
        }
        GovernanceInstruction::SetRealmAuthority { action } => {
            process_set_realm_authority(program_id, accounts, action)
        }
        GovernanceInstruction::SetRealmConfig { config_args } => {
            process_set_realm_config(program_id, accounts, config_args)
        }
        GovernanceInstruction::CreateTokenOwnerRecord {} => {
            process_create_token_owner_record(program_id, accounts)
        }
        GovernanceInstruction::UpdateProgramMetadata {} => {
            process_update_program_metadata(program_id, accounts)
        }
        GovernanceInstruction::CreateNativeTreasury {} => {
            process_create_native_treasury(program_id, accounts)
        }

        GovernanceInstruction::RevokeGoverningTokens { amount } => {
            process_revoke_governing_tokens(program_id, accounts, amount)
        }

        GovernanceInstruction::RefundProposalDeposit {} => {
            process_refund_proposal_deposit(program_id, accounts)
        }

        GovernanceInstruction::CompleteProposal {} => {
            process_complete_proposal(program_id, accounts)
        }

        GovernanceInstruction::AddRequiredSignatory { signatory } => {
            process_add_required_signatory(program_id, accounts, signatory)
        }
        GovernanceInstruction::RemoveRequiredSignatory => {
            process_remove_required_signatory(program_id, accounts)
        }
    }
}
