use anchor_lang::prelude::*;

/// Pool state containing pool information and statistics
#[account]
pub struct Pool {
    /// Authority who can manage the pool
    pub authority: Pubkey,
    /// Name of the pool (max 32 characters)
    pub name: String,
    /// Total number of participants who joined
    pub total_participants: u64,
    /// Total score sum from all participants
    pub total_score: u64,
    /// Total rewards available in the pool (in lamports)
    pub total_rewards: u64,
    /// Number of participants who have claimed rewards
    pub claimed_count: u64,
    /// Timestamp when the pool was created
    pub created_at: i64,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl Pool {
    /// Calculate space needed for Pool account
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        4 + 32 + // name (String with length prefix, max 32 chars)
        8 + // total_participants
        8 + // total_score
        8 + // total_rewards
        8 + // claimed_count
        8 + // created_at
        1; // bump

    /// Calculate average score for reward distribution
    pub fn average_score(&self) -> u64 {
        if self.total_participants == 0 {
            0
        } else {
            self.total_score / self.total_participants
        }
    }

    /// Check if pool has rewards available
    pub fn has_rewards(&self) -> bool {
        self.total_rewards > 0 && self.claimed_count < self.total_participants
    }
}

/// Participant state containing individual participation data
#[account]
pub struct Participant {
    /// Pool this participant belongs to
    pub pool: Pubkey,
    /// Participant's wallet address
    pub user: Pubkey,
    /// Participant's submitted score
    pub score: u64,
    /// Whether the participant has claimed their reward
    pub has_claimed: bool,
    /// Timestamp when the participant joined
    pub joined_at: i64,
    /// Timestamp when score was last updated
    pub score_updated_at: i64,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl Participant {
    /// Calculate space needed for Participant account
    pub const LEN: usize = 8 + // discriminator
        32 + // pool
        32 + // user
        8 + // score
        1 + // has_claimed
        8 + // joined_at
        8 + // score_updated_at
        1; // bump

    /// Calculate reward amount based on score and pool statistics
    pub fn calculate_reward(&self, pool: &Pool) -> u64 {
        if pool.total_participants == 0 || pool.total_rewards == 0 {
            return 0;
        }

        // Simple reward calculation: base reward + bonus based on score
        let base_reward = pool.total_rewards / pool.total_participants;
        let average_score = pool.average_score();
        
        if average_score == 0 {
            base_reward
        } else {
            // Give bonus/penalty based on performance relative to average
            let score_multiplier = (self.score * 100) / average_score.max(1);
            (base_reward * score_multiplier) / 100
        }
    }
}

/// Seeds for PDA derivation
pub mod seeds {
    pub const POOL_SEED: &[u8] = b"pool";
    pub const PARTICIPANT_SEED: &[u8] = b"participant";
}