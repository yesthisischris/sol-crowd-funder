# This utility centralizes how we connect to Solana and your Anchor program.
# It reads environment variables, sets up an RPC client + wallet, and returns a Program handle.

import os
from dotenv import load_dotenv
from anchorpy import Provider, Program
from solana.rpc.async_api import AsyncClient
from solders.pubkey import Pubkey

load_dotenv()  # load variables from client_py/.env if present

def env(key: str, default: str | None = None) -> str | None:
    """Tiny helper to read environment variables with an optional default."""
    return os.getenv(key, default)

async def load_program() -> tuple[Program, Provider]:
    """
    Create a Provider (wallet + RPC) and a Program object bound to your deployed ID.
    AnchorPy looks for the IDL output in target/idl/<program>.json after you build.
    """
    url = env("ANCHOR_PROVIDER_URL", "https://api.devnet.solana.com")
    wallet_path = env("ANCHOR_WALLET", os.path.expanduser("~/.config/solana/id.json"))

    # Async RPC client for talking to the cluster (devnet by default).
    client = AsyncClient(url)

    # Provider bundles RPC + wallet signer; handy for sending transactions.
    provider = await Provider.from_wallet_path(client, wallet_path)

    # PROGRAM_ID must be set in client_py/.env after you deploy.
    program_id = Pubkey.from_string(env("PROGRAM_ID"))
    program: Program = await Program.at(program_id, provider)

    return program, provider
