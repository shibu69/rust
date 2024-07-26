use anchor_lang::prelude::*;
use solana_program::clock::Clock;

declare_id!("5aEZQf3hvt1bEyKEYnvy1zTNjwR5qwAagc7Aqdm43qfM");

#[program]
pub mod guess_game {
    use super::*;

    pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
        let guessing_account = &mut ctx.accounts.guessing_account;
        guessing_account.number = generate_number();
        Ok(())
    }
    pub fn guess(ctx: Context<AccountContext>, number: u32) -> Result<()> {
        let guessing_account = &mut ctx.accounts.guessing_account;
        let target = guessing_account.number;

        match number.cmp(&target) {
            Ordering::Less => return err!(MyError::NumberTooSmall),
            Ordering::Greater => {
                return err!(MyError::NumberTooLarge);
            }
            Ordering::Equal => return Ok(()),
        }
    }

    fn generate_number() -> u32 {
        let clock = Clock::get().expect("Failed");
        let last_digit = (clock.unix_timestamp % 10) as u8;
        let result = (last_digit + 1) as u32;
        result
    }
}

#[account]
pub struct GuessingAccount {
    pub number: u32,
}

//AccountContext use to contain all necessary account references needed
#[derive(Accounts)] //used to transform struct into a type corresponding to blockchain accounts
pub struct AccountContext<'info> {
    //'info = alifetime param for validity period
    #[account(
        init_if_needed,
        space=32,
        payer=payer,
        seeds=[b"guessing pda"],
        bump
    )]
    pub guessing_account: Account<'info, GuessingAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum MyError {
    #[msg("Too small")]
    NumberTooSmall,
    #[msg("Too larget")]
    NumberTooLarge,
}
