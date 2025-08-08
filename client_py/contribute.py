# Contribute SOL to a campaign as the connected wallet.
# For simplicity we assume you are contributing to a campaign you created.
# To contribute to someone else’s campaign, set `creator_pubkey_base58` to their address.

import asyncio
from solders.pubkey import Pubkey
from anchorpy import Context
from util import load_program

async def main(amount_sol: float = 0.1, creator_pubkey_base58: str | None = None):
    program, provider = await load_program()
    backer = provider.wallet.pubkey()

    # If contributing to your own campaign, the creator is you.
    creator = Pubkey.from_string(creator_pubkey_base58) if creator_pubkey_base58 else backer

    campaign, _ = Pubkey.find_program_address(
        [b"campaign", bytes(creator)], program.program_id
    )
    vault, _ = Pubkey.find_program_address([b"vault", bytes(campaign)], program.program_id)
    backer_state, _ = Pubkey.find_program_address(
        [b"backer", bytes(campaign), bytes(backer)], program.program_id
    )

    amount = int(amount_sol * 1_000_000_000)
    tx = await program.rpc["contribute"](
        amount,
        ctx=Context(
            accounts={
                "backer": backer,
                "campaign": campaign,
                "backer_state": backer_state,
                "campaign_vault": vault,
                "system_program": Pubkey.from_string("11111111111111111111111111111111"),
            }
        ),
    )
    print(f"Contributed {amount} lamports — tx:", tx)

if __name__ == "__main__":
    asyncio.run(main())
