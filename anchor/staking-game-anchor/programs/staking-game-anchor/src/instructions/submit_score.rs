use anchor_lang::prelude::*;
use crate::accounts::SubmitScore;
use crate::errors::PoolError;

/// Handler for submitting a score
pub fn handler(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;

    // Validate score (you can add more validation logic here)
    if score == 0 {
        return Err(PoolError::InvalidScore.into());
    }

    // Update pool's total score (subtract old score, add new score)
    pool.total_score = pool.total_score
        .checked_sub(participant.score)
        .ok_or(PoolError::MathOverflow)?
        .checked_add(score)
        .ok_or(PoolError::MathOverflow)?;

    // Update participant's score
    participant.score = score;
    participant.score_updated_at = clock.unix_timestamp;

    msg!(
        "User {} submitted score {} to pool '{}'", 
        user.key(), 
        score, 
        pool.name
    );

    Ok(())
}