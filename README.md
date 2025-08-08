# Solana Crowdfund (Anchor + Python)

A basic crowdfunding program on Solana.

Features:
- Derives PDAs (Program Derived Addresses) for state and vaults
- Instruction handlers written with Anchor
- Interact from **Python** (no TypeScript needed)

## Requirements
- Solana CLI
- Rust + cargo
- Anchor CLI (`npm i -g @coral-xyz/anchor-cli`)
- Python 3.11+ (venv recommended)
- Phantom wallet (set to **Devnet**)

## Devnet Setup & Airdrop
```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new -o ~/.config/solana/id.json --no-bip39-passphrase --force
solana airdrop 2
solana balance
```

## Build & Deploy (Anchor)
```bash
anchor keys list                                  # copy your Program ID
# 1) Add Program ID to programs/<name>/src/lib.rs (declare_id!)
# 2) Also add it in Anchor.toml under [programs.devnet]
anchor build
anchor deploy
```

## Python Client
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

## Codespaces
- Click **Code → Create codespace on main**.
- First boot runs `.devcontainer/scripts/devcontainer-setup.sh`:
  - Installs Solana CLI, Anchor CLI, Rust, Node, Python deps
  - Generates a devnet keypair at `.devcontainer/keys/id.json`
  - Funds it with a small devnet airdrop
- Use **Tasks**: `Terminal → Run Task…` → e.g. *Anchor: Build*, *Python: Init campaign*.

## Local Testing
```bash
solana-test-validator   # run in a separate terminal (optional)
pytest -q
```

## Notes
- All lamports are stored in a vault PDA owned by the program.
- Refunds are available if the deadline passes and the goal isn’t met.
- Cluster time is based on `Clock::get()?.unix_timestamp`.

License: MIT
