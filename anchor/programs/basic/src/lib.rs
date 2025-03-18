use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("73QNc754LR5d8DiWFPWxyqMDUYkWvHni9NiHV16RHp26");

#[program]
pub mod proposal_investment {
    use super::*;

    /// Initializes the Config account with the company owner.
    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = owner;
        Ok(())
    }

    /// Anyone can create a new proposal.
    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.creator = ctx.accounts.creator.key();
        proposal.description = description;
        proposal.status = ProposalStatus::Pending;
        proposal.total_invested = 0;
        proposal.rewards_distributed = false;
        Ok(())
    }

    /// Anyone can invest in an existing proposal.
    /// The investor sends `amount` lamports which are transferred into the proposal account (acting as escrow).
    /// A separate Investment account is created to log the investment.
    pub fn invest(ctx: Context<Invest>, amount: u64) -> Result<()> {
        // Record the investment details.
        let proposal = &mut ctx.accounts.proposal;
        let investment = &mut ctx.accounts.investment;
        investment.proposal = proposal.key();
        investment.investor = ctx.accounts.investor.key();
        investment.amount = amount;

        // Update the proposal's total invested funds.
        proposal.total_invested = proposal
            .total_invested
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        // Transfer lamports from the investor to the proposal account.
        let ix = system_instruction::transfer(
            &ctx.accounts.investor.key(),
            &proposal.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.investor.to_account_info(),
                ctx.accounts.proposal.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    /// Only the company owner (via the Config account) can accept a proposal.
    pub fn accept_proposal(ctx: Context<UpdateProposalStatus>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.status = ProposalStatus::Accepted;
        Ok(())
    }

    /// Only the company owner can reject a proposal.
    pub fn reject_proposal(ctx: Context<UpdateProposalStatus>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.status = ProposalStatus::Rejected;
        Ok(())
    }

    /// Distributes revenue rewards for a proposal.
    ///
    /// The company owner (or oracle acting on their behalf) supplies the total revenue amount
    /// that is held in the provided reward vault account. Then the instruction expects the remaining
    /// accounts to be provided in pairs: [investment account, corresponding investor wallet account].
    /// For each investment linked to this proposal, a reward is computed as:
    ///
    ///   reward = (investment.amount / proposal.total_invested) * revenue_amount
    ///
    /// and the vault transfers that many lamports to the investor.
    pub fn distribute_rewards(ctx: Context<DistributeRewards>, revenue_amount: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        // Only allow reward distribution if the proposal was accepted.
        require!(
            proposal.status == ProposalStatus::Accepted,
            ErrorCode::InvalidProposalStatus
        );
        // Prevent double distribution.
        require!(
            !proposal.rewards_distributed,
            ErrorCode::RewardsAlreadyDistributed
        );
        // Check that the vault has enough funds.
        let vault_balance = ctx.accounts.reward_vault.lamports();
        require!(
            vault_balance >= revenue_amount,
            ErrorCode::InsufficientVaultBalance
        );

        let total_invested = proposal.total_invested;
        // Expecting remaining_accounts to be provided in pairs:
        // [investment account, investor wallet account] for each investment related to this proposal.
        let accounts = &ctx.remaining_accounts;
        let mut i = 0;
        while i < accounts.len() {
            let investment_info = &accounts[i];
            let investor_wallet_info = &accounts[i + 1];

            // Verify program ownership
            require!(investment_info.owner == ctx.program_id, ErrorCode::InvalidOwner);
            
            // Deserialize the investment data directly
            let investment_data = investment_info.try_borrow_data()?;
            let investment = Investment::try_deserialize(&mut &investment_data[8..])?;

            // Ensure this investment belongs to the proposal.
            if investment.proposal != proposal.key() {
                i += 2;
                continue;
            }

            // Calculate the investor's share proportionally.
            let investor_share = (investment.amount as u128)
                .checked_mul(revenue_amount as u128)
                .unwrap()
                .checked_div(total_invested as u128)
                .unwrap() as u64;

            // Transfer lamports from the reward vault to the investor's wallet.
            **ctx.accounts.reward_vault.try_borrow_mut_lamports()? -= investor_share;
            **investor_wallet_info.try_borrow_mut_lamports()? += investor_share;

            i += 2;
        }
        proposal.rewards_distributed = true;
        Ok(())
    }
}

/// Config account holds the company owner.
#[account]
pub struct Config {
    pub owner: Pubkey,
}

impl Config {
    // Size: 32 bytes for the Pubkey.
    pub const LEN: usize = 32;
}

/// Proposal account holds the proposal data.
#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub description: String,         // Maximum 200 characters (UTF-8)
    pub status: ProposalStatus,      // 1 byte (enum)
    pub total_invested: u64,         // 8 bytes
    pub rewards_distributed: bool,   // 1 byte
}

impl Proposal {
    // Calculation for space:
    // 32 (creator) + 4 (string length prefix) + 200 (max description) + 1 (status) + 8 (total_invested) + 1 (bool)
    pub const LEN: usize = 32 + 4 + 200 + 1 + 8 + 1;
}

/// Investment account records an individual investment.
#[account]
pub struct Investment {
    pub proposal: Pubkey,
    pub investor: Pubkey,
    pub amount: u64,
}

impl Investment {
    // 32 (proposal) + 32 (investor) + 8 (amount)
    pub const LEN: usize = 32 + 32 + 8;
}

/// Enum for the proposal status.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
}

/// Context for initializing the Config account.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + Config::LEN)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for creating a proposal.
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = 8 + Proposal::LEN)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for investing in a proposal.
#[derive(Accounts)]
pub struct Invest<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(init, payer = investor, space = 8 + Investment::LEN)]
    pub investment: Account<'info, Investment>,
    #[account(mut)]
    pub investor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for updating a proposal's status (accept/reject).
#[derive(Accounts)]
pub struct UpdateProposalStatus<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut, has_one = owner)]
    pub config: Account<'info, Config>,
    #[account(signer)]
    /// CHECK: The owner is verified via the Config account.
    pub owner: AccountInfo<'info>,
}

/// Context for distributing revenue rewards.
#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: Vault account holding revenue funds (trusted/oracle fed)
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,
    #[account(signer)]
    /// CHECK: The owner is verified via the Config account.
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // The remaining accounts are expected to be provided in pairs:
    // [Investment account, corresponding investor wallet account] for each investment.
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred.")]
    Overflow,
    #[msg("Proposal status does not allow this operation.")]
    InvalidProposalStatus,
    #[msg("Rewards have already been distributed for this proposal.")]
    RewardsAlreadyDistributed,
    #[msg("The reward vault does not have enough funds.")]
    InsufficientVaultBalance,
    #[msg("Invalid owner for the investment account.")]
    InvalidOwner,
}