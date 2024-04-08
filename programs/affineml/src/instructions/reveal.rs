use anchor_lang::prelude::*;

use crate::constants::*;
use crate::{
    errors::BizError,
    seeds::*,
    state::{Config, TaskInfo, UserInfo},
};
use std::collections::HashSet;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct RevealArgs {
    pub task_index: u64,
    pub reveal_info: [u8; 32],
}

#[derive(Accounts)]
#[instruction(args: RevealArgs)]

pub struct Reveal<'info> {
    /// config account
    #[account(
        init_if_needed,
        payer=signer,
        space = 8+ Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Box<Account<'info, Config>>,

    /// task info
    #[account(
        mut,
        seeds = [
            TASK_SEED,
            args.task_index.to_be_bytes().as_ref(),
        ],
        bump=task_info.bump,
    )]
    pub task_info: Box<Account<'info, TaskInfo>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn reveal_handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Reveal<'info>>,
    args: RevealArgs,
) -> Result<()> {
    let task_info = &mut ctx.accounts.task_info;
    require!(task_info.total_user >= 5, BizError::TooFewUserError);
    let mut matched = 0;

    // Check for duplicate user info
    let mut users: HashSet<Pubkey> = HashSet::new();

    for account_info in ctx.remaining_accounts.into_iter() {
        let user_info = &mut Box::from(Account::<UserInfo>::try_from(account_info)?);

        if user_info.user_pubkey == Pubkey::default() || users.contains(&user_info.user_pubkey) {
            continue;
        }
        users.insert(user_info.user_pubkey);

        let check_result = task_info.check_commit_result(&args.reveal_info, &user_info);
        if check_result {
            matched = matched + 1;
            user_info.commit_count += 1;
        }
    }
    require!(matched >= 5, BizError::RevealError);
    task_info.cur_round_commit = 0;
    task_info.cur_commit_user = [Pubkey::default(); MAX_COMMIT_USER_NUM];
    if task_info.cur_round == task_info.total_round {
        task_info.ended = true;
    } else {
        task_info.cur_round += 1;
    }
    Ok(())
}
