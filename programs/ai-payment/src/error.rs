
use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("Calculation Error.")]
    CalcError,
    
    #[msg("Invalid address.")]
    InvalidAddress,
    
    #[msg("Invalid authority.")]
    InvalidAuthority,

    #[msg("Invalid Token.")]
    InvalidToken,

    #[msg("MathOverflow.")]
    MathOverflow,
}
