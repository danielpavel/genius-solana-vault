use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

pub use crate::state::VaultConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        space = 8 + VaultConfig::INIT_SPACE,
        seeds = [b"config".as_ref(), mint.key().as_ref()],
        bump
    )]
    config: Account<'info, VaultConfig>,

    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"shares", config.key().as_ref()],
        bump

    )]
    shares_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner(VaultConfig {
            fee: 300, // 3% fee = 300 basis points
            mint: self.mint.to_account_info().key(),
            max_deposit: u64::MAX,
            max_shares: u128::MAX,
            bump: bumps.config,
            shares_bump: bumps.shares_mint,
        });

        Ok(())
    }
}
