# End-to-end "happy path" test using your Python client utilities.
# For speed in CI we keep it minimal; expand with more cases as you learn.

import asyncio, time, pytest
from client_py.util import load_program
from solders.pubkey import Pubkey
from anchorpy import Context

pytestmark = pytest.mark.asyncio

async def init_campaign(program, provider, goal=500):
    creator = provider.wallet.pubkey()
    campaign, _ = Pubkey.find_program_address([b"campaign", bytes(creator)], program.program_id)
    vault, _    = Pubkey.find_program_address([b"vault", bytes(campaign)], program.program_id)
    deadline = int(time.time()) + 60  # 1 minute window

    await program.rpc["initialize_campaign"](goal, deadline, ctx=Context(accounts={
        "creator": creator,
        "campaign": campaign,
        "campaign_vault": vault,
        "system_program": Pubkey.from_string("11111111111111111111111111111111"),
    }))
    return campaign, vault, creator

async def contribute(program, provider, campaign, vault, amount=500):
    backer = provider.wallet.pubkey()
    backer_state, _ = Pubkey.find_program_address([b"backer", bytes(campaign), bytes(backer)], program.program_id)
    await program.rpc["contribute"](amount, ctx=Context(accounts={
        "backer": backer,
        "campaign": campaign,
        "backer_state": backer_state,
        "campaign_vault": vault,
        "system_program": Pubkey.from_string("11111111111111111111111111111111"),
    }))

async def test_happy_path():
    program, provider = await load_program()
    campaign, vault, creator = await init_campaign(program, provider, goal=500)
    await contribute(program, provider, campaign, vault, amount=500)
    tx = await program.rpc["withdraw"](ctx=Context(accounts={
        "creator": creator,
        "campaign": campaign,
        "campaign_vault": vault
    }))
    assert isinstance(tx, str)
