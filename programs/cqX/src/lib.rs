//! cqX — on-chain dispatcher for every DEX cq2 routes through.
//!
//! Forked from the cq2-owned dgenkm router (program-id `Dgenkm…`) and
//! redeployed at `cq2Xj7Sg…`. Adds HumidiFi as a new `SwapType` variant on
//! top of the original DEX fleet so a single `SwapArgs` Borsh blob can
//! address every leg — Raydium AMM/V2/CP/CLMM/CLMM-V2/Stable, Meteora
//! damm/dlmm/dlmm-v2/damm-v2, Orca v1/v2/Whirlpool, Pump AMM Buy/Sell,
//! HumidiFi — under one program.
//!
//! WSOL handling: a temp WSOL token account is created via PDA seeds
//! `[b"cq2wsol", user_pubkey]` (renamed from the original `b"spdrwsol"`
//! for cq2 ownership). The off-chain `instruction_api::ata_util::wsol`
//! must derive with the same seeds; both flip together.

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program_pack::Pack, system_instruction},
};
use anchor_spl::token::{self, spl_token, CloseAccount};

use constant::*;
use context::*;
use instructions::*;
use util::*;

mod constant;
mod context;
mod error;
mod instructions;
mod state;
mod util;

use crate::error::CqxErrorCode;
use crate::state::RoutePlanStep;
use crate::state::SwapType;

declare_id!("cqXSJi3MdPgPfxKfe86nxrrq6speZP4LTEJ2WUXoSQF");

// solana-security-txt — appears on Solscan / explorers as a green badge
// once this program is redeployed. Per https://github.com/neodyme-labs/solana-security-txt.
#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "cqX",
    project_url: "https://blog.carbium.io",
    contacts: "email:modsiw@carbium.io",
    policy: "https://carbium.io",
    preferred_languages: "en",
    source_code: "https://github.com/itsmodsiw/cqX",
    source_revision: "0.2.0",
    source_release: "0.2.0",
    auditors: "internal review only, no third party audit yet, PRs welcome",
    acknowledgements: "neodyme-labs/solana-security-txt for the badge format."
}

#[program]
pub mod cqx {
    use super::*;

    pub fn swap<'info>(
        ctx: Context<'_, '_, 'info, 'info, Swap<'info>>,
        in_amount: u64,
        quoted_out_amount: u64,
        slippage_bps: u16,
        route_plan: Vec<RoutePlanStep>,
        wsol_account_used: bool,
        wsol_is_start_input: bool,
        // 2026-05-14: integrator fee on output. 0 = no fee, no extra accounts.
        // When fee_bps > 0, api-http appends the fee destination as the LAST
        // entry in remaining_accounts:
        //   - SPL output  → treasury_ata (user_dst_ata → treasury_ata token::transfer)
        //   - SOL output  → treasury_wallet (post-close system::transfer)
        // For SPL output we also need user_dst_ata at the SECOND-TO-LAST slot.
        // Capped at MAX_FEE_BPS (500 = 5%) on-chain via require!.
        fee_bps: u16,
    ) -> Result<()> {
        require!(fee_bps <= MAX_FEE_BPS, CqxErrorCode::InvalidFeeBps);
        let mut i: usize = 0;
        let mut amount = in_amount;

        require!(
            amount > 0,
            CqxErrorCode::AmountInMustBeGreaterThanZero
        );
        require!(
            slippage_bps <= 10_000,
            CqxErrorCode::InvalidSlippageBps
        );

        const USER_INDEX: usize = 0;
        const TEMP_WSOL_INDEX: usize = 1;
        const WSOL_MINT_INDEX: usize = 2;
        const TOKEN_PROGRAM_INDEX: usize = 3;
        const SYSTEM_PROGRAM_INDEX: usize = 4;

        let remaining = &ctx.remaining_accounts;

        if wsol_account_used {
            require!(
                remaining.len() > SYSTEM_PROGRAM_INDEX,
                CqxErrorCode::InvalidSwapRoute
            );

            let user_ai = remaining[USER_INDEX].clone();
            let temp_wsol_ai = remaining[TEMP_WSOL_INDEX].clone();
            let wsol_mint_ai = remaining[WSOL_MINT_INDEX].clone();
            let token_program_ai = remaining[TOKEN_PROGRAM_INDEX].clone();

            // Must be the native mint
            require!(
                wsol_mint_ai.key() == spl_token::native_mint::ID,
                CqxErrorCode::InvalidSwapRoute
            );

            // Derive the expected WSOL PDA: seeds = ["cq2wsol", user]
            let (expected_pda, bump) =
                Pubkey::find_program_address(&[b"cq2wsol", user_ai.key.as_ref()], ctx.program_id);

            require!(
                expected_pda == *temp_wsol_ai.key,
                CqxErrorCode::InvalidSwapRoute
            );

            let signer_seeds = &[b"cq2wsol", user_ai.key.as_ref(), &[bump][..]];

            // create/init WSOL PDA account
            if temp_wsol_ai.data_is_empty() {
                let rent = Rent::get()?;
                let rent_lamports = rent.minimum_balance(spl_token::state::Account::LEN);

                let total_lamports = if wsol_is_start_input {
                    rent_lamports
                        .checked_add(in_amount)
                        .ok_or(CqxErrorCode::MathOverflow)?
                } else {
                    rent_lamports
                };

                let create_ix = system_instruction::create_account(
                    user_ai.key,
                    temp_wsol_ai.key,
                    total_lamports,
                    spl_token::state::Account::LEN as u64,
                    token_program_ai.key,
                );

                invoke_signed(
                    &create_ix,
                    &[user_ai.clone(), temp_wsol_ai.clone()],
                    &[signer_seeds],
                )?;

                token::initialize_account3(CpiContext::new(
                    token_program_ai.clone(),
                    token::InitializeAccount3 {
                        account: temp_wsol_ai.clone(),
                        mint: wsol_mint_ai.clone(),
                        authority: user_ai.clone(),
                    },
                ))?;
            }

            i = SYSTEM_PROGRAM_INDEX + 1;
        } else {
            i = 0;
        }

        // ---------------------- bucket router logic ----------------------
        let mut buckets: Vec<u128> = vec![0; 8];
        buckets[0] = in_amount as u128;

        let mut base: Vec<u128> = vec![0; 8];
        let mut used: Vec<u128> = vec![0; 8];

        let mut last_out_idx: usize = 0;

        for (_step_idx, step) in route_plan.iter().enumerate() {
            let in_idx = step.input_index as usize;
            let out_idx = step.output_index as usize;

            if in_idx >= buckets.len() {
                let new_len = in_idx + 1;
                buckets.resize(new_len, 0);
                base.resize(new_len, 0);
                used.resize(new_len, 0);
            }
            if out_idx >= buckets.len() {
                let new_len = out_idx + 1;
                buckets.resize(new_len, 0);
                base.resize(new_len, 0);
                used.resize(new_len, 0);
            }

            let src_bal_u128 = buckets[in_idx];

            if base[in_idx] == 0 {
                base[in_idx] = src_bal_u128;
            }
            let original = base[in_idx];

            if original == 0 {
                continue;
            }

            let percent_val: u16 = step.percent;
            require!(
                percent_val > 0 && percent_val <= 10_000,
                CqxErrorCode::InvalidPercentBps
            );

            let alloc_u128: u128 = if step.is_last_percent {
                let already_used = used[in_idx];
                let remaining_amt = original
                    .checked_sub(already_used)
                    .ok_or(CqxErrorCode::MathUnderflow)?;
                if remaining_amt == 0 {
                    continue;
                }
                remaining_amt
            } else {
                let pct = percent_val as u128;
                let denom: u128 = if percent_val <= 100 { 100 } else { 10_000 };

                let num = original
                    .checked_mul(pct)
                    .ok_or(CqxErrorCode::MathOverflow)?;
                let v = num / denom;
                if v == 0 {
                    continue;
                }
                v
            };

            require!(
                alloc_u128 <= u64::MAX as u128,
                CqxErrorCode::MathOverflow
            );
            let alloc: u64 = alloc_u128 as u64;

            used[in_idx] = used[in_idx]
                .checked_add(alloc_u128)
                .ok_or(CqxErrorCode::MathOverflow)?;

            let out_amt: u64 = match &step.swap {
                SwapType::PumpAmmBuyExactQuoteIn {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        PUMP_AMM_BUY_EXACT_IN_ACCOUNT,
                        pump_amm(),
                    )?;
                    i = new_i;
                    pump_swap::pump_amm_buy_exact_in_swap(
                        accounts.as_slice(),
                        alloc,
                        PUMP_AMM_BUY_EXACT_INDEX,
                    )?
                }

                SwapType::PumpAmmSell {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        PUMP_AMM_SELL_ACCOUNT,
                        pump_amm(),
                    )?;
                    i = new_i;
                    pump_swap::pump_amm_sell_swap(
                        accounts.as_slice(),
                        alloc,
                        PUMP_AMM_SELL_INDEX,
                    )?
                }
                SwapType::Raydium {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_MAX_ACCOUNT,
                        raydium(),
                    )?;
                    i = new_i;
                    raydium_swap(accounts.as_slice(), alloc, RAYDIUM_INDEX)?
                }
                SwapType::RaydiumV2 {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_V2_MAX_ACCOUNT,
                        raydium(),
                    )?;
                    i = new_i;
                    raydium_v2_swap(accounts.as_slice(), alloc, RAYDIUM_V2_INDEX)?
                }
                SwapType::RaydiumCp {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_CPMM_MAX_ACCOUNT,
                        raydium_cp(),
                    )?;
                    i = new_i;
                    raydium_cp_swap(accounts.as_slice(), alloc, RAYDIUMCPMM_INDEX)?
                }
                SwapType::RaydiumClmm {
                    is_base_input,
                    sqrt_price_limit,
                    other_amount_threshold,
                } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_CLMM_MAX_ACCOUNT,
                        raydium_clmm(),
                    )?;
                    i = new_i;
                    raydium_clmm_swap(
                        *is_base_input,
                        *sqrt_price_limit,
                        *other_amount_threshold,
                        accounts.as_slice(),
                        alloc,
                        RAYDIUMCLMM_INDEX,
                    )?
                }
                SwapType::RaydiumClmmV2 {
                    is_base_input,
                    sqrt_price_limit,
                    other_amount_threshold,
                } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_CLMM_V2_MAX_ACCOUNT,
                        raydium_clmm(),
                    )?;
                    i = new_i;
                    raydium_clmm_swap_v2(
                        *is_base_input,
                        *sqrt_price_limit,
                        *other_amount_threshold,
                        accounts.as_slice(),
                        alloc,
                        RAYDIUMCLMM_V2_INDEX,
                    )?
                }
                SwapType::RaydiumStable {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        RAYDIUM_MAX_ACCOUNT,
                        raydium_stable(),
                    )?;
                    i = new_i;
                    raydium_stable_swap(accounts.as_slice(), alloc, RAYDIUM_INDEX)?
                }

                SwapType::Meteora {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        METEORA_MAX_ACCOUNT,
                        meteora(),
                    )?;
                    i = new_i;
                    meteora_swap(accounts.as_slice(), alloc, METEORA_INDEX)?
                }
                SwapType::MeteoraDlmm {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        METEORA_DLMM_MAX_ACCOUNT,
                        meteora_dlmm(),
                    )?;
                    i = new_i;
                    meteora_dlmm_swap(accounts.as_slice(), alloc, METEORADLMM_INDEX)?
                }
                SwapType::MeteoraDlmmV2 {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        METEORA_DLMM_V2_MAX_ACCOUNT,
                        meteora_dlmm(),
                    )?;
                    i = new_i;
                    meteora_dlmm_swap_v2(accounts.as_slice(), alloc, METEORADLMM_INDEX)?
                }
                SwapType::OrcaV1S {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        ORCA_V1_MAX_ACCOUNT,
                        orca_v1(),
                    )?;
                    i = new_i;
                    orca_v1_and_v2_swap(
                        accounts.as_slice(),
                        alloc,
                        ORCA_V1_AND_V2_USER_DESTINATION_ACCOUNT_INDEX,
                        false,
                    )?
                }
                SwapType::OrcaV2S {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        ORCA_V2_MAX_ACCOUNT,
                        orca_v2(),
                    )?;
                    i = new_i;
                    orca_v1_and_v2_swap(
                        accounts.as_slice(),
                        alloc,
                        ORCA_V1_AND_V2_USER_DESTINATION_ACCOUNT_INDEX,
                        true,
                    )?
                }
                SwapType::WhirlpoolS2 {
                    other_amount_threshold: _,
                    sqrt_price_limit,
                    amount_specified_is_input: _,
                    a_to_b,
                    remaining_accounts_info,
                } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        WHIRLPOOL_V2_MAX_ACCOUNT,
                        whirlpool(),
                    )?;
                    i = new_i;

                    let user_destination_account_index = if *a_to_b {
                        WHIRLPOOL_V2_INDEX_A_TO_B
                    } else {
                        WHIRLPOOL_V2_INDEX_B_TO_A
                    };
                    whirlpool_swap_v2(
                        *a_to_b,
                        remaining_accounts_info,
                        *sqrt_price_limit,
                        accounts.as_slice(),
                        alloc,
                        user_destination_account_index,
                    )?
                }
                SwapType::MeteoraDammV2 {} => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        METEORA_DAMM_V2_MAX_ACCOUNT,
                        meteora_damm(),
                    )?;
                    i = new_i;
                    meteora_damm_swap(accounts.as_slice(), alloc, METEORADAMMV2_INDEX)?
                }
                SwapType::HumidiFi { direction, dst_idx } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        HUMIDIFI_MAX_ACCOUNT,
                        humidifi(),
                    )?;
                    i = new_i;
                    humidifi_swap(
                        accounts.as_slice(),
                        alloc,
                        *direction,
                        *dst_idx as usize,
                    )?
                }
                SwapType::BisonFi { direction, dst_idx } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        BISONFI_MAX_ACCOUNT,
                        bisonfi(),
                    )?;
                    i = new_i;
                    bisonfi_swap(
                        accounts.as_slice(),
                        alloc,
                        *direction,
                        *dst_idx as usize,
                    )?
                }
                SwapType::GoonfiV2 { is_user_bid, bump, dst_idx } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        GOONFI_V2_MAX_ACCOUNT,
                        goonfi_v2(),
                    )?;
                    i = new_i;
                    goonfi_v2_swap(
                        accounts.as_slice(),
                        alloc,
                        *is_user_bid,
                        *bump,
                        *dst_idx as usize,
                    )?
                }
                SwapType::AlphaQ { direction, dst_idx } => {
                    let (accounts, new_i) = load_swap_accounts(
                        ctx.remaining_accounts,
                        i,
                        ALPHAQ_MAX_ACCOUNT,
                        alphaq(),
                    )?;
                    i = new_i;
                    alphaq_swap(
                        accounts.as_slice(),
                        alloc,
                        *direction,
                        *dst_idx as usize,
                    )?
                }
                _ => {
                    return Err(error!(CqxErrorCode::InvalidSwapRoute));
                }
            };

            let out_before = buckets[out_idx];
            buckets[out_idx] = out_before.saturating_add(out_amt as u128);
            last_out_idx = out_idx;
        }

        amount = buckets[last_out_idx] as u64;

        let slippage_multiplier = 10_000u64
            .checked_sub(slippage_bps as u64)
            .ok_or(CqxErrorCode::InvalidSlippageBps)?;
        let adjusted_out_amount = quoted_out_amount
            .checked_mul(slippage_multiplier)
            .ok_or(CqxErrorCode::MathOverflow)?
            / 10_000;

        msg!("Final amount: {:?}", amount);
        msg!(
            "Adjusted quoted amount (with slippage): {:?}",
            adjusted_out_amount
        );

        if amount < adjusted_out_amount {
            return Err(CqxErrorCode::SlippageExceedDesireLimit.into());
        }

        msg!("🌐 carbium cqX swap: gross_out={} legs={}", amount, route_plan.len());

        // 2026-05-14: integrator fee. Slippage gate is on the GROSS output
        // (user is told what they'll receive minus the fee_bps deduction at
        // quote time, so quoted_out_amount already accounts for it client-side).
        // We compute fee on `amount` (actual delivered gross), so the fee
        // floats with real slippage. Fee rounds DOWN (integer division) so
        // the user never receives less than the slippage-floor minus fee.
        let fee_amount: u64 = if fee_bps > 0 {
            ((amount as u128)
                .checked_mul(fee_bps as u128)
                .ok_or(CqxErrorCode::MathOverflow)?
                / 10_000u128) as u64
        } else {
            0
        };

        // Unified fee path via token::transfer. Works for BOTH:
        //   SPL output → source = user_dst_ata at remaining[n-2], dest = treasury_ata at remaining[n-1]
        //   SOL output (wSOL temp) → source = temp_wsol at remaining[TEMP_WSOL_INDEX], dest = treasury_wsol_ata at remaining[n-1]
        // The SOL path avoids the rent-exempt issue with system_transfer to a
        // fresh wallet (97k lamports < 890k minimum → silently auto-reverted).
        // Treasury's wSOL ATA can hold any amount > 0 and the treasury can
        // unwrap to native SOL on their end via close_account.
        let output_is_sol = wsol_account_used && !wsol_is_start_input;

        if fee_amount > 0 {
            require!(remaining.len() >= 1, CqxErrorCode::InvalidSwapRoute);
            let n = remaining.len();
            let treasury_ata = remaining[n - 1].clone();
            let user_ai = remaining[USER_INDEX].clone();
            let token_program_ai = if wsol_account_used {
                remaining[TOKEN_PROGRAM_INDEX].clone()
            } else {
                require!(remaining.len() > TOKEN_PROGRAM_INDEX, CqxErrorCode::InvalidSwapRoute);
                remaining[TOKEN_PROGRAM_INDEX].clone()
            };

            let fee_source = if output_is_sol {
                // For SOL output the swap deposited into temp_wsol (managed by
                // cqX PDA but authority = user via initialize_account3).
                // Transfer wSOL out BEFORE the close_account so the close
                // ends up sending only the post-fee balance to the user.
                remaining[TEMP_WSOL_INDEX].clone()
            } else {
                require!(n >= 2, CqxErrorCode::InvalidSwapRoute);
                remaining[n - 2].clone()
            };

            token::transfer(
                CpiContext::new(
                    token_program_ai,
                    token::Transfer {
                        from: fee_source,
                        to: treasury_ata,
                        authority: user_ai,
                    },
                ),
                fee_amount,
            )?;
            if output_is_sol {
                msg!("🌐 carbium fee: {} bps → {} wSOL units to treasury (auto-unwrappable)", fee_bps, fee_amount);
            } else {
                msg!("🌐 carbium fee: {} bps → {} units to treasury (SPL)", fee_bps, fee_amount);
            }
        }

        // Close temp WSOL if used (unchanged from pre-fee behavior).
        if wsol_account_used {
            let user_ai = remaining[USER_INDEX].clone();
            let temp_wsol_ai = remaining[TEMP_WSOL_INDEX].clone();
            let token_program_ai = remaining[TOKEN_PROGRAM_INDEX].clone();

            token::close_account(CpiContext::new(
                token_program_ai,
                CloseAccount {
                    account: temp_wsol_ai,
                    destination: user_ai.clone(),
                    authority: user_ai,
                },
            ))?;
        }

        Ok(())
    }
}
