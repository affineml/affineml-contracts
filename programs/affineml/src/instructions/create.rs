use anchor_lang::prelude::*;

use crate::{
    seeds::*,
    state::{Config, TaskInfo},
};

#[derive(Accounts)]
pub struct Create<'info> {
    // config account
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump=config.bump
    )]
    pub config: Box<Account<'info, Config>>,

    // task info account
    #[account(
        init,
        payer = signer,
        space = 8+ TaskInfo::INIT_SPACE,
        seeds = [
            TASK_SEED,
            config.task_count.to_be_bytes().as_ref(),
        ],
        bump
    )]
    pub task_info: Box<Account<'info, TaskInfo>>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Create<'info>>,
    total_round: u8,
) -> Result<()> {
    let task_info = &mut ctx.accounts.task_info;
    task_info.bump = ctx.bumps.task_info;
    task_info.creator = ctx.accounts.signer.key();
    task_info.commit_reward = 1000000000;
    task_info.total_round = total_round;
    task_info.cur_round = 1;

    let config = &mut ctx.accounts.config;
    config.task_count = config.task_count + 1;

    Ok(())
}
