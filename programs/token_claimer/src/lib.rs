use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

declare_id!("E2dMh4oEcsPJqfy13vdFqLy52W1tbws4n5jQ5j9XLP5G");

#[program]
pub mod token_claimer {
    use super::*;
    
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()>{
        let claim_record = &mut ctx.accounts.claim_record;
        let current_time = Clock::get().unwrap().unix_timestamp;

        if claim_record.total_claimed >= 15000 {
            return Err(ProgramError::Custom(0).into()); // Max tokens claimed error
        }

        if claim_record.last_claim_timestamp != 0 &&
           current_time - claim_record.last_claim_timestamp < 5 * 86400 {
            return Err(ProgramError::Custom(1).into()); // Claim too soon error
        }

        let days_since_first_claim = (current_time - claim_record.first_claim_timestamp) / 86400;
        let mut allowed_claim = 0;

        if days_since_first_claim < 5 {
            allowed_claim = 5000;
        } else if days_since_first_claim >= 5 && days_since_first_claim < 10 {
            allowed_claim = 10000;
        } else if days_since_first_claim >= 10 {
            allowed_claim = 15000 - claim_record.total_claimed;  // Ensure max 15000
        }

        if claim_record.total_claimed + allowed_claim > 15000 {
            allowed_claim = 15000 - claim_record.total_claimed;
        }

        if allowed_claim <= 0 {
            return Err(ProgramError::Custom(2).into()); // No tokens left to claim error
        }

        transfer_tokens(
            &ctx.accounts.from, 
            &ctx.accounts.to, 
            &ctx.accounts.authority, 
            &ctx.accounts.token_program, 
            allowed_claim
        )?;

        claim_record.total_claimed += allowed_claim;
        claim_record.last_claim_timestamp = current_time;

        Ok(())
    }
}

#[account]
pub struct ClaimRecord {
    pub last_claim_timestamp: i64,
    pub total_claimed: u64,
    pub first_claim_timestamp: i64,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub claim_record: Account<'info, ClaimRecord>,
    pub from: Account<'info, TokenAccount>,
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

fn transfer_tokens<'a>(
    from: &Account<'a, TokenAccount>,
    to: &Account<'a, TokenAccount>,
    authority: &Signer<'a>,
    token_program: &Program<'a, Token>,
    amount: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, amount)?;
    Ok(())
}
