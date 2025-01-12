use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{ self, Data };
declare_id!("5onw2hp6bWiKheSufVHiChyTohoiU9jRb1JZGc9e9iQz");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, bump: u8, data: u64) -> Result<()> {
        let bump = &[bump][..];
        puppet::cpi::set_data(ctx.accounts.set_data_ctx().with_signer(&[&[bump]]), data)?;
        //without reloading, the 'puppet would remain'.
        //this is because the 'puppet' in the ctx is detached from
        //the underlying account after the type deseriablization
        ctx.accounts.puppet.reload()?;
        if ctx.accounts.puppet.data != 42 {
            panic!("data not updated");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    // the writable is required, this is part of the 'privilege extension'
    // so does the authority
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
    /// CHECK: used as a PDA
    pub authority: UncheckedAccount<'info>,
}
impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
