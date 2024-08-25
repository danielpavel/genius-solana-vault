use anchor_lang::prelude::*;

declare_id!("FnpUgSxeMQJAGYADfem4Zz7gf9i2sPJjYiK9ay8VBoJm");

pub mod contexts;
pub mod errors;
pub mod state;

pub use contexts::*;

#[program]
pub mod genius_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
}

