use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenInterface, TokenAccount, TransferChecked, transfer_checked, MintTo, mint_to}, 
    associated_token::AssociatedToken
};

use crate::VaultConfig;

use crate::errors::GeniusVaultErrors;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    user: Signer<'info>,

    mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"config".as_ref(), mint.key().as_ref()],
        bump = config.bump
    )]
    config: Account<'info, VaultConfig>,

    #[account(
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds = [b"shares", config.key().as_ref()],
        bump = config.shares_bump
    )]
    shares_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_mint_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = shares_mint,
        associated_token::authority = user,
    )]
    user_shares_ata: InterfaceAccount<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount < self.config.max_deposit, GeniusVaultErrors::ExceededMaxDeposit);

        self.deposit_tokens(amount)?;
        self.mint_shares(amount)
    }

    pub fn deposit_tokens(&mut self, amount: u64) -> Result<()> {
        let accounts = TransferChecked {
            from: self.user_mint_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_context, amount, self.mint.decimals)
    }

    pub fn mint_shares(&mut self, amount: u64) -> Result<()> {
        let total_shares = self.shares_mint.supply;
        let total_supply = self.vault.amount;

        let shares_to_mint = match total_shares == 0 && total_supply == 0 {
            true => amount,
            false => amount
                .checked_mul(total_supply).ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(total_shares).ok_or(ProgramError::ArithmeticOverflow)?
        };

        require!(shares_to_mint < self.config.max_shares, GeniusVaultErrors::ExceededMaxMint);

        let bump = [self.config.bump];
        let signer_seeds = [&[
            b"config",
            self.mint.to_account_info().key.as_ref(),
            &bump
        ][..]];

        let accounts = MintTo {
            mint: self.shares_mint.to_account_info(),
            to: self.user_shares_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            &signer_seeds
        );

        mint_to(cpi_context, shares_to_mint)
    }
}
