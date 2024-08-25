use anchor_lang::error_code;

#[error_code]
pub enum GeniusVaultErrors {
    #[msg("Attempted to deposit more assets than the max amount for `receiver`.")]
    ExceededMaxDeposit,
    #[msg("Attempted to mint more shares than the max amount for `receiver`.")]
    ExceededMaxMint,
    #[msg("Attempted to withdraw more assets than the max amount for `receiver`.")]
    ExceededMaxWithdraw,
    #[msg("Attempted to redeem more shares than the max amount for `receiver`.")]
    ExceededMaxRedeem,
}
