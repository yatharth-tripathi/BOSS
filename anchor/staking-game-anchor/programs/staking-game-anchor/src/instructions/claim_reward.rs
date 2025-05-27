use anchor_lang::prelude::*;
use super::super::state::{Pool, Participant};
use super::super::errors::PoolError;

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    /// The user claiming the reward
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The pool from which rewards are being claimed
    #[account(
        mut,
        seeds = [Pool::SEEDS, pool.authority.as_ref(), pool.name.as_bytes()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// The participant's account
    #[account(
        mut,
        seeds = [Participant::SEEDS, pool.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
        constraint = participant.wallet == user.key()
    )]
    pub participant: Account<'info, Participant>,
    
    /// System program for transferring SOL
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimReward>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;

    // Check if participant has already claimed rewards
    if participant.has_claimed {
        return Err(PoolError::RewardAlreadyClaimed.into());
    }

    // Check if participant has a score
    if participant.score == 0 {
        return Err(PoolError::NoRewardsAvailable.into());
    }

    // Check if pool has total score to calculate rewards
    if pool.total_score == 0 {
        return Err(PoolError::NoRewardsAvailable.into());
    }

    // Calculate participant's share of rewards
    // For this example, we'll distribute based on score percentage
    // Reward = (participant_score / total_score) * pool_balance
    let pool_balance = pool.to_account_info().lamports();
    
    // Reserve some lamports for rent (approximately 0.002 SOL)
    let reserved_lamports = 2_000_000u64;
    
    if pool_balance <= reserved_lamports {
        return Err(PoolError::InsufficientPoolBalance.into());
    }

    let available_balance = pool_balance - reserved_lamports;
    
    let reward_amount = (available_balance as u128)
        .checked_mul(participant.score as u128)
        .ok_or(PoolError::ArithmeticOverflow)?
        .checked_div(pool.total_score as u128)
        .ok_or(PoolError::ArithmeticOverflow)? as u64;

    if reward_amount == 0 {
        return Err(PoolError::NoRewardsAvailable.into());
    }

    // Transfer rewards from pool to participant
    let pool_info = pool.to_account_info();
    let user_info = ctx.accounts.user.to_account_info();

    **pool_info.try_borrow_mut_lamports()? = pool_info
        .lamports()
        .checked_sub(reward_amount)
        .ok_or(PoolError::InsufficientPoolBalance)?;

    **user_info.try_borrow_mut_lamports()? = user_info
        .lamports()
        .checked_add(reward_amount)
        .ok_or(PoolError::ArithmeticOverflow)?;

    // Mark participant as having claimed rewards
    participant.has_claimed = true;
    participant.rewards_claimed = reward_amount;

    // Update pool total rewards distributed
    pool.total_rewards = pool.total_rewards
        .checked_add(reward_amount)
        .ok_or(PoolError::ArithmeticOverflow)?;

    msg!(
        "User {} claimed {} lamports from pool '{}' based on score {}",
        ctx.accounts.user.key(),
        reward_amount,
        pool.name,
        participant.score
    );

    Ok(())
}