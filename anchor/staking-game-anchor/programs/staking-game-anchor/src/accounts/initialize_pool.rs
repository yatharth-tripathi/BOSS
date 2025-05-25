use anchor_lang::prelude::*;
use crate::state::GamePool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<GamePool>())]
    pub game_pool: Account<'info, GamePool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
