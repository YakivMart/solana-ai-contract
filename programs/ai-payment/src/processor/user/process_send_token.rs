use {
    crate::{constant::*, error::ContractError, state::*, utils::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SendTokenIx {
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(ix: SendTokenIx)]
pub struct SendTokenCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [CONFIG_TAG],
      constraint = configuration.token_mint == token_mint.key() && !configuration.paused @ ContractError::InvalidToken,
      bump
    )]
    pub configuration: Box<Account<'info, Configuration>>,

    /// CHECK: we read this key only
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = authority,
    )]
    pub user_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = configuration.treasury_wallet == treasury_wallet.key() @ ContractError::InvalidAddress
    )]
    pub treasury_wallet: Signer<'info>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = treasury_wallet,
    )]
    pub treasury_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = configuration.dev_wallet1 == dev_wallet1.key() @ ContractError::InvalidAddress
    )]
    pub dev_wallet1: Signer<'info>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = dev_wallet1,
    )]
    pub dev1_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = configuration.dev_wallet2 == dev_wallet2.key() @ ContractError::InvalidAddress
    )]
    pub dev_wallet2: Signer<'info>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = dev_wallet2,
    )]
    pub dev2_token_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<SendTokenCtx>, ix: SendTokenIx) -> Result<()> {
    let dev_fee1 = ctx.accounts.configuration.dev_fee1 as u64;
    let dev_fee2 = ctx.accounts.configuration.dev_fee2 as u64;

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_vault.to_account_info(),
                to: ctx.accounts.treasury_token_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        ix.amount.safe_mul(TOTAL_PERCENT.safe_sub(dev_fee1 + dev_fee2).unwrap()).unwrap().safe_div(TOTAL_PERCENT).unwrap(),
    )?;

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_vault.to_account_info(),
                to: ctx.accounts.dev1_token_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        ix.amount.safe_mul(dev_fee1).unwrap().safe_div(TOTAL_PERCENT).unwrap(),
    )?;

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_vault.to_account_info(),
                to: ctx.accounts.dev2_token_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        ix.amount.safe_mul(dev_fee2).unwrap().safe_div(TOTAL_PERCENT).unwrap(),
    )?;

    Ok(())
}
