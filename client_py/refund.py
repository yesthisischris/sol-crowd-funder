# Claim a refund after the deadline if the goal was NOT met.

import asyncio
from solders.pubkey import Pubkey
from anchorpy import Context
from util import load_program

async def main(creator_pubkey_base58: str | None = None):
    program, provider = await load_program()
    backer = provider.wallet.pubkey()

    # If refunding your own campaign contribution, set creator = your address when you contributed.
    creator = Pubkey.from_string(creator_pubkey_base58) if creator_pubkey_base58 else backer

    campaign, _ = Pubkey.find_program_address([b"campaign", bytes(creator)], program.program_id)
    vault, _ = Pubkey.find_program_address([b"vault", bytes(campaign)], program.program_id)
    backer_state, _ = Pubkey.find_program_address(
        [b"backer", bytes(campaign), bytes(backer)], program.program_id
    )

    tx = await program.rpc["refund"](
        ctx=Context(accounts={
            "backer": backer,
            "campaign": campaign,
            "backer_state": backer_state,
            "campaign_vault": vault
        })
    )
    print("Refund tx:", tx)

if __name__ == "__main__":
    asyncio.run(main())
