use anchor_lang::prelude::*;

#[error_code]
pub enum CqxErrorCode {
    #[msg("Invalid Program")]
    InvalidProgram,
    #[msg("Invalid Token")]
    InvalidToken,
    #[msg("Invalid Number Of Accounts")]
    NotEnoughAccounts,
    #[msg("Provider account not found")]
    ProviderNotFound,
    #[msg("Slippage exceed desire limit")]
    SlippageExceedDesireLimit,
    #[msg("Underflow occurred while calculating swapped amount.")]
    SwapUnderflow,
    #[msg("Amount in must be greater than zero")]
    AmountInMustBeGreaterThanZero,
    #[msg("Invalid route")]
    InvalidSwapRoute,
    #[msg("Invalid input amount")]
    InvalidInputAmount,
    #[msg("InvalidSlippageBps")]
    InvalidSlippageBps,
    #[msg("InvalidPercentBps")]
    InvalidPercentBps,
    #[msg("MathOverflow")]
    MathOverflow,
    #[msg("MathUnderflow")]
    MathUnderflow,
    #[msg("HumidiFi pool data invalid")]
    HumidiFiPoolInvalid,
    #[msg("Fee bps exceeds MAX_FEE_BPS")]
    InvalidFeeBps,
}
