use anchor_lang::prelude::*;

declare_id!("Fixr1111111111111111111111111111111111111111");

#[program]
pub mod fixr_core {
    use super::*;
    pub fn ping(_ctx: Context<Ping>) -> Result<()> { Ok(()) }
}

#[derive(Accounts)]
pub struct Ping {}
