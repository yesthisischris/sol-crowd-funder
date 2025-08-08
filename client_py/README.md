# Python client quickstart

# 1) Create & fund a devnet wallet (CLI)
solana config set --url https://api.devnet.solana.com
solana-keygen new -o ~/.config/solana/id.json --no-bip39-passphrase --force
solana airdrop 2
solana balance

# 2) Install Python deps
python -m venv .venv && source .venv/bin/activate
pip install -r client_py/requirements.txt

# 3) Configure env (after you deploy; see project README)
cp client_py/.env.example client_py/.env
# set PROGRAM_ID in client_py/.env

# 4) Run scripts
python client_py/init_campaign.py
python client_py/contribute.py
python client_py/withdraw.py
python client_py/refund.py
