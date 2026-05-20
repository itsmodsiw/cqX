use crate::error::CqxErrorCode;
use anchor_lang::prelude::*;

pub fn extract_accounts<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    arr: &mut Vec<AccountInfo<'info>>,
    max_len: usize,
    y: &mut usize,
    program_id: Pubkey,
) {
    while *y < max_len {
        let acc = &remaining_accounts[*y];

        if acc.key() == program_id {
            arr.extend_from_slice(&remaining_accounts[*y..*y + 1]);
            *y += 1;
            break;
        } else if acc.owner == &program_id {
            arr.extend_from_slice(&remaining_accounts[*y..*y + 1]);
        }

        *y += 1;
    }
}

pub fn load_swap_accounts<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    start_index: usize,
    fixed_len: usize,
    program_id: Pubkey,
) -> Result<(Vec<AccountInfo<'info>>, usize)> {
    let upper_bound = start_index + fixed_len;

    require!(
        remaining_accounts.len() >= upper_bound,
        CqxErrorCode::NotEnoughAccounts
    );

    let mut arr: Vec<AccountInfo<'info>> = remaining_accounts[start_index..upper_bound].to_vec();
    let mut i = upper_bound;

    if remaining_accounts[i - 1].key() != program_id {
        let mut y = i;
        let max_len = remaining_accounts.len();
        extract_accounts(remaining_accounts, &mut arr, max_len, &mut y, program_id);
        require!(y <= max_len, CqxErrorCode::NotEnoughAccounts);
        i = y;
    }

    Ok((arr, i))
}
