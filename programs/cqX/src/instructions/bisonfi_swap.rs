//! BisonFi swap dispatcher. Plain 19-byte Borsh ix data.
//!
//! Layout reverse-engineered from on-chain BisonFi swap signatures
//! (e.g. tx 3VNctP4Nr8nSNqZG2dseJGf7NAbLkfBDaLmxCpuYiAtMvqFGYezpmBGeddd11rjgXveVuWrK6F9TdSR2x8v6K742):
//!
//! Account layout (9 + 1 trailing program-id terminator):
//!   [0]  caller (signer + writable)
//!   [1]  pool / market (writable)
//!   [2]  base vault (writable)
//!   [3]  quote vault (writable)
//!   [4]  caller's source ATA (writable)
//!   [5]  caller's destination ATA (writable)
//!   [6]  SPL Token program
//!   [7]  SPL Token program (duplicate)
//!   [8]  trailing wallet (referrer / system-owned tip account)
//!   [9]  BisonFi swap program (load_swap_accounts terminator)
//!
//! Data layout (19 bytes):
//!   [0]      selector = 0x07
//!   [1..9]   amount_in (u64 LE)
//!   [9..17]  min_amount_out (u64 LE, = 0 — slippage enforced by cqX)
//!   [17]     direction-tag byte (matches observed 0x01)
//!   [18]     trailing flag (0x00 in observed swaps)

use super::swap::perform_swap;
use crate::constant::bisonfi;
use crate::error::CqxErrorCode;
use anchor_lang::prelude::*;

/// Build the 19-byte BisonFi swap-ix data (plain Borsh, no scrambling).
/// Real on-chain swap example data:
///   `0790c211050000000000000000000000000100`
/// where bytes 1..9 = amount_in, 9..17 = min_out (= 0), 17 = 0x01, 18 = 0x00.
///
/// 2026-05-09 RE: byte 17 is the swap-direction byte AND BisonFi's convention
/// is INVERTED from cq2's. cq2's caller emits `direction = 0` for base→quote
/// (SOL→USDC) but BisonFi's canonical direct swap on the same pool has
/// byte 17 = 0x01 for SOL→USDC. We flip the bit on the way down. Discovered
/// after a 0/5 BisonFi landing run failed with MissingRequiredSignature where
/// byte 17 was the only diff vs canonical.
fn build_bisonfi_data(amount_in: u64, direction: u8) -> [u8; 19] {
    let mut data = [0u8; 19];
    data[0] = 0x07;                                              // selector
    data[1..9].copy_from_slice(&amount_in.to_le_bytes());        // amount_in
    // bytes 9..17 stay zero (min_amount_out)
    data[17] = if direction == 0 { 1 } else { 0 };               // BisonFi-inverted direction
    // byte 18 stays 0x00 (matches every observed BisonFi swap on chain)
    data
}

pub fn bisonfi_swap(
    remaining_accounts: &[AccountInfo],
    amount_in: u64,
    direction: u8,
    dst_idx: usize,
) -> Result<u64> {
    let program = &bisonfi();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(last.key() == *program, CqxErrorCode::ProviderNotFound);
    require!(direction <= 1, CqxErrorCode::InvalidSwapRoute);
    require!(dst_idx < remaining_accounts.len() - 1, CqxErrorCode::InvalidSwapRoute);

    let data = build_bisonfi_data(amount_in, direction);
    perform_swap(remaining_accounts, &data, program, dst_idx)
}
