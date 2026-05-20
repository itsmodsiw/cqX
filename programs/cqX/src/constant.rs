use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

/// Maximum integrator fee in basis points. 500 = 5% (operator-set ceiling
/// 2026-05-14). The on-chain `swap` ix `require!`s `fee_bps <= MAX_FEE_BPS`
/// so even a misconfigured api-http can't pass through a higher value.
pub const MAX_FEE_BPS: u16 = 500;

pub const ARGS_LEN: usize = 17;
pub const ARGS_CLMM_LEN: usize = 41;
pub const ARGS_CPMM_LEN: usize = 24;
pub const ARGS_DLMMV2_LEN: usize = 33;
pub const ARGS_V2_LEN: usize = 43;
pub const SWAP_V2_SELECTOR: &[u8; 8] = &[43, 4, 237, 11, 26, 201, 30, 98];
pub const CPSWAP_SELECTOR: &[u8; 8] = &[143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_SELECTOR: &[u8; 8] = &[248, 198, 158, 145, 225, 117, 135, 200];
pub const SWAP_V2_SELECTOR_DLMM: &[u8; 8] = &[65, 75, 63, 76, 235, 91, 91, 136];
// PumpAmm `buy_exact_quote_in` discriminator — Anchor sha256("global:buy_exact_quote_in")[..8].
// Verified against the on-chain Anchor IDL at PDA
//   `find_program_address(&[b"anchor:idl", pAMMBay6...as_ref()], pAMMBay6...)`
// AND against 5 real on-chain `buy_exact_quote_in` ix bodies fetched via
// QuickNode (e.g. tx `huQxt4zqfEcBsevY9hAY4XZwXShXmedBdE6NgWtwbv7oSRXRme5rCYLiMFF7KEqFt3eHKDji24UEY3NYNFpP3kV`).
// Args layout in production: `disc(8) + spendable_quote_in:u64 + min_base_amount_out:u64 = 24 bytes`.
// (IDL declares a trailing `track_volume: OptionBool` but real callers omit it
// and the on-chain program accepts the short body; so cqX does too.)
pub const BUY_EXACT_QUOTE_IN_SELECTOR: &[u8; 8] = &[198, 46, 21, 82, 180, 217, 232, 112];
// PumpAmm `sell` discriminator — Anchor sha256("global:sell")[..8]. Already correct.
// Args layout: `disc(8) + base_amount_in:u64 + min_quote_amount_out:u64 = 24 bytes`.
pub const SELL_SELECTOR: &[u8; 8] = &[51, 230, 133, 164, 1, 127, 131, 173];

pub const WHIRLPOOL_V2_INDEX_A_TO_B: usize = 9;
pub const WHIRLPOOL_V2_INDEX_B_TO_A: usize = 7;
pub const WHIRLPOOL_INDEX_A_TO_B: usize = 5;
pub const WHIRLPOOL_INDEX_B_TO_A: usize = 3;
pub const RAYDIUM_INDEX: usize = 16;
pub const RAYDIUM_V2_INDEX: usize = 6;
pub const RAYDIUMCPMM_INDEX: usize = 5;
pub const RAYDIUMCLMM_V2_INDEX: usize = 4;
pub const RAYDIUMCLMM_INDEX: usize = 4;
pub const METEORA_INDEX: usize = 2;
pub const METEORADLMM_INDEX: usize = 5;

pub const PUMP_AMM_BUY_EXACT_INDEX: usize = 5;
pub const PUMP_AMM_SELL_INDEX: usize = 6;

pub const ORCA_V1_AND_V2_USER_DESTINATION_ACCOUNT_INDEX: usize = 6;

pub const METEORADAMMV2_INDEX: usize = 3;
pub const WHIRLPOOL_V2_MAX_ACCOUNT: usize = 16;
pub const WHIRLPOOL_MAX_ACCOUNT: usize = 12;

pub const RAYDIUM_MAX_ACCOUNT: usize = 19;
pub const RAYDIUM_V2_MAX_ACCOUNT: usize = 9;
pub const RAYDIUM_CPMM_MAX_ACCOUNT: usize = 14;
pub const RAYDIUM_CLMM_V2_MAX_ACCOUNT: usize = 17;
pub const RAYDIUM_CLMM_MAX_ACCOUNT: usize = 10;
// PumpAmm fixed_len for `load_swap_accounts` = IDL_accounts + 3 trailing fee
// accounts (pool_v2, recipient, recipient_ata — added by 2026-04-28
// BREAKING_FEE_RECIPIENT.md upgrade) + 1 program_id terminator.
//
// PRE-2026-04-28 (broken on chain after the upgrade):
//   BUY:  IDL=23  +1 terminator = 24
//   SELL: IDL=21  +1 terminator = 22
//
// POST-2026-04-28 (correct):
//   BUY:  IDL=23 + 3 trailing + 1 terminator = 27
//   SELL: IDL=21 + 3 trailing + 1 terminator = 25
//
// Why the bump matters: `extract_accounts` (in
// `util/extract_accounts.rs`) only INCLUDES accounts whose owner == pump_amm
// or that ARE the program_id terminator. The 3 trailing accounts are owned
// by:
//   pool_v2          → system (uninitialized PDA) — SKIPPED by extract
//   recipient        → pfeeUxB6 (pump_fees)        — SKIPPED by extract
//   recipient_ata    → Tokenkeg                    — SKIPPED by extract
// So with fixed_len=24, the 3 trailing slots are filtered out and pump-amm
// rejects with `BuybackFeeRecipientMissing (6058)` at swap/buy.rs:118.
// Bumping fixed_len to 27 / 25 makes load_swap_accounts take the full
// trailing block as-is, no ownership filtering needed.
//
// W9-B 2026-04-30 — root-caused via mainnet sim diff (transcript_2026-04-30_w9b_pumpamm_golive.md).
pub const PUMP_AMM_BUY_EXACT_IN_ACCOUNT: usize = 27;
pub const PUMP_AMM_SELL_ACCOUNT: usize = 25;

pub const METEORA_MAX_ACCOUNT: usize = 16;
pub const METEORA_DLMM_MAX_ACCOUNT: usize = 18;
pub const METEORA_DLMM_V2_MAX_ACCOUNT: usize = 17;
pub const METEORA_DAMM_V2_MAX_ACCOUNT: usize = 15;

pub const ORCA_V1_MAX_ACCOUNT: usize = 10;
pub const ORCA_V2_MAX_ACCOUNT: usize = 10;

/// HumidiFi: 14 accounts (matches build_txn_humidifi.rs::cqx_humidifi_swap_ix
/// which loads 15 — one of those is the inner program slot which load_swap_accounts
/// appends as the trailing program-id check). So fixed_len = 14, the load helper
/// will tag the program as the trailing slot.
pub const HUMIDIFI_MAX_ACCOUNT: usize = 14;

pub fn whirlpool() -> Pubkey {
    Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap()
}

pub fn raydium() -> Pubkey {
    Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap()
}

pub fn raydium_clmm() -> Pubkey {
    Pubkey::from_str("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK").unwrap()
}

pub fn raydium_cp() -> Pubkey {
    Pubkey::from_str("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C").unwrap()
}

pub fn raydium_stable() -> Pubkey {
    Pubkey::from_str("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h").unwrap()
}

pub fn meteora() -> Pubkey {
    Pubkey::from_str("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB").unwrap()
}

pub fn meteora_dlmm() -> Pubkey {
    Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap()
}

pub fn meteora_damm() -> Pubkey {
    Pubkey::from_str("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG").unwrap()
}

pub fn orca_v1() -> Pubkey {
    Pubkey::from_str("DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1").unwrap()
}

pub fn orca_v2() -> Pubkey {
    Pubkey::from_str("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP").unwrap()
}

pub fn orca_whirlpool() -> Pubkey {
    Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap()
}

pub fn pump_amm() -> Pubkey {
    Pubkey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA").unwrap()
}

pub fn humidifi() -> Pubkey {
    Pubkey::from_str("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp").unwrap()
}

/// BisonFi direct swap layout (9 entries + 1 trailing program-id terminator).
/// Selector 0x07, 19-byte data. RE'd from canonical mainnet swap
/// 3VNctP4Nr8nSNqZG2dseJGf7NAbLkfBDaLmxCpuYiAtMvqFGYezpmBGeddd11rjgXveVuWrK6F9TdSR2x8v6K742
/// on 2026-05-09. The 10-account Jupiter-CPI form (selector 0x02) is a
/// different variant that cq2 does not invoke.
pub const BISONFI_MAX_ACCOUNT: usize = 9;

/// GoonFi V2: 13-account swap layout (13 entries + 1 trailing program-id terminator).
/// Includes per-pool blacklist PDA at slot 8 and the global config PDA at slot 9.
pub const GOONFI_V2_MAX_ACCOUNT: usize = 13;

/// AlphaQ: 12-account swap layout (12 entries + 1 trailing program-id terminator).
/// 18-byte plaintext ix data. No probe — price_q10 lives in pool@424.
pub const ALPHAQ_MAX_ACCOUNT: usize = 12;

pub fn bisonfi() -> Pubkey {
    Pubkey::from_str("BiSoNHVpsVZW2F7rx2eQ59yQwKxzU5NvBcmKshCSUypi").unwrap()
}

pub fn goonfi_v2() -> Pubkey {
    Pubkey::from_str("goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE").unwrap()
}

pub fn alphaq() -> Pubkey {
    Pubkey::from_str("ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA").unwrap()
}
