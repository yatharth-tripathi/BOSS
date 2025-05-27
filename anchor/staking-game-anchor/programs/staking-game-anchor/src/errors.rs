use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("Pool name is too long. Maximum 64 characters allowed.")]
    PoolNameTooLong,
    
    #[msg("Pool name cannot be empty.")]
    PoolNameEmpty,
    
    #[msg("Pool is not active.")]
    PoolInactive,
    
    #[msg("Participant has already joined this pool.")]
    ParticipantAlreadyJoined,
    
    #[msg("Participant has not joined this pool.")]
    ParticipantNotFound,
    
    #[msg("Score must be greater than zero.")]
    InvalidScore,
    
    #[msg("Participant has already claimed rewards.")]
    RewardAlreadyClaimed,
    
    #[msg("No rewards available to claim.")]
    NoRewardsAvailable,
    
    #[msg("Insufficient pool balance for reward distribution.")]
    InsufficientPoolBalance,
    
    #[msg("Only pool authority can perform this action.")]
    UnauthorizedAccess,
    
    #[msg("Arithmetic overflow occurred.")]
    ArithmeticOverflow,
    
    #[msg("Invalid participant for this pool.")]
    InvalidParticipant,
}