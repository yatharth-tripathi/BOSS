use anchor_lang::prelude::*;
use crate::state::{Pool, seeds};

/// Accounts required for initializing a new pool
#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializePool<'info> {
    /// Pool account to be created (PDA)
    #[account(
        init,
        payer = authority,
        space = Pool::LEN,
        seeds = [seeds::POOL_SEED, authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    /// Authority who will own and manage the pool
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}