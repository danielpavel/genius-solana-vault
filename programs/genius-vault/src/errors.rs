use anchor_lang::error_code;

#[error_code]
pub enum GeniusVaultErrors {
    #[msg("Attempted to deposit more assets than the max amount.")]
    ExceededMaxDeposit,
    #[msg("Attempted to mint more shares than the max amount.")]
    ExceededMaxMint,
    #[msg("Attempted to withdraw more assets than the max amount.")]
    ExceededMaxWithdraw,
    #[msg("Attempted to burn more shares than the max amount.")]
    ExceededMaxBurn,
}
