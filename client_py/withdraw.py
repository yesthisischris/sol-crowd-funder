# Withdraw raised funds if (and only if) the goal was met.
# Only the original campaign creator can call this successfully.

import asyncio
from solders.pubkey import Pubkey
from anchorpy import Context
from util import load_program

async def main():
    program, provider = await load_program()
    creator = provider.wallet.pubkey()

    campaign, _ = Pubkey.find_program_address([b"campaign", bytes(creator)], program.program_id)
    vault, _ = Pubkey.find_program_address([b"vault", bytes(campaign)], program.program_id)

    tx = await program.rpc["withdraw"](
        ctx=Context(accounts={
            "creator": creator,
            "campaign": campaign,
            "campaign_vault": vault
        })
    )
    print("Withdraw tx:", tx)

if __name__ == "__main__":
    asyncio.run(main())
