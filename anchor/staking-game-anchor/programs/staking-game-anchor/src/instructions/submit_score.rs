use anchor_lang::prelude::*;
use super::super::state::{Pool, Participant};
use super::super::errors::PoolError;

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// The user submitting the score
    pub user: Signer<'info>,
    
    /// The pool the participant belongs to
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    
    /// The participant's account
    #[account(
        mut,
        seeds = [Participant::SEEDS, pool.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
        constraint = participant.wallet == user.key()
    )]
    pub participant: Account<'info, Participant>,
}

pub fn handler(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;
    let clock = Clock::get()?;

    // Check if pool is active
    if !pool.is_active {
        return Err(PoolError::PoolInactive.into());
    }

    // Validate score
    if score == 0 {
        return Err(PoolError::InvalidScore.into());
    }

    // Update pool total score (subtract old score, add new score)
    pool.total_score = pool.total_score
        .checked_sub(participant.score)
        .ok_or(PoolError::ArithmeticOverflow)?
        .checked_add(score)
        .ok_or(PoolError::ArithmeticOverflow)?;

    // Update participant score and timestamp
    participant.score = score;
    participant.score_updated_at = clock.unix_timestamp;

    msg!(
        "User {} submitted score {} to pool '{}'. Pool total score: {}",
        ctx.accounts.user.key(),
        score,
        pool.name,
        pool.total_score
    );

    Ok(())
}