use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub fee: u16, // 5% fee = 500 basis points
    pub mint: Pubkey,

    pub max_deposit: u64,
    pub max_shares: u64,

    pub shares_bump: u8,
    pub bump: u8,
}
