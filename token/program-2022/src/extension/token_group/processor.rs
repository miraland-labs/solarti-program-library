//! Token-group processor

use {
    crate::{
        check_program_account,
        error::TokenError,
        extension::{
            alloc_and_serialize, group_pointer::GroupPointer, BaseStateWithExtensions,
            StateWithExtensions, StateWithExtensionsMut,
        },
        state::Mint,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_option::COption,
        pubkey::Pubkey,
    },
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_group_interface::{
        error::TokenGroupError,
        instruction::{
            InitializeGroup, TokenGroupInstruction, UpdateGroupAuthority, UpdateGroupMaxSize,
        },
        state::TokenGroup,
    },
};

fn check_update_authority(
    update_authority_info: &AccountInfo,
    expected_update_authority: &OptionalNonZeroPubkey,
) -> Result<(), ProgramError> {
    if !update_authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let update_authority = Option::<Pubkey>::from(*expected_update_authority)
        .ok_or(TokenGroupError::ImmutableGroup)?;
    if update_authority != *update_authority_info.key {
        return Err(TokenGroupError::IncorrectUpdateAuthority.into());
    }
    Ok(())
}

/// Processes a [InitializeGroup](enum.TokenGroupInstruction.html) instruction.
pub fn process_initialize_group(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: InitializeGroup,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let group_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;

    // check that the mint and group accounts are the same, since the group
    // extension should only describe itself
    if group_info.key != mint_info.key {
        msg!("Group configurations for a mint must be initialized in the mint itself.");
        return Err(TokenError::MintMismatch.into());
    }

    // scope the mint authority check, since the mint is in the same account!
    {
        // This check isn't really needed since we'll be writing into the account,
        // but auditors like it
        check_program_account(mint_info.owner)?;
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;

        if !mint_authority_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if mint.base.mint_authority.as_ref() != COption::Some(mint_authority_info.key) {
            return Err(TokenGroupError::IncorrectMintAuthority.into());
        }

        if mint.get_extension::<GroupPointer>().is_err() {
            msg!(
                "A mint with group configurations must have the group-pointer extension \
                 initialized"
            );
            return Err(TokenError::InvalidExtensionCombination.into());
        }
    }

    // Allocate a TLV entry for the space and write it in
    // Assumes that there's enough SOL for the new rent-exemption
    let group = TokenGroup::new(mint_info.key, data.update_authority, data.max_size.into());
    alloc_and_serialize::<Mint, TokenGroup>(group_info, &group, false)?;

    Ok(())
}

/// Processes an
/// [UpdateGroupMaxSize](enum.GroupInterfaceInstruction.html)
/// instruction
pub fn process_update_group_max_size(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: UpdateGroupMaxSize,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let group_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;

    let mut buffer = group_info.try_borrow_mut_data()?;
    let mut state = StateWithExtensionsMut::<Mint>::unpack(&mut buffer)?;
    let group = state.get_extension_mut::<TokenGroup>()?;

    check_update_authority(update_authority_info, &group.update_authority)?;

    group.update_max_size(data.max_size.into())?;

    Ok(())
}

/// Processes an
/// [UpdateGroupAuthority](enum.GroupInterfaceInstruction.html)
/// instruction
pub fn process_update_group_authority(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: UpdateGroupAuthority,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let group_info = next_account_info(account_info_iter)?;
    let update_authority_info = next_account_info(account_info_iter)?;

    let mut buffer = group_info.try_borrow_mut_data()?;
    let mut state = StateWithExtensionsMut::<Mint>::unpack(&mut buffer)?;
    let group = state.get_extension_mut::<TokenGroup>()?;

    check_update_authority(update_authority_info, &group.update_authority)?;

    group.update_authority = data.new_authority;

    Ok(())
}

/// Processes an [Instruction](enum.Instruction.html).
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: TokenGroupInstruction,
) -> ProgramResult {
    match instruction {
        TokenGroupInstruction::InitializeGroup(data) => {
            msg!("TokenGroupInstruction: InitializeGroup");
            process_initialize_group(program_id, accounts, data)
        }
        TokenGroupInstruction::UpdateGroupMaxSize(data) => {
            msg!("TokenGroupInstruction: UpdateGroupMaxSize");
            process_update_group_max_size(program_id, accounts, data)
        }
        TokenGroupInstruction::UpdateGroupAuthority(data) => {
            msg!("TokenGroupInstruction: UpdateGroupAuthority");
            process_update_group_authority(program_id, accounts, data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
