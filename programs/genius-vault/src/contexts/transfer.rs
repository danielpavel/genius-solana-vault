use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenInterface, TokenAccount, TransferChecked, transfer_checked, MintTo, mint_to, Burn, burn}, 
    associated_token::AssociatedToken,
};

use crate::VaultConfig;

use crate::errors::GeniusVaultErrors;

#[derive(Accounts)]
pub struct Transfer<'info> {
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

impl<'info> Transfer<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount < self.config.max_deposit, GeniusVaultErrors::ExceededMaxDeposit);

        self.transfer_tokens(amount, true)?;

        let total_shares = self.shares_mint.supply;
        let total_supply = self.vault.amount;

        let shares_to_mint = match total_shares == 0 && total_supply == 0 {
            true => amount,
            false => amount
                .checked_mul(total_supply).ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(total_shares).ok_or(ProgramError::ArithmeticOverflow)?
        };

        self.mint_shares(shares_to_mint)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(amount < self.config.max_shares, GeniusVaultErrors::ExceededMaxBurn);

        self.burn_shares(amount)?;

        let amount_to_withdraw = amount
            .checked_mul(self.vault.amount).ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(self.shares_mint.supply).ok_or(ProgramError::ArithmeticOverflow)?;

        self.transfer_tokens(amount_to_withdraw, false)
    }

    pub fn transfer_tokens(&mut self, amount: u64, to_vault: bool) -> Result<()> {
        let (from, to, authority) = match to_vault {
            false => (
                self.user_mint_ata.to_account_info(),
                self.vault.to_account_info(),
                self.user.to_account_info()
                ),
            true => (
                self.vault.to_account_info(),
                self.user_mint_ata.to_account_info(),
                self.config.to_account_info()
                )
        };

        let accounts = TransferChecked {
            from,
            to,
            mint: self.mint.to_account_info(),
            authority,
        };

        if to_vault {
            let mint = self.mint.to_account_info();
            let bump = [self.config.bump];
            let signer_seeds = [&[
                b"config",
                mint.key.as_ref(),
                &bump,
            ][..]];

            transfer_checked(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    accounts,
                    &signer_seeds),
                    amount,
                    self.mint.decimals)?;
        } else {
            transfer_checked(CpiContext::new(self.token_program.to_account_info(), accounts), amount, self.mint.decimals)?;
        };

        Ok(())
    }

    pub fn burn_shares(&mut self, amount: u64) -> Result<()> {
        let bump = [self.config.bump];
        let signer_seeds = [&[
            b"config",
            self.mint.to_account_info().key.as_ref(),
            &bump
        ][..]];

        let accounts = Burn {
            mint: self.shares_mint.to_account_info(),
            from: self.user_shares_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds
            );

        burn(cpi_context, amount)
    }

    pub fn mint_shares(&mut self, amount: u64) -> Result<()> {
        require!(amount < self.config.max_shares, GeniusVaultErrors::ExceededMaxMint);

        let bump = [self.config.bump];
        let signer_seeds = [&[
            b"config",
            self.mint.to_account_info().key.as_ref(),
            &bump
        ][..]];

        let accounts = MintTo {
            mint: self.shares_mint.to_account_info(),
            to: self.user_shares_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            &signer_seeds
            );

        mint_to(cpi_context, amount)
    }
}
