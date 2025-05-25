use anchor_lang::prelude::*;
use crate::state::{Pool, Participant, seeds};

/// Accounts required for joining a pool
#[derive(Accounts)]
pub struct JoinPool<'info> {
    /// Pool to join (must exist)
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// Participant account to be created (PDA)
    #[account(
        init,
        payer = user,
        space = Participant::LEN,
        seeds = [seeds::PARTICIPANT_SEED, pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub participant: Account<'info, Participant>,

    /// User who wants to join the pool
    #[account(mut)]
    pub user: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}