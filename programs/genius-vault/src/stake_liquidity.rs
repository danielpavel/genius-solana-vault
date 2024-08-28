use anchor_lang::prelude::*;
use anchor_spl::{ 
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, Mint, TokenInterface}
};

pub use crate::constants::{ SHARES_MINT_ADDRESS, USDC_ADDRESS, VAULT_SEED };

#[derive(Accounts, Clone)]
#[instruction(
    amount: u64,
)]
pub struct StakeLiquidity<'info> {
    pub user: Signer<'info>,

    //  destination token account of user
    #[account(
        mut,
        associated_token::mint = usdc_mint, 
        associated_token::authority = user,
    )]
    pub ata_user: InterfaceAccount<'info, TokenAccount>,

    //  USDC ata of vault
    #[account(
        mut, 
        associated_token::mint = usdc_mint, 
        //  Authority is set to vault
        associated_token::authority = vault,
    )]
    pub ata_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = SHARES_MINT_ADDRESS,
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = shares_mint,
        associated_token::authority = user,
    )]
    pub user_shares_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(	
        seeds = [VAULT_SEED],	
        bump,	
    )]	
    /// CHECK: This is not dangerous because we don't read or write from this account	
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // The mint of $USDC because it's needed from above ⬆ token::mint = ...
    #[account(
        address = USDC_ADDRESS,
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StakeLiquidityArgs {
    pub amount: u64,
}
