use anchor_lang::prelude::*;

declare_id!("ByuDmqDW9hMrU4LKk3XfBekrXRjSV2f8Hjk3ZMRcjD4R");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
