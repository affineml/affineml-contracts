use anchor_lang::prelude::*;

declare_id!("GD8gkSYT3nK3Y5kfNVZBXET98U6PUUrP65eMec7m1az8");

pub mod constants;
pub mod errors;
pub mod events;
mod instructions;
pub mod seeds;
pub mod state;

use instructions::*;

#[program]
pub mod affineml {
    use super::*;

    // The configuation of program
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    ///
    pub fn init<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Initialize<'info>>,
    ) -> Result<()> {
        init_handler(ctx)
    }

    // Create new task
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    ///
    pub fn create<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Create<'info>>,
        total_round: u8,
    ) -> Result<()> {
        create_handler(ctx, total_round)
    }

    // Commit the hash of the calc result.
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `args.task_index` - index of task
    /// * `args.commit_hash` - hash
    ///
    pub fn commit<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Commit<'info>>,
        args: CommitArgs,
    ) -> Result<()> {
        commit_handler(ctx, args)
    }

    // Reveal the result of the task.
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `args.task_index` - index of task
    /// * `args.reward_pubkey` - winning pubkey array
    /// * `args.reward_amount` - corresponding prize amounts
    ///
    pub fn reveal<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Reveal<'info>>,
        args: RevealArgs,
    ) -> Result<()> {
        reveal_handler(ctx, args)
    }

    // Claim the reward
    /// # Arguments
    ///
    /// * `ctx`- The accounts needed by instruction.
    /// * `args.index` - winning pubkey array
    ///
    pub fn claim<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Claim<'info>>,
        args: ClaimArgs,
    ) -> Result<()> {
        claim_handler(ctx, args)
    }
}
