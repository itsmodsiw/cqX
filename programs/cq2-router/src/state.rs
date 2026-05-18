use crate::util::RemainingAccountsInfo;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RoutePlanStep {
    pub swap: SwapType,
    pub percent: u16,     // 0..=100 = %, >100 = BPS
    pub input_index: u8,
    pub output_index: u8,
    pub is_last_percent: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct BucketElement {
    pub amount: u128,
    pub use_amount: u128,
}

/// On-chain swap-variant enum. Borsh ordinals MUST match what off-chain
/// `instruction_api::quote_parser::SwapType` emits — these are the wire
/// format of `SwapArgs.route_plan`. Order = enum discriminant.
///
/// HumidiFi was added to cq2-router (not present in dgenkm). Direction:
/// 0 = base→quote, 1 = quote→base. dst_idx is the account-list slot whose
/// post-balance is delta-checked for the slippage assert (if our caller
/// wants min-out enforcement; the bucket router already enforces final
/// `slippage_bps` so this is just for in-leg sanity).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum SwapType {
    Raydium {},
    RaydiumV2 {},
    RaydiumCp {},
    RaydiumClmm {
        is_base_input: bool,
        sqrt_price_limit: u128,
        other_amount_threshold: u64,
    },
    RaydiumClmmV2 {
        is_base_input: bool,
        sqrt_price_limit: u128,
        other_amount_threshold: u64,
    },
    RaydiumStable {},
    Meteora {},
    MeteoraDlmm {},
    MeteoraDlmmV2 {},
    MeteoraDammV2 {},
    WhirlpoolS2 {
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    OrcaV1S {},
    OrcaV2S {},
    PumpAmmBuyExactQuoteIn {},
    PumpAmmSell {},
    /// HumidiFi swap. Reads pool header from the pool account on-chain,
    /// scrambles the 25-byte swap ix-data with HumidiFi's XOR key, CPIs
    /// to the HumidiFi swap program. Direction: 0 = base→quote, 1 = quote→base.
    HumidiFi {
        direction: u8,
        dst_idx: u8,
    },
    /// BisonFi swap. Plain Borsh swap-ix data; pool layout has POOLSTAT discriminator.
    /// Direction: 0 = base→quote, 1 = quote→base.
    BisonFi {
        direction: u8,
        dst_idx: u8,
    },
    /// GoonFi V2 swap. Borsh-shaped data: is_user_bid + bump + amount_in + min_out.
    /// is_user_bid: false = base→quote, true = quote→base. bump = blacklist PDA bump.
    GoonfiV2 {
        is_user_bid: bool,
        bump: u8,
        dst_idx: u8,
    },
    /// AlphaQ swap. 18-byte plaintext data (selector 0x0c, direction byte,
    /// amount_in u64 LE, min_amount_out u64 LE = 0). 12-account layout
    /// (no probe daemon — `price_q10` lives in the pool account at @424).
    /// Direction: 0 = B→A (quote→base), 1 = A→B (base→quote).
    /// `dst_idx` = 4 (user_dst_ata) regardless of direction (the swap-ix
    /// already names src/dst by direction).
    AlphaQ {
        direction: u8,
        dst_idx: u8,
    },
}
