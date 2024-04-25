use anchor_lang::prelude::*;

declare_id!("E2dMh4oEcsPJqfy13vdFqLy52W1tbws4n5jQ5j9XLP5G");

#[program]
pub mod token_claimer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
