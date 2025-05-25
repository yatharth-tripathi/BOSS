use anchor_lang::prelude::*;
use crate::state::{Pool, Participant, seeds};

/// Accounts required for claiming rewards
#[derive(Accounts)]
pub struct ClaimReward<'info> {
    /// Pool from which rewards are being claimed
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// Participant claiming the reward
    #[account(
        mut,
        seeds = [seeds::PARTICIPANT_SEED, pool.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
        constraint = participant.user == user.key(),
        constraint = participant.pool == pool.key()
    )]
    pub participant: Account<'info, Participant>,

    /// Pool authority account (source of reward funds)
    #[account(
        mut,
        constraint = pool_authority.key() == pool.authority
    )]
    pub pool_authority: SystemAccount<'info>,

    /// User claiming the reward
    #[account(mut)]
    pub user: Signer<'info>,

    /// System program for transferring lamports
    pub system_program: Program<'info, System>,
}