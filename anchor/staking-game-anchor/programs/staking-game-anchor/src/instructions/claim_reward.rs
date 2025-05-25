use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::accounts::ClaimReward;
use crate::errors::PoolError;

/// Handler for claiming rewards
pub fn handler(ctx: Context<ClaimReward>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let participant = &mut ctx.accounts.participant;
    let pool_authority = &mut ctx.accounts.pool_authority;
    let user = &mut ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;

    // Check if participant has already claimed
    if participant.has_claimed {
        return Err(PoolError::AlreadyClaimed.into());
    }

    // Check if participant has a valid score
    if participant.score == 0 {
        return Err(PoolError::ZeroScore.into());
    }

    // Check if pool has rewards available
    if !pool.has_rewards() {
        return Err(PoolError::NoRewardsAvailable.into());
    }

    // Calculate reward amount
    let reward_amount = participant.calculate_reward(pool);
    
    if reward_amount == 0 {
        return Err(PoolError::RewardCalculationFailed.into());
    }

    // Check if pool authority has sufficient funds
    if pool_authority.lamports() < reward_amount {
        return Err(PoolError::InsufficientPoolFunds.into());
    }

    // Transfer reward from pool authority to user
    let transfer_instruction = system_program::Transfer {
        from: pool_authority.to_account_info(),
        to: user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        system_program.to_account_info(),
        transfer_instruction,
    );

    system_program::transfer(cpi_ctx, reward_amount)?;

    // Update participant state
    participant.has_claimed = true;

    // Update pool statistics
    pool.claimed_count = pool.claimed_count
        .checked_add(1)
        .ok_or(PoolError::MathOverflow)?;

    // Update total rewards distributed
    pool.total_rewards = pool.total_rewards
        .checked_add(reward_amount)
        .ok_or(PoolError::MathOverflow)?;

    msg!(
        "User {} claimed reward of {} lamports from pool '{}'",
        user.key(),
        reward_amount,
        pool.name
    );

    Ok(())
}