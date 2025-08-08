# Solana Crowdfund (Anchor + Python)

A tiny, educational crowdfunding program on Solana. You’ll learn:
- How to derive PDAs (Program Derived Addresses) for state + vaults
- How to write instruction handlers in Anchor
- How to call them from **Python** (no TypeScript required)

## Prereqs (one-time)
- Solana CLI
- Rust + cargo
- Anchor CLI (`npm i -g @coral-xyz/anchor-cli`)
- Python 3.11+ (venv recommended)
- Phantom wallet (switch to **Devnet** in Phantom settings)

## Configure devnet + airdrop (CLI)
```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new -o ~/.config/solana/id.json --no-bip39-passphrase --force
solana airdrop 2
solana balance
```

Build & deploy (Anchor)
```bash
anchor keys list                                  # copy the Program ID
# 1) Put Program ID in programs/<name>/src/lib.rs (declare_id!)
# 2) Also put it in Anchor.toml under [programs.devnet]
anchor build
anchor deploy
```

Python client
```bash
python -m venv .venv && source .venv/bin/activate
pip install -r client_py/requirements.txt
cp client_py/.env.example client_py/.env
# Set PROGRAM_ID in client_py/.env to your deployed ID
python client_py/init_campaign.py
python client_py/contribute.py
python client_py/withdraw.py
python client_py/refund.py
```

Local testing
```bash
solana-test-validator   # in a separate terminal (optional)
pytest -q
```

Notes
All lamports live in a vault PDA owned by the program.

Refunds only possible if deadline passed and goal not met.

Cluster time comes from Clock::get()?.unix_timestamp.

License: MIT
