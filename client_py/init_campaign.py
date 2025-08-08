# Initialize a new crowdfunding campaign from your local wallet.
# This demonstrates how to derive PDAs the same way the program does,
# then call an instruction with the correct accounts.

import asyncio, time
from solders.pubkey import Pubkey
from anchorpy import Context
from util import load_program

async def main():
    program, provider = await load_program()

    # The creator is your local wallet from ANCHOR_WALLET.
    creator = provider.wallet.pubkey()

    # Derive the Campaign PDA using the same seeds as the on-chain program.
    campaign, _ = Pubkey.find_program_address(
        [b"campaign", bytes(creator)], program.program_id
    )
    # Derive the Vault PDA which will hold SOL contributions.
    vault, _ = Pubkey.find_program_address(
        [b"vault", bytes(campaign)], program.program_id
    )

    # Pick a small goal and a near-term deadline for quick testing.
    goal_sol = 0.5
    goal_lamports = int(goal_sol * 1_000_000_000)
    deadline = int(time.time()) + 3600  # one hour from now

    # Call the program method with named accounts.
    tx_sig = await program.rpc["initialize_campaign"](
        goal_lamports,
        deadline,
        ctx=Context(
            accounts={
                "creator": creator,
                "campaign": campaign,
                "campaign_vault": vault,
                "system_program": Pubkey.from_string("11111111111111111111111111111111"),
            }
        ),
    )
    print("Initialized campaign:", campaign, "tx:", tx_sig)

if __name__ == "__main__":
    asyncio.run(main())
