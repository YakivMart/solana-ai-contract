use {
    crate::{constant::*, error::ContractError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateIx {
    pub dev_fee1: u16,
    pub dev_fee2: u16,
    pub paused: bool,
}

#[derive(Accounts)]
#[instruction(ix: UpdateIx)]
pub struct UpdateCtx<'info> {
    #[account(
        mut,
        constraint = configuration.authority == authority.key() @ ContractError::InvalidAuthority,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
      seeds = [CONFIG_TAG],
      bump,
    )]
    pub configuration: Box<Account<'info, Configuration>>,

    /// CHECK: we read this key only
    pub new_authority: AccountInfo<'info>,

    /// CHECK: we read this key only
    pub token_mint: Account<'info, Mint>,

    /// CHECK: we read this key only
    pub treasury_wallet: AccountInfo<'info>,

    /// CHECK: we read this key only
    pub dev_wallet1: AccountInfo<'info>,

    /// CHECK: we read this key only
    pub dev_wallet2: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<UpdateCtx>, ix: UpdateIx) -> Result<()> {
    let configuration = &mut ctx.accounts.configuration;

    // TODO: check given values and save it
    configuration.authority = ctx.accounts.new_authority.key();
    configuration.token_mint = ctx.accounts.token_mint.key();
    configuration.paused = ix.paused;
    configuration.treasury_wallet = ctx.accounts.treasury_wallet.key();
    configuration.dev_wallet1 = ctx.accounts.dev_wallet1.key();
    configuration.dev_fee1 = ix.dev_fee1;
    configuration.dev_wallet2 = ctx.accounts.dev_wallet2.key();
    configuration.dev_fee2 = ix.dev_fee2;
    configuration.updated_at = ctx.accounts.clock.unix_timestamp as u64;

    Ok(())
}
