use anchor_lang::prelude::*;

declare_id!("HRN8wvPUctTjQB7vgD6wJiURXDdFuAL5yKj1Nvw39CBH");

/// constant
pub mod constant;
/// error
pub mod error;
/// processor
pub mod processor;
/// states
pub mod state;
/// utils
pub mod utils;

use crate::processor::*;

#[program]
pub mod ai_payment {
    use super::*;

    // admin 
    pub fn initialize(ctx: Context<InitializeCtx>, ix: InitializeIx) -> Result<()> {
        process_intialize::handler(ctx, ix)
    }

    pub fn update(ctx: Context<UpdateCtx>, ix: UpdateIx) -> Result<()> {
        process_update::handler(ctx, ix)
    }

    // user
    pub fn send_token(ctx: Context<SendTokenCtx>, ix: SendTokenIx) -> Result<()> { // unlock the deal by paying fee(fee -> contract, whitelister)
        process_send_token::handler(ctx, ix)
    }
}
