use anchor_lang::prelude::*;

/// Pool account structure
#[account]
pub struct Pool {
    /// The authority who created and manages the pool
    pub authority: Pubkey,
    /// Name of the pool
    pub name: String,
    /// Total number of participants in the pool
    pub participant_count: u64,
    /// Total score accumulated by all participants
    pub total_score: u64,
    /// Total rewards available in the pool (in lamports)
    pub total_rewards: u64,
    /// Whether the pool is active or closed
    pub is_active: bool,
    /// Timestamp when the pool was created
    pub created_at: i64,
    /// Pool bump for PDA derivation
    pub bump: u8,
}

impl Pool {
    /// Calculate space needed for Pool account
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        4 + 64 + // name (assuming max 64 chars)
        8 + // participant_count
        8 + // total_score
        8 + // total_rewards
        1 + // is_active
        8 + // created_at
        1; // bump

    /// Seeds for Pool PDA
    pub const SEEDS: &'static [u8] = b"pool";
}

/// Participant account structure
#[account]
pub struct Participant {
    /// The participant's wallet address
    pub wallet: Pubkey,
    /// The pool this participant belongs to
    pub pool: Pubkey,
    /// The participant's score
    pub score: u64,
    /// Whether the participant has claimed their reward
    pub has_claimed: bool,
    /// Total rewards claimed by this participant
    pub rewards_claimed: u64,
    /// Timestamp when participant joined
    pub joined_at: i64,
    /// Timestamp when score was last updated
    pub score_updated_at: i64,
    /// Participant bump for PDA derivation
    pub bump: u8,
}

impl Participant {
    /// Calculate space needed for Participant account
    pub const LEN: usize = 8 + // discriminator
        32 + // wallet
        32 + // pool
        8 + // score
        1 + // has_claimed
        8 + // rewards_claimed
        8 + // joined_at
        8 + // score_updated_at
        1; // bump

    /// Seeds for Participant PDA
    pub const SEEDS: &'static [u8] = b"participant";
}