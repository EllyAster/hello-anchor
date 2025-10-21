use anchor_lang::prelude::*;

declare_id!("Bpn8fqtx7rf2YypAfYtrZPFcUboqccy8krZ6GJrzRTas");

#[program]
pub mod hello_anchor {
    use super::*;



    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
