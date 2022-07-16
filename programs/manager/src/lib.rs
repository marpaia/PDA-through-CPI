use anchor_lang::prelude::*;
use worker::cpi::accounts::SetData;
use worker::program::Worker;
use worker::{self, Data};

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
mod manager {
    use super::*;

    pub fn set_worker_data_through_cpi(ctx: Context<SetWorkerDataThroughCPI>, data: u64) -> Result<()> {
        let cpi_program = ctx.accounts.worker_program.to_account_info();
        let cpi_accounts = SetData {
            data: ctx.accounts.data.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        worker::cpi::set_data(cpi_ctx, data)
    }
}

#[derive(Accounts)]
pub struct SetWorkerDataThroughCPI<'info> {
    #[account(mut)]
    pub data: Account<'info, Data>,
    pub worker_program: Program<'info, Worker>,
}
