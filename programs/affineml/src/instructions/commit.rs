use anchor_lang::prelude::*;

use crate::{
    seeds::*,
    state::{TaskInfo, UserInfo},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CommitArgs {
    pub commit_hash: [u8; 32],
    pub task_index: u64,
}

#[derive(Accounts)]
#[instruction(args: CommitArgs)]
pub struct Commit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // task info account
    #[account(
        mut,
        seeds = [
            TASK_SEED,
            args.task_index.to_be_bytes().as_ref(),
        ],
        bump=task_info.bump,
    )]
    pub task_info: Box<Account<'info, TaskInfo>>,

    // task info account
    #[account(
        init_if_needed,
        payer = signer,
        space = 8+ UserInfo::INIT_SPACE,
        seeds = [
            USER_SEED,
            args.task_index.to_be_bytes().as_ref(),
            signer.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    pub system_program: Program<'info, System>,
}

pub fn commit_handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Commit<'info>>,
    args: CommitArgs,
) -> Result<()> {
    let task_info = &ctx.accounts.task_info;
    let user_info = &mut ctx.accounts.user_info;
    let signer = ctx.accounts.signer.key();

    task_info.check_commitable()?;

    user_info.commit_hash = args.commit_hash;
    user_info.last_round = task_info.cur_round;
    let task_info = &mut ctx.accounts.task_info;

    // Check if the user has commited in this task
    if !user_info.init {
        user_info.init = true;
        user_info.user_pubkey = signer;
        task_info.total_user += 1;
    }
    // Check if the user has commited in this round
    let find_result = task_info
        .cur_commit_user
        .iter()
        .position(|user| user == &signer);
    if find_result.is_none() {
        let cur_round_commit = task_info.cur_round_commit as usize;
        task_info.cur_commit_user[cur_round_commit] = signer;
        task_info.cur_round_commit += 1;
    }
    Ok(())
}
