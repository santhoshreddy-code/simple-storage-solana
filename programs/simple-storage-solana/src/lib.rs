use anchor_lang::prelude::*;

declare_id!("7uexy9CdAgZseuHWxKWmatP6NajvS5LiHvWzkHyrjy7C");

#[program]
pub mod simple_storage_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.owner = ctx.accounts.user.key();
        storage.stored_number = 0;
        msg!("Storage initialized! Owner: {}", storage.owner);
        Ok(())
    }

    pub fn set_number(ctx: Context<SetNumber>, new_number: u64) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        require!(
            storage.owner == ctx.accounts.user.key(),
            CustomError::NotOwner
        );
        storage.stored_number = new_number;
        msg!("Number updated to: {}", new_number);
        Ok(())
    }

    pub fn get_number(ctx: Context<GetNumber>) -> Result<u64> {
        Ok(ctx.accounts.storage.stored_number)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8
    )]
    pub storage: Account<'info, StorageData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetNumber<'info> {
    #[account(mut)]
    pub storage: Account<'info, StorageData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetNumber<'info> {
    pub storage: Account<'info, StorageData>,
}

#[account]
pub struct StorageData {
    pub owner: Pubkey,       // 32 bytes
    pub stored_number: u64,  // 8 bytes
}

#[error_code]
pub enum CustomError {
    #[msg("Only the owner can change the number")]
    NotOwner,
}