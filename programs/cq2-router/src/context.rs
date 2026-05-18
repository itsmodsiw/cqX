use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    /// CHECK: caller-controlled program account; only used as a placeholder
    /// for Anchor's required `Accounts` struct. The real action lives in
    /// `remaining_accounts`.
    pub program: UncheckedAccount<'info>,
}
