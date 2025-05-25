use anchor_lang::prelude::*;
use crate::accounts::InitializePool;
use crate::errors::PoolError;

/// Handler for initializing a new pool
pub fn handler(ctx: Context<InitializePool>, name: String) -> Result<()> {
    // Validate pool name
    if name.is_empty() {
        return Err(PoolError::PoolNameEmpty.into());
    }
    
    if name.len() > 32 {
        return Err(PoolError::PoolNameTooLong.into());
    }

    let pool = &mut ctx.accounts.pool;
    let authority = &ctx.accounts.authority;
    let clock = Clock::get()?;

    // Initialize pool state
    pool.authority = authority.key();
    pool.name = name;
    pool.total_participants = 0;
    pool.total_score = 0;
    pool.total_rewards = 0;
    pool.claimed_count = 0;
    pool.created_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.pool;

    msg!("Pool '{}' initialized by authority: {}", pool.name, pool.authority);

    Ok(())
}