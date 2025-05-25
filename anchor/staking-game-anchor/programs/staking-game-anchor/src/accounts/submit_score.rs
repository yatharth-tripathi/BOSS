use anchor_lang::prelude::*;
use crate::state::{Pool, Participant, seeds};

/// Accounts required for submitting a score
#[derive(Accounts)]
pub struct SubmitScore<'info> {
    /// Pool where the score is being submitted
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    /// Participant submitting the score (must exist and belong to user)
    #[account(
        mut,
        seeds = [seeds::PARTICIPANT_SEED, pool.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
        constraint = participant.user == user.key(),
        constraint = participant.pool == pool.key()
    )]
    pub participant: Account<'info, Participant>,

    /// User submitting the score
    pub user: Signer<'info>,
}