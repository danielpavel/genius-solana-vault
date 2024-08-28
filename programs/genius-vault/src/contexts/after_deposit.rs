use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenInterface, TokenAccount}, 
    associated_token::AssociatedToken,
};
use solana_program::program::invoke_signed;

use crate::{
    constants::{ATA_POOL_VAULT, GENIUS_POOL_CONTRACT, POOL_VAULT, USDC_ADDRESS}, stake_liquidity::{StakeLiquidity, StakeLiquidityArgs}, VaultConfig};

#[derive(Accounts)]
pub struct LiquidityTransfer<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        address = USDC_ADDRESS
    )]
    mint: InterfaceAccount<'info, Mint>,

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
        seeds = [b"config".as_ref(), mint.key().as_ref()],
        bump = config.bump
    )]
    config: Account<'info, VaultConfig>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = POOL_VAULT,
    )]
    pool_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = ATA_POOL_VAULT
    )]
    ata_pool_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = shares_mint,
        associated_token::authority = user,
    )]
    user_shares_ata: InterfaceAccount<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,

    #[account(
        address = GENIUS_POOL_CONTRACT
    )]
    /// CHECK: This is the Genius Pool Program account
    genius_pool_program: AccountInfo<'info>
}

impl<'info> LiquidityTransfer<'info> {
    pub fn after_deposit(&mut self, amount: u64) -> Result<()> {

        let cpi_program = self.genius_pool_program.to_account_info();
        let cpi_accounts = StakeLiquidity {
            user: self.user.clone(),
            ata_user: self.vault.clone(),
            ata_vault: self.ata_pool_vault.clone(),
            vault: self.pool_vault.clone(),
            shares_mint: self.shares_mint.clone(),
            user_shares_ata: self.user_shares_ata.clone(),
            usdc_mint: self.mint.clone(),
            token_program: self.token_program.clone(),
            associated_token_program: self.associated_token_program.clone(),
            system_program: self.system_program.clone(),
        };

        let params = StakeLiquidityArgs {
            amount
        };

        let ix = solana_program::instruction::Instruction {
            program_id: cpi_program.key(),
            accounts: cpi_accounts
                .to_account_metas(None)
                .into_iter()
                .zip(cpi_accounts.to_account_infos())
                .map(|mut pair| {
                    pair.0.is_signer = pair.1.is_signer;
                    if pair.0.pubkey == self.config.to_account_info().key() {
                        pair.0.is_signer = true;
                    }
                    pair.0
                })
            .collect(),
            data: params.try_to_vec().unwrap(),
        };

        let bump = [self.config.bump];
        let signer_seeds = [&[
            b"config",
            self.mint.to_account_info().key.as_ref(),
            &bump
        ][..]];

        invoke_signed(&ix, &cpi_accounts.to_account_infos(), &signer_seeds)?;

        Ok(())
    }
}
