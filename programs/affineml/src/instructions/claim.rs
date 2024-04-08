use anchor_lang::prelude::*;

use crate::{
    errors::BizError,
    seeds::*,
    state::{Config, TaskInfo, UserInfo},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ClaimArgs {
    pub task_index: u64,
}

#[derive(Accounts)]
#[instruction(_args: ClaimArgs)]
pub struct Claim<'info> {
    /// config account
    #[account(
        seeds = [CONFIG_SEED],
        bump=config.bump
    )]
    pub config: Box<Account<'info, Config>>,

    /// token mint account
    #[account(
        mut,
        constraint = mint.key()==config.mint @ BizError::MintError,
    )]
    pub mint: Box<Account<'info, Mint>>,

    /// task info
    #[account(
        mut,
        seeds = [
            TASK_SEED,
            _args.task_index.to_be_bytes().as_ref(),
        ],
        bump=task_info.bump,
    )]
    pub task_info: Box<Account<'info, TaskInfo>>,

    /// user associated token account to receive reward
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            USER_SEED,
            _args.task_index.to_be_bytes().as_ref(),
            signer.key().as_ref(),
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
impl<'info> Claim<'info> {
    pub fn mint_to_receiver_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn claim_handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Claim<'info>>,
    _args: ClaimArgs,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let task_info = &mut ctx.accounts.task_info;
    require!(task_info.ended, BizError::MustEndError);

    let user_info = &mut ctx.accounts.user_info;
    require!(!user_info.claimed, BizError::ClaimerError);
    user_info.claimed = true;

    let reward = task_info.commit_reward * user_info.commit_count as u64;

    if reward > 0 {
        let seeds = CONFIG_SEED;
        let signer_seeds: &[&[&[u8]]] = &[&[seeds, &[config.bump]]];
        mint_to(
            ctx.accounts
                .mint_to_receiver_ctx()
                .with_signer(signer_seeds),
            reward,
        )?;
    }
    Ok(())
}
