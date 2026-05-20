//! AlphaQ swap dispatcher. Plaintext 18-byte ix data, 12-account layout.
//!
//! AlphaQ is an oracle-priced PMM at program
//! `ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA`. Permissionless (no caller
//! whitelist), no XOR / scrambling, no on-chain probe needed (price lives in
//! the pool account at @424 as `price_q10`).
//!
//! Account layout (12 + 1 trailing program-id terminator):
//!   [0]  user wallet (signer + writable)
//!   [1]  pool_state (writable, 672B)
//!   [2]  pool_sibling_state (writable, 336B)
//!   [3]  user_src_ata (writable)
//!   [4]  user_dst_ata (writable)
//!   [5]  vault_a (writable)
//!   [6]  vault_b (writable)
//!   [7]  DUPLICATE of slot 5 (vault_a, writable)
//!   [8]  DUPLICATE of slot 6 (vault_b, writable)
//!   [9]  DUPLICATE of slot 6 (vault_b, writable)
//!   [10] SPL Token program (TokenkegQ…)
//!   [11] Sysvar Instructions
//!   [12] AlphaQ swap program (load_swap_accounts terminator)
//!
//! Data layout (18 bytes):
//!   [0]      selector = 0x0c
//!   [1]      direction (0 = B→A, 1 = A→B)
//!   [2..10]  amount_in u64 LE
//!   [10..18] min_amount_out u64 LE (= 0 — slippage enforced by cqX)
//!
//! 197 / 202 sampled successful AlphaQ callers pass min_amount_out = 0;
//! cqX's outer slippage assert covers correctness.

use super::swap::perform_swap;
use crate::constant::alphaq;
use crate::error::CqxErrorCode;
use anchor_lang::prelude::*;

/// Build the 18-byte AlphaQ swap-ix data. Plain byte layout, no scrambling.
fn build_alphaq_data(amount_in: u64, direction: u8) -> [u8; 18] {
    let mut data = [0u8; 18];
    data[0] = 0x0c;                                              // selector
    data[1] = direction;                                          // 0 = B→A, 1 = A→B
    data[2..10].copy_from_slice(&amount_in.to_le_bytes());        // amount_in
    // bytes 10..18 stay zero (min_amount_out = 0)
    data
}

pub fn alphaq_swap(
    remaining_accounts: &[AccountInfo],
    amount_in: u64,
    direction: u8,
    dst_idx: usize,
) -> Result<u64> {
    let program = &alphaq();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(last.key() == *program, CqxErrorCode::ProviderNotFound);
    require!(direction <= 1, CqxErrorCode::InvalidSwapRoute);
    require!(
        dst_idx < remaining_accounts.len() - 1,
        CqxErrorCode::InvalidSwapRoute
    );

    let data = build_alphaq_data(amount_in, direction);
    perform_swap(remaining_accounts, &data, program, dst_idx)
}
