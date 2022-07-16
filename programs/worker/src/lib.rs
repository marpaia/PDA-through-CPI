use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod worker {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, value: u64) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.value = value;
        Ok(())
    }
}

#[account]
pub struct Data {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"worker", user.key().as_ref()], bump)]
    pub data: Account<'info, Data>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub data: Account<'info, Data>,
}
