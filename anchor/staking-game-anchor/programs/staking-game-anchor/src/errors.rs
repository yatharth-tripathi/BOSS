use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("Pool name is too long (max 32 characters)")]
    PoolNameTooLong,

    #[msg("Pool name cannot be empty")]
    PoolNameEmpty,

    #[msg("User has already joined this pool")]
    AlreadyJoined,

    #[msg("User has not joined this pool yet")]
    NotJoined,

    #[msg("Invalid score value")]
    InvalidScore,

    #[msg("User has already claimed rewards")]
    AlreadyClaimed,

    #[msg("No rewards available to claim")]
    NoRewardsAvailable,

    #[msg("Insufficient funds in pool for rewards")]
    InsufficientPoolFunds,

    #[msg("Only pool authority can perform this action")]
    UnauthorizedAction,

    #[msg("Pool does not exist")]
    PoolNotFound,

    #[msg("Participant does not exist")]
    ParticipantNotFound,

    #[msg("Mathematical overflow occurred")]
    MathOverflow,

    #[msg("Cannot claim rewards with zero score")]
    ZeroScore,

    #[msg("Pool has no participants")]
    NoParticipants,

    #[msg("Invalid pool authority")]
    InvalidAuthority,

    #[msg("Reward calculation failed")]
    RewardCalculationFailed,
}