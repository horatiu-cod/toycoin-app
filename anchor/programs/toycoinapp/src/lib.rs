#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Horiz3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod counter {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateJournalEntry>,
        title: String,
        content: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.user.key;
        journal_entry.title = title;
        journal_entry.content = content;

        Ok(())
    }

    pub fn update_journal_entry(
        ctx: Context<UpdateJournalEntry>,
        title: String,
        new_content: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.content = new_content;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
    #[account(
        init, 
        seeds = [b"journal_entry", owner.key().as_ref(), title.as_bytes()],
        bump,
        payer = owner, 
        space = JournalEntryState::INIT_SPACE + 8
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
    #[account(
    mut,
    has_one = owner,
    seeds = [b"journal_entry", owner.key().as_ref(), journal_entry.title.as_bytes()],
    bump,
    realoc = JournalEntryState::INIT_SPACE + 8,
    realoc::payer = owner,
    realoc::zero = true)]

    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub sub JournalEntryState{
    pub owner: Pubkey,
    #[max_length = 50]
    pub title: String,
    #[max_length = 500]
    pub content: String,
}
