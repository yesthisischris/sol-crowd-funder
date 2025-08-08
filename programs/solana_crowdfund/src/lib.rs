// Import the Anchor framework — this brings in macros, traits, and helpers for Solana smart contracts.
use anchor_lang::prelude::*;

// This is the unique identifier for the smart contract (program) on Solana.
// In development it’s a placeholder; after `anchor deploy` you’ll replace it
// with your real, generated Program ID (from `anchor keys list`).
declare_id!("11111111111111111111111111111111");

/// The #[program] module tells Anchor: “these are the instruction handlers”.
/// Think of each function inside as a transaction users can send.
#[program]
pub mod solana_crowdfund {
    // Bring everything from parent scope into this module (types, macros, etc.)
    use super::*;

    /// Create a brand-new crowdfunding campaign.
    ///
    /// - `ctx`: carries all accounts this instruction touches/validates.
    /// - `goal_lamports`: target in lamports (1 SOL = 1_000_000_000 lamports).
    /// - `deadline_unix`: unix timestamp when the campaign ends.
    pub fn initialize_campaign(
        ctx: Context<InitializeCampaign>,
        goal_lamports: u64,
        deadline_unix: i64,
    ) -> Result<()> {
        // Safety check: deadlines must be in the future (cluster time, not local PC time).
        require!(
            deadline_unix > Clock::get()?.unix_timestamp,
            CrowdfundError::InvalidDeadline
        );

        // Grab a mutable reference to the newly created on-chain campaign account
        // so we can write initial values into it.
        let c = &mut ctx.accounts.campaign;

        // Initialize the campaign fields. These persist on chain.
        c.creator = ctx.accounts.creator.key();  // who owns/created this campaign
        c.goal_lamports = goal_lamports;         // how much we aim to raise
        c.deadline_unix = deadline_unix;         // when the campaign ends
        c.total_raised = 0;                      // start at zero raised

        // Indicate success (no return value beyond Ok(())).
        Ok(())
    }

    /// Allow any wallet to back the campaign with SOL.
    ///
    /// - `amount`: how many lamports the backer wants to contribute.
    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        // Enforce campaign is still active based on cluster time.
        let now = Clock::get()?.unix_timestamp;
        require!(
            now < ctx.accounts.campaign.deadline_unix,
            CrowdfundError::CampaignEnded
        );

        // Build a System Program transfer instruction to move lamports from
        // the backer to the campaign’s vault PDA (a program-owned system account).
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.backer.key(),         // from: the backer
            &ctx.accounts.campaign_vault.key(), // to: the PDA vault
            amount,                             // lamports to transfer
        );

        // Actually invoke the System Program. This performs the lamports move.
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.backer.to_account_info(),
                ctx.accounts.campaign_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?; // `?` bubbles up any error to the runtime.

        // Update/record this backer’s total contribution to this campaign.
        // This account is unique per (campaign, backer) pair.
        let b = &mut ctx.accounts.backer_state;
        b.backer = ctx.accounts.backer.key();
        b.campaign = ctx.accounts.campaign.key();
        b.amount = b.amount.checked_add(amount).unwrap(); // checked math to avoid overflow

        // Update aggregate raised amount on the campaign.
        ctx.accounts.campaign.total_raised = ctx.accounts.campaign
            .total_raised
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    /// Allow the campaign creator to withdraw **only if** the goal was met.
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        // Shorthand reference to the campaign data.
        let c = &ctx.accounts.campaign;

        // Enforce success criteria: raised >= goal.
        require!(c.total_raised >= c.goal_lamports, CrowdfundError::GoalNotMet);

        // Ensure the signer is the original creator (authorization).
        require_keys_eq!(c.creator, ctx.accounts.creator.key(), CrowdfundError::NotCreator);

        // Move all lamports from the vault PDA to the creator.
        // Here we adjust lamport balances directly — a common low-level pattern.
        let vault_lamports = **ctx.accounts.campaign_vault.lamports.borrow();
        **ctx.accounts.campaign_vault.lamports.borrow_mut() = 0;
        **ctx.accounts.creator.lamports.borrow_mut() = ctx
            .accounts
            .creator
            .lamports()
            .checked_add(vault_lamports)
            .unwrap();

        Ok(())
    }

    /// Allow a backer to claim a refund **only if** the deadline passed
    /// and the goal was **not** reached.
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let c = &ctx.accounts.campaign;

        // Refunds open only after deadline…
        require!(now >= c.deadline_unix, CrowdfundError::NotExpired);
        // …and only if the campaign failed to meet its goal.
        require!(c.total_raised < c.goal_lamports, CrowdfundError::GoalMet);

        // How much this specific backer contributed to this specific campaign.
        let amt = ctx.accounts.backer_state.amount;
        require!(amt > 0, CrowdfundError::NothingToRefund);

        // Subtract from the vault and credit the backer.
        **ctx.accounts.campaign_vault.lamports.borrow_mut() = ctx
            .accounts
            .campaign_vault
            .lamports()
            .checked_sub(amt)
            .unwrap();

        **ctx.accounts.backer.lamports.borrow_mut() = ctx
            .accounts
            .backer
            .lamports()
            .checked_add(amt)
            .unwrap();

        // Prevent double refunds by zeroing out their recorded amount.
        ctx.accounts.backer_state.amount = 0;

        Ok(())
    }
}

/// The structs below define which accounts each instruction needs,
/// how they’re derived, who pays for them, and which constraints Anchor enforces.

/// Accounts for creating a campaign.
#[derive(Accounts)]
pub struct InitializeCampaign<'info> {
    /// The wallet starting the campaign (must sign; may pay rent for new accounts).
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The on-chain data account that stores campaign details.
    /// We create it deterministically as a Program Derived Address (PDA).
    #[account(
        init,                                                    // create this account
        payer = creator,                                         // creator pays rent
        space = 8 + 32 + 8 + 8 + 8,                              // discriminator + fields
        seeds = [b"campaign", creator.key().as_ref()],           // PDA seed recipe
        bump                                                     // Anchor will find the bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// The “vault” PDA (a system account owned by this program) that holds SOL.
    #[account(
        mut,                                                     // will receive SOL
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe because we only store SOL (lamports) here and don’t deserialize data.
    pub campaign_vault: AccountInfo<'info>,

    /// The Solana System Program: needed for creating accounts and transferring SOL.
    pub system_program: Program<'info, System>,
}

/// Accounts for contributing to a campaign.
#[derive(Accounts)]
pub struct Contribute<'info> {
    /// The backer sending SOL (must sign; their balance will decrease).
    #[account(mut)]
    pub backer: Signer<'info>,

    /// The target campaign (we mutate to update `total_raised`).
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// Per-backer record for this campaign (tracks the total contributed).
    #[account(
        init_if_needed,                                          // create on first contribution
        payer = backer,                                          // backer pays rent
        space = 8 + 32 + 32 + 8,                                 // discriminator + fields
        seeds = [b"backer", campaign.key().as_ref(), backer.key().as_ref()],
        bump
    )]
    pub backer_state: Account<'info, BackerState>,

    /// The vault PDA that receives SOL.
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    /// CHECK: Only lamports storage/transfer; no custom data.
    pub campaign_vault: AccountInfo<'info>,

    /// System program (required for SOL transfers).
    pub system_program: Program<'info, System>,
}

/// Accounts for withdrawing raised funds after success.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// The campaign creator (must match `campaign.creator`).
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The campaign state account.
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// The vault to drain into the creator’s wallet.
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    /// CHECK: Only lamports manipulation.
    pub campaign_vault: AccountInfo<'info>,
}

/// Accounts for getting a refund after failure.
#[derive(Accounts)]
pub struct Refund<'info> {
    /// The backer receiving the refund.
    #[account(mut)]
    pub backer: Signer<'info>,

    /// The campaign that failed to reach its goal.
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    /// This backer’s contribution record (we zero it after refund).
    #[account(
        mut,
        seeds = [b"backer", campaign.key().as_ref(), backer.key().as_ref()],
        bump
    )]
    pub backer_state: Account<'info, BackerState>,

    /// The vault holding the contributed SOL.
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump
    )]
    /// CHECK: Only lamports manipulation.
    pub campaign_vault: AccountInfo<'info>,
}

/// On-chain data representing a campaign.
#[account] // Anchor adds an 8-byte discriminator and handles (de)serialization.
pub struct Campaign {
    pub creator: Pubkey,        // who created the campaign (32 bytes)
    pub goal_lamports: u64,     // target amount in lamports (8 bytes)
    pub deadline_unix: i64,     // end timestamp (8 bytes, signed i64)
    pub total_raised: u64,      // how many lamports raised so far (8 bytes)
}

/// Tracks how much a particular backer gave to a particular campaign.
#[account]
pub struct BackerState {
    pub backer: Pubkey,         // backer’s public key (32 bytes)
    pub campaign: Pubkey,       // campaign public key (32 bytes)
    pub amount: u64,            // total contributed in lamports (8 bytes)
}

/// Custom errors returned by our program. Anchor maps these to readable messages.
#[error_code]
pub enum CrowdfundError {
    #[msg("Deadline must be in the future")]
    InvalidDeadline,
    #[msg("Campaign has ended")]
    CampaignEnded,
    #[msg("Only the creator can withdraw")]
    NotCreator,
    #[msg("Goal not met")]
    GoalNotMet,
    #[msg("Campaign not expired")]
    NotExpired,
    #[msg("Goal met; refunds disabled")]
    GoalMet,
    #[msg("Nothing to refund")]
    NothingToRefund,
}
