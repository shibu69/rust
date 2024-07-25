use anchor_lang::prelude::*;
use solana_program::clock::Clock;

declare_id!("5aEZQf3hvt1bEyKEYnvy1zTNjwR5qwAagc7Aqdm43qfM");

#[program]
pub mod guess_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    fn generate_number()-> u32{
       let clock = Clock::get().expect("Failed");
       let last_digit = (clock.unix_timestamp % 10) as u8;
       let result = (last_digit+1) as u32;
       result
    }
}

#[account]
pub struct GuessingAccount {
    
}

#[derive(Accounts)]
pub struct Initialize {}
