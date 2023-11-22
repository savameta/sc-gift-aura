use anchor_lang::prelude::*;

declare_id!("BtYdTY7KuPGVNDvCvPB9UCc5EaurRrUQ4jGW2qqpmVff");

#[program]
pub mod gift_aura {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
