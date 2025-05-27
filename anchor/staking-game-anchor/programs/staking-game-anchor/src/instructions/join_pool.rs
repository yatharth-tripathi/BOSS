use anchor_lang::prelude::*;
use super::super::state::{Pool, Participant};
use super::super::errors::PoolError;

#[derive(Accounts)]
pub struct JoinPool<'info> {
    /// The user who wants to join the pool
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The pool to join
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// The participant account to be created
    #[account(
        init,
        payer = user,
        space = Participant::LEN,
        seeds = [Participant::SEEDS, pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub participant: Account<'info, Participant>,
    
    /// System program for account creation
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<JoinPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;
    let clock = Clock::get()?;

    // Check if pool is active
    if !pool.is_active {
        return Err(PoolError::PoolInactive.into());
    }

    // Initialize participant data
    participant.wallet = ctx.accounts.user.key();
    participant.pool = pool.key();
    participant.score = 0;
    participant.has_claimed = false;
    participant.rewards_claimed = 0;
    participant.joined_at = clock.unix_timestamp;
    participant.score_updated_at = 0;
    participant.bump = ctx.bumps.participant;

    // Update pool participant count
    pool.participant_count = pool.participant_count
        .checked_add(1)
        .ok_or(PoolError::ArithmeticOverflow)?;

    msg!(
        "User {} joined pool '{}'. Total participants: {}",
        ctx.accounts.user.key(),
        pool.name,
        pool.participant_count
    );

    Ok(())
}