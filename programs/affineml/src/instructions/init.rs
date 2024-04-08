use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token};

use crate::{seeds::*, state::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// token mint account
    #[account(
        init,
        seeds = [MINT_SEED],
        bump,
        payer = signer,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint: Box<Account<'info, Mint>>,

    /// config account
    #[account(
        init,
        payer=signer,
        space = 8+ Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn init_handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Initialize<'info>>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.bump = ctx.bumps.config;
    config.mint = ctx.accounts.mint.key();
    Ok(())
}
