use crate::constant::{
    meteora, meteora_damm, meteora_dlmm, orca_v1, orca_v2, raydium, raydium_clmm, raydium_cp,
    raydium_stable, whirlpool,
};
use crate::error::CqxErrorCode;
use crate::instructions::data::*;
use crate::util::RemainingAccountsInfo;
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke, pubkey::Pubkey},
};
use anchor_spl::token_interface::TokenAccount;

fn get_token_account_amount(remaining_accounts: &[AccountInfo], i: usize) -> Result<u64> {
    let account_data = remaining_accounts[i].try_borrow_data()?;
    let mut data_slice: &[u8] = &account_data;
    let token_account = TokenAccount::try_deserialize(&mut data_slice)?;
    Ok(token_account.amount)
}

fn prepare_accounts(remaining_accounts: &[AccountInfo]) -> Vec<AccountMeta> {
    remaining_accounts
        .iter()
        .take(remaining_accounts.len().saturating_sub(1))
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect()
}

fn swap(program: &Pubkey, remaining_accounts: &[AccountInfo], data: &[u8]) -> Result<()> {
    let accounts = prepare_accounts(remaining_accounts);

    invoke(
        &Instruction {
            program_id: *program,
            accounts,
            data: data.to_vec(),
        },
        remaining_accounts,
    )?;

    Ok(())
}

pub fn perform_swap(
    remaining_accounts: &[AccountInfo],
    data: &[u8],
    program: &Pubkey,
    account_index: usize,
) -> Result<u64> {
    let before_swap_amount = get_token_account_amount(remaining_accounts, account_index)?;
    swap(program, remaining_accounts, data)?;
    let after_swap_amount = get_token_account_amount(remaining_accounts, account_index)?;

    let actual_amount = after_swap_amount
        .checked_sub(before_swap_amount)
        .ok_or(CqxErrorCode::SwapUnderflow)?;

    Ok(actual_amount)
}

pub fn raydium_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn raydium_v2_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_v2_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn raydium_stable_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium_stable();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn raydium_clmm_swap(
    is_base_input: bool,
    sqrt_price_limit: u128,
    other_amount_threshold: u64,
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium_clmm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_clmm_swap_data(
        amount,
        sqrt_price_limit,
        is_base_input,
        other_amount_threshold,
    );
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn raydium_clmm_swap_v2(
    is_base_input: bool,
    sqrt_price_limit: u128,
    other_amount_threshold: u64,
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium_clmm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_clmm_swap_v2_data(
        amount,
        sqrt_price_limit,
        is_base_input,
        other_amount_threshold,
    );
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn raydium_cp_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &raydium_cp();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = raydium_cpmm_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn meteora_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &meteora();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = meteora_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn meteora_dlmm_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &meteora_dlmm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = meteora_dlmm_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn meteora_dlmm_swap_v2(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &meteora_dlmm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = meteora_dlmm_swap_v2_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn meteora_damm_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &meteora_damm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = meteora_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn whirlpool_swap_v2(
    a_to_b: bool,
    _remaining_accounts_info: &Option<RemainingAccountsInfo>,
    sqrt_price_limit: u128,
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &whirlpool();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = whirlpool_swap_v2_data(a_to_b, amount, sqrt_price_limit);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn whirlpool_swap(
    a_to_b: bool,
    sqrt_price_limit: u128,
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &whirlpool();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = whirlpool_swap_data(a_to_b, amount, sqrt_price_limit);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn orca_v1_and_v2_swap(
    remaining_accounts: &[AccountInfo],
    amount_in: u64,
    account_index: usize,
    is_orca_v2: bool,
) -> Result<u64> {
    let program = if is_orca_v2 { orca_v2() } else { orca_v1() };
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == program,
        CqxErrorCode::ProviderNotFound
    );
    let data_slice = orca_v1_and_v2_swap_data(amount_in, 1);
    perform_swap(remaining_accounts, data_slice.as_slice(), &program, account_index)
}
