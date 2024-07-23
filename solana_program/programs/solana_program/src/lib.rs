use anchor_lang::prelude::*;

declare_id!("ByuDmqDW9hMrU4LKk3XfBekrXRjSV2f8Hjk3ZMRcjD4R");

#[program]
mod solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current counter: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"my_seed", user.key().as_ref()], bump, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(Default, Debug)]
pub struct Counter {
    count: u64,
}
