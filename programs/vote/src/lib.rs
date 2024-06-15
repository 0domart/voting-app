use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("HzuXSkskFx6Ai1NNP8Q42yXFTE6nuh8ZCPQB8ctamJqz");

#[program]
pub mod voting {
    use super::*;

    pub fn create_vote(
        ctx: Context<CreateVote>,
        topic: String,
        options: Vec<String>,
        voting_days: i32,
    ) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.topic = topic;
        vote_account.voting_deadline = Clock::get()?.unix_timestamp + (voting_days * 86400) as i64;
        vote_account.options_count = options.len() as u8;
        vote_account.options = options
            .into_iter()
            .map(|opt| VoteOption {
                name: opt,
                votes: 0,
            }).collect();
        Ok(())
    }

    // Vote for a specific option
    pub fn vote(ctx: Context<CastVote>, option_index: u8) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        require!(
            Clock::get()?.unix_timestamp < vote_account.voting_deadline,
            VotingErr::VotingIsOver
        );
        require!(
            option_index < vote_account.options.len() as u8,
            VotingErr::InvalidOption
        );

        let voter = &mut ctx.accounts.voter_account;
        require!(!voter.voted, VotingErr::AlreadyVoted);
        voter.voted = true;

        vote_account.options[option_index as usize].votes += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    #[account(init, payer = user, space = VoteAccount::LEN)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(init, payer = user, space = Voter::LEN, seeds = [vote_account.key().as_ref(), user.key().as_ref()], bump)]
    pub voter_account: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VoteAccount {
    pub topic: String,
    pub voting_deadline: i64,
    pub options_count: u8,
    pub options: Vec<VoteOption>,
}

impl VoteAccount {
    const LEN: usize = 8 + 32 + 8 + 1 + (32 + 8) * 10;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VoteOption {
    pub name: String,
    pub votes: u64,
}

#[account]
pub struct Voter {
    pub voted: bool,
}

impl Voter {
    const LEN: usize = 8 + 1;
}

#[error_code]
pub enum VotingErr {
    #[msg("Voting period is over!")]
    VotingIsOver,
    #[msg("You have already voted!")]
    AlreadyVoted,
    #[msg("Invalid option!")]
    InvalidOption,
}