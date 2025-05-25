
use anchor_lang::prelude::*;

pub mod accounts;
pub mod errors;
pub mod instructions;
pub mod state;

use accounts::*;
use errors::*;
use instructions::*;

#[program]
pub mod pool_system {
    use super::*;

    /// Initialize a new pool with the given name
    /// Only the authority can initialize pools
    pub fn initialize_pool(ctx: Context<InitializePool>, name: String) -> Result<()> {
        instructions::initialize_pool::handler(ctx, name)
    }

    /// Join an existing pool as a participant
    /// Each user can only join once per pool
    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        instructions::join_pool::handler(ctx)
    }

    /// Submit a score for the participant in the pool
    /// Participants can submit multiple scores (updates previous score)
    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
        instructions::submit_score::handler(ctx, score)
    }

    /// Claim rewards based on participation and score
    /// Each participant can only claim once
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::handler(ctx)
    }
}