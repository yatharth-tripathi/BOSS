use anchor_lang::prelude::*;
use crate::accounts::JoinPool;

/// Handler for joining a pool
pub fn handler(ctx: Context<JoinPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;

    // Initialize participant state
    participant.pool = pool.key();
    participant.user = user.key();
    participant.score = 0; // Default score is 0
    participant.has_claimed = false;
    participant.joined_at = clock.unix_timestamp;
    participant.score_updated_at = clock.unix_timestamp;
    participant.bump = ctx.bumps.participant;

    // Update pool statistics
    pool.total_participants = pool.total_participants
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("User {} joined pool '{}'", user.key(), pool.name);

    Ok(())
}