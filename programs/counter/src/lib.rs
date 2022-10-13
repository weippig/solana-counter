use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("9EoiyC3P15LGQs6ZyLeY9a9FmrRMENUyHfQ2W9PkhYxV");

#[program]
pub mod counter {

    use super::*;

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count -=1 ;
        Ok(())
    }
}
// Transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}