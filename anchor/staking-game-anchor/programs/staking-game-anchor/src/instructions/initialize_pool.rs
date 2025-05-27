use anchor_lang::prelude::*;
use super::super::state::Pool;
use super::super::errors::PoolError;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializePool<'info> {
    /// The authority who will manage the pool
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// The pool account to be created
    #[account(
        init,
        payer = authority,
        space = Pool::LEN,
        seeds = [Pool::SEEDS, authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// System program for account creation
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePool>, name: String) -> Result<()> {
    // Validate pool name
    if name.is_empty() {
        return Err(PoolError::PoolNameEmpty.into());
    }
    
    if name.len() > 64 {
        return Err(PoolError::PoolNameTooLong.into());
    }

    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Initialize pool data
    pool.authority = ctx.accounts.authority.key();
    pool.name = name;
    pool.participant_count = 0;
    pool.total_score = 0;
    pool.total_rewards = 0;
    pool.is_active = true;
    pool.created_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.pool;

    msg!("Pool '{}' initialized by authority: {}", pool.name, pool.authority);
    
    Ok(())
}