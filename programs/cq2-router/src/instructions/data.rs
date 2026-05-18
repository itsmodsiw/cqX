use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;

use crate::constant::{
    ARGS_CLMM_LEN, ARGS_CPMM_LEN, ARGS_DLMMV2_LEN, ARGS_LEN, ARGS_V2_LEN, CPSWAP_SELECTOR,
    SWAP_SELECTOR, SWAP_V2_SELECTOR, SWAP_V2_SELECTOR_DLMM,
};
use crate::util::{AccountsType, RemainingAccountsInfo, RemainingAccountsSlice};

pub fn raydium_swap_data(amount_in: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_LEN);
    data.push(9);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());
    data
}

pub fn raydium_v2_swap_data(amount_in: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_LEN);
    data.push(16);
    data.extend_from_slice(&amount_in.to_be_bytes());
    data.extend_from_slice(&1u64.to_be_bytes());
    data
}

pub fn raydium_clmm_swap_data(
    amount_in: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
    other_amount_threshold: u64,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_CLMM_LEN);
    data.extend_from_slice(SWAP_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&other_amount_threshold.to_le_bytes());
    data.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    data.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    data
}

pub fn raydium_clmm_swap_v2_data(
    amount_in: u64,
    sqrt_price_limit_x64: u128,
    is_base_input: bool,
    other_amount_threshold: u64,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_CLMM_LEN);
    data.extend_from_slice(SWAP_V2_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&other_amount_threshold.to_le_bytes());
    data.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    data.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    data
}

pub fn raydium_cpmm_swap_data(amount_in: u64) -> Vec<u8> {
    let minimum_amount_out = 0u64;
    let mut data = Vec::with_capacity(ARGS_CPMM_LEN);
    data.extend_from_slice(CPSWAP_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&minimum_amount_out.to_le_bytes());
    data
}

pub fn meteora_swap_data(amount_in: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_LEN);
    data.extend_from_slice(SWAP_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());
    data
}

pub fn meteora_dlmm_swap_data(amount_in: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_LEN);
    data.extend_from_slice(SWAP_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());
    data
}

pub fn meteora_dlmm_swap_v2_data(amount_in: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_DLMMV2_LEN);
    data.extend_from_slice(SWAP_V2_SELECTOR_DLMM);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&1u64.to_le_bytes());

    let accounts_info = RemainingAccountsInfo {
        slices: vec![
            RemainingAccountsSlice {
                accounts_type: AccountsType::TransferHookA,
                length: 0,
            },
            RemainingAccountsSlice {
                accounts_type: AccountsType::TransferHookB,
                length: 0,
            },
        ],
    };

    data.extend_from_slice(&accounts_info.try_to_vec().unwrap());
    data
}

pub fn whirlpool_swap_v2_data(a_to_b: bool, amount_in: u64, sqrt_price_limit: u128) -> Vec<u8> {
    let amount_specified_is_input = true;
    let other_amount_threshold = 1u64;

    let mut data = Vec::with_capacity(ARGS_V2_LEN);
    data.extend_from_slice(SWAP_V2_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&other_amount_threshold.to_le_bytes());
    data.extend_from_slice(&sqrt_price_limit.to_le_bytes());
    data.extend_from_slice(&(amount_specified_is_input as u8).to_le_bytes());
    data.extend_from_slice(&(a_to_b as u8).to_le_bytes());
    data.extend_from_slice(&(0u8).to_le_bytes());
    data
}

pub fn whirlpool_swap_data(a_to_b: bool, amount_in: u64, sqrt_price_limit: u128) -> Vec<u8> {
    let amount_specified_is_input = true;
    let other_amount_threshold = 1u64;

    let mut data = Vec::with_capacity(ARGS_LEN);
    data.extend_from_slice(SWAP_SELECTOR);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&other_amount_threshold.to_le_bytes());
    data.extend_from_slice(&sqrt_price_limit.to_le_bytes());
    data.extend_from_slice(&(amount_specified_is_input as u8).to_le_bytes());
    data.extend_from_slice(&(a_to_b as u8).to_le_bytes());
    data
}

pub fn orca_v1_and_v2_swap_data(amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(ARGS_LEN);
    data.push(1);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&min_amount_out.to_le_bytes());
    data
}
