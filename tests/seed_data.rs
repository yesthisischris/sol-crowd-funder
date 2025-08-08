use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_client::solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};
use anchor_client::{Client, Cluster};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use solana_crowdfund::accounts as crowdfund_accounts;
use solana_crowdfund::instruction as crowdfund_ix;

fn sol(amount: f64) -> u64 {
    (amount * 1_000_000_000.0) as u64
}

#[tokio::test]
async fn seed_sample_data() {
    // === 1) Setup local client ===
    let program_id = solana_crowdfund::id();
    let url = Cluster::Localnet; // Change to Cluster::Devnet to seed devnet (requires deployed program)
    let payer = Rc::new(Keypair::new());
    let client = Client::new_with_options(url, payer.clone(), CommitmentConfig::processed());
    let program = client.program(program_id);

    // === 2) Fund payer (localnet auto-funds in anchor test, but airdrop manually if needed) ===
    // program.rpc().request_airdrop(&payer.pubkey(), sol(10)).unwrap();

    // === 3) Create multiple campaigns ===
    let mut creators = vec![];
    for _i in 0..3 {
        let creator = Keypair::new();
        creators.push(creator);
    }

    // Airdrop to creators
    for c in &creators {
        program.rpc().request_airdrop(&c.pubkey(), sol(5)).unwrap();
    }

    // === 4) For each creator, initialize a campaign with a unique goal/deadline ===
    for (idx, creator) in creators.iter().enumerate() {
        let (campaign_pda, _bump1) =
            Pubkey::find_program_address(&[b"campaign", creator.pubkey().as_ref()], &program_id);
        let (vault_pda, _bump2) =
            Pubkey::find_program_address(&[b"vault", campaign_pda.as_ref()], &program_id);

        let goal_sol = 0.5 + (idx as f64) * 0.25; // e.g., 0.5, 0.75, 1.0 SOL
        let goal_lamports = sol(goal_sol);
        let deadline_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + (300 + idx as i64 * 60); // 5 min + increments

        println!(
            "Creating campaign {}: goal {} SOL, deadline {}",
            idx + 1,
            goal_sol,
            deadline_unix
        );

        program
            .request()
            .accounts(crowdfund_accounts::InitializeCampaign {
                creator: creator.pubkey(),
                campaign: campaign_pda,
                campaign_vault: vault_pda,
                system_program: system_program::ID,
            })
            .args(crowdfund_ix::InitializeCampaign {
                goal_lamports,
                deadline_unix,
            })
            .signer(creator)
            .send()
            .unwrap();

        // === 5) Seed contributions from 2 backers ===
        for b in 0..2 {
            let backer = Keypair::new();
            program
                .rpc()
                .request_airdrop(&backer.pubkey(), sol(2))
                .unwrap();

            let (backer_state_pda, _bump3) = Pubkey::find_program_address(
                &[b"backer", campaign_pda.as_ref(), backer.pubkey().as_ref()],
                &program_id,
            );

            let amount_sol = 0.1 + b as f64 * 0.05; // 0.1, 0.15 SOL
            let amount_lamports = sol(amount_sol);

            println!(
                " - Backer {} contributing {} SOL to campaign {}",
                b + 1,
                amount_sol,
                idx + 1
            );

            program
                .request()
                .accounts(crowdfund_accounts::Contribute {
                    backer: backer.pubkey(),
                    campaign: campaign_pda,
                    backer_state: backer_state_pda,
                    campaign_vault: vault_pda,
                    system_program: system_program::ID,
                })
                .args(crowdfund_ix::Contribute {
                    amount: amount_lamports,
                })
                .signer(&backer)
                .send()
                .unwrap();
        }
    }

    println!("✅ Seed data complete. Check campaigns and backers via your client/UI.");
}

