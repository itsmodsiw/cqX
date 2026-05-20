//! GoonFi V2 swap dispatcher. 19-byte Borsh ix data.
//!
//! Account layout (13 + 1 trailing program-id terminator):
//!   [0]  caller (signer + writable)
//!   [1]  pool / market (writable)
//!   [2]  caller's base ATA (writable)
//!   [3]  caller's quote ATA (writable)
//!   [4]  base vault (writable)
//!   [5]  quote vault (writable)
//!   [6]  base mint
//!   [7]  quote mint
//!   [8]  blacklist PDA (per-pool)
//!   [9]  global config PDA `BNrK9LpEn65QA4TyBLVSMdngW3XHj3xLfFPwGdCBv8wV`
//!   [10] Sysvar Instructions
//!   [11] SPL Token program
//!   [12] SPL Token program (duplicate)
//!   [13] GoonFi swap program (load_swap_accounts terminator)

use super::swap::perform_swap;
use crate::constant::goonfi_v2;
use crate::error::CqxErrorCode;
use anchor_lang::prelude::*;

/// Build the 19-byte GoonFi V2 swap-ix data.
///
/// Layout (re-RE'd 2026-05-09 against 20+ live mainnet swaps for
/// pool `GMCJv…` SOL/USDC; bytes 1+amount-offset cross-validated with
/// pre/post token-balance deltas, all matched in 4/4 sampled txns):
///   [0]      = 0x01 selector
///   [1]      = is_user_bid (1 = quote→base, 0 = base→quote)
///   [2..10]  = amount_in u64 LE     ← amount_in offset is 2, NOT 3
///   [10..18] = min_out u64 LE       ← min_out offset is 10, NOT 11
///   [18]     = trailing flag (varies 0|1 across observed swaps; not a
///              per-pool bump — the value differs across swaps for the
///              same pool, so it's not a PDA bump byte; left at 0).
///
/// The earlier 2026-05-08 RE incorrectly placed amount_in at [3..11] and
/// inserted a "bump" byte at [2]; that produced Custom 1 errors because
/// the on-chain GoonFi V2 program reads amount_in at [2..10] and then
/// tried to transfer ~256× the intended size (amount<<8) from a wallet
/// with insufficient funds. The `bump` field on `SwapType::GoonfiV2`
/// is preserved (Borsh-wire-compat with the deployed dispatcher) but
/// ignored here.
fn build_goonfi_data(is_user_bid: bool, _bump: u8, amount_in: u64) -> [u8; 19] {
    let mut data = [0u8; 19];
    data[0] = 0x01;
    data[1] = is_user_bid as u8;
    data[2..10].copy_from_slice(&amount_in.to_le_bytes());
    // bytes 10..18 stay zero (min_amount_out = 0)
    // byte 18 stays zero (trailing flag — the on-chain program tolerates
    // 0 here based on observed sample swaps with byte18 == 0)
    data
}

pub fn goonfi_v2_swap(
    remaining_accounts: &[AccountInfo],
    amount_in: u64,
    is_user_bid: bool,
    bump: u8,
    dst_idx: usize,
) -> Result<u64> {
    let program = &goonfi_v2();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(last.key() == *program, CqxErrorCode::ProviderNotFound);
    require!(dst_idx < remaining_accounts.len() - 1, CqxErrorCode::InvalidSwapRoute);

    let data = build_goonfi_data(is_user_bid, bump, amount_in);
    perform_swap(remaining_accounts, &data, program, dst_idx)
}
