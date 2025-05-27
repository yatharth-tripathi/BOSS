use anchor_lang::prelude::*;

declare_id!("8FjJrRvqdJhUtPt2hfBKMkhWkfpw73Yvucv2cFH9ouaN");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::{
    ClaimReward,
    InitializePool,
    JoinPool,
    SubmitScore
};

#[program]
pub mod pool_system {
    use super::*;

    /// Initialize a new pool
    pub fn initialize_pool(ctx: Context<InitializePool>, name: String) -> Result<()> {
        instructions::initialize_pool::handler(ctx, name)
    }

    /// Join an existing pool as a participant
    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        instructions::join_pool::handler(ctx)
    }

    /// Submit a score for a participant
    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
        instructions::submit_score::handler(ctx, score)
    }

    /// Claim rewards based on participation
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::handler(ctx)
    }
}