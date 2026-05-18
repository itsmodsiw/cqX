//! HumidiFi swap dispatcher — addition over the dgenkm fork.
//!
//! HumidiFi differs from the other DEXes in two ways:
//!
//!   1. The 25-byte swap ix data is XOR-scrambled with a fixed key plus a
//!      per-chunk position mask. We unscramble client-side and re-scramble
//!      here on chain after stamping the live header (read from pool slot
//!      offset 0x240).
//!   2. There's no exposed IDL — layout was discovered via on-chain analysis
//!      (see `reference_humidifi_integration` memory). The 8-byte header at
//!      offset 0x240 is the spot-rate state HumidiFi's program consumes.
//!
//! Account layout (14 entries — `HUMIDIFI_MAX_ACCOUNT`):
//!   [0]  caller (signer + writable)
//!   [1]  HumidiFi pool (writable)            ← we read header from here
//!   [2]  base vault (writable)
//!   [3]  quote vault (writable)
//!   [4]  caller's base ATA (writable)
//!   [5]  caller's quote ATA (writable)
//!   [6]  Sysvar Clock
//!   [7]  SPL Token program
//!   [8]  SPL Token program (duplicate slot HumidiFi expects)
//!   [9]  Sysvar Instructions
//!   [10] base mint
//!   [11] quote mint
//!   [12] user_data PDA gate (HumidiFi-owned, data[0] must be 2)
//!   [13] referrer / Jito tip
//!   [14] HumidiFi swap program (load_swap_accounts appends this when
//!        verifying the trailing program-id)
//!
//! On bucket-router accounting: the caller passes `dst_idx` so this fn knows
//! which ATA's post-balance is the leg's "out" amount. With direction=0
//! (base→quote) it's the quote ATA at slot 5; direction=1 it's slot 4.

use super::swap::perform_swap;
use crate::constant::humidifi;
use crate::error::Cq2RouterErrorCode;
use anchor_lang::prelude::*;

/// 8-byte XOR key used by HumidiFi for swap-ix scrambling. Per-chunk it's
/// further XOR'd with a position mask (`pos_lo,pos_hi` repeated 4×).
const HUMIDIFI_KEY: [u8; 8] = [0x3a, 0xff, 0x2f, 0xff, 0xe2, 0xba, 0xeb, 0xc3];

/// In-place HumidiFi 25-byte ix-data scrambler. Operation is its own inverse.
fn humidifi_xor(buf: &mut [u8]) {
    let mut pos: u16 = 0;
    let mut i = 0;
    while i < buf.len() {
        let chunk = core::cmp::min(8, buf.len() - i);
        let p = pos.to_le_bytes();
        let pm = [p[0], p[1], p[0], p[1], p[0], p[1], p[0], p[1]];
        let mut j = 0;
        while j < chunk {
            buf[i + j] ^= HUMIDIFI_KEY[j] ^ pm[j];
            j += 1;
        }
        i += 8;
        pos += 1;
    }
}

/// Build the 25-byte HumidiFi inner-program ix data, scrambled.
/// Layout (pre-scramble): [header(8) | amount_in(8 LE) | direction(1) | padding(7) | terminator(0x14)]
fn build_humidifi_data(header: &[u8; 8], amount_in: u64, direction: u8) -> [u8; 25] {
    let mut data = [0u8; 25];
    data[0..8].copy_from_slice(header);
    data[8..16].copy_from_slice(&amount_in.to_le_bytes());
    data[16] = direction;
    data[24] = 0x14;
    humidifi_xor(&mut data);
    data
}

/// Pool header lives at offset 0x240 (576). Anything past that = end-of-data.
const POOL_HEADER_OFFSET: usize = 0x240;

pub fn humidifi_swap(
    remaining_accounts: &[AccountInfo],
    amount_in: u64,
    direction: u8,
    dst_idx: usize,
) -> Result<u64> {
    let program = &humidifi();
    let last = &remaining_accounts[remaining_accounts.len() - 1];
    require!(
        last.key() == *program,
        Cq2RouterErrorCode::ProviderNotFound
    );

    require!(direction <= 1, Cq2RouterErrorCode::InvalidSwapRoute);
    require!(
        dst_idx < remaining_accounts.len() - 1,
        Cq2RouterErrorCode::InvalidSwapRoute
    );

    // Read live pool header from pool slot[1].
    let header = {
        let pool_data = remaining_accounts[1].try_borrow_data()?;
        if pool_data.len() < POOL_HEADER_OFFSET + 8 {
            return Err(Cq2RouterErrorCode::HumidiFiPoolInvalid.into());
        }
        let mut h = [0u8; 8];
        h.copy_from_slice(&pool_data[POOL_HEADER_OFFSET..POOL_HEADER_OFFSET + 8]);
        h
    };

    let data_slice = build_humidifi_data(&header, amount_in, direction);

    // perform_swap reads dst_idx's pre-balance, CPIs, then reads post and
    // returns the delta. Same accounting cq2-router uses for every other DEX.
    perform_swap(remaining_accounts, &data_slice, program, dst_idx)
}
