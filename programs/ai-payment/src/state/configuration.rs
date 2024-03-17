use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Configuration {
    pub bump: u8,
    pub authority: Pubkey, // admin's wallet
    pub token_mint: Pubkey,
    pub paused: bool,
    pub treasury_wallet: Pubkey,
    pub dev_wallet1: Pubkey,
    pub dev_fee1: u16,
    pub dev_wallet2: Pubkey,
    pub dev_fee2: u16,
    pub created_at: u64,
    pub updated_at: u64,
    pub reserved: [u128; 5],
}