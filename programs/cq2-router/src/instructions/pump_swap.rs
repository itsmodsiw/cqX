use super::swap::perform_swap;
use crate::constant::pump_amm;
use crate::constant::{BUY_EXACT_QUOTE_IN_SELECTOR, SELL_SELECTOR};
use crate::error::Cq2RouterErrorCode;
use anchor_lang::prelude::*;

// PumpAmm `buy_exact_quote_in` ix body.
//
//   args = disc(8) + spendable_quote_in:u64 + min_base_amount_out:u64
//   total = 24 bytes
//
// Matches the canonical on-chain layout used by every production
// `buy_exact_quote_in` call (verified across 5 mainnet samples — all 24 bytes,
// no trailing `track_volume` byte). The PumpAmm Anchor IDL describes a
// trailing `track_volume: OptionBool` field but the on-chain program accepts
// the short body so cq2-router omits it (matches sell layout for symmetry).
//
// `min_base_amount_out = 1`: per-leg slippage gate is intentionally permissive;
// the cq2-router's bucket-level `slippage_bps` check at the end of the route
// is the redline that protects user funds.
pub fn pump_fun_buy_exact_in_swap_data(quote_amount_in: u64) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(24);
    data.extend_from_slice(BUY_EXACT_QUOTE_IN_SELECTOR);
    data.extend_from_slice(&quote_amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());
    data
}

// PumpAmm `sell` ix body.
//
//   args = disc(8) + base_amount_in:u64 + min_quote_amount_out:u64
//   total = 24 bytes
//
// `min_quote_amount_out = 1`: same rationale as buy — bucket-level slippage
// is the user-funds gate; per-leg min is permissive.
pub fn pump_fun_sell_swap_data(base_amount_in: u64) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(24);
    data.extend_from_slice(SELL_SELECTOR);
    data.extend_from_slice(&base_amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());
    data
}

pub fn pump_amm_buy_exact_in_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &pump_amm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        Cq2RouterErrorCode::ProviderNotFound
    );
    let data_slice = pump_fun_buy_exact_in_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}

pub fn pump_amm_sell_swap(
    remaining_accounts: &[AccountInfo],
    amount: u64,
    account_index: usize,
) -> Result<u64> {
    let program = &pump_amm();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        Cq2RouterErrorCode::ProviderNotFound
    );
    let data_slice = pump_fun_sell_swap_data(amount);
    perform_swap(remaining_accounts, data_slice.as_slice(), program, account_index)
}
