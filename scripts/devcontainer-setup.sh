#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing Solana CLI (stable)…"
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

echo "==> Solana version:"
solana --version || true

echo "==> Installing Anchor CLI…"
npm i -g @coral-xyz/anchor-cli
anchor --version || true

# Make sure our devnet config is set
echo "==> Configuring Solana devnet…"
solana config set --url https://api.devnet.solana.com

# Create a workspace keypair for Codespaces (not your local machine keys)
WALLET="/workspaces/$(basename "$PWD")/.devcontainer/keys/id.json"
mkdir -p "$(dirname "$WALLET")"
if [ ! -f "$WALLET" ]; then
  echo "==> Generating devnet keypair for this Codespace…"
  solana-keygen new -o "$WALLET" --no-bip39-passphrase --force
fi
solana config set --keypair "$WALLET"

# Airdrop some devnet SOL to make first txs smoother (ignore errors if faucet is busy)
echo "==> Requesting airdrop (2 SOL)…"
solana airdrop 2 || true
solana balance || true

# Python: venv + deps for the client
echo "==> Python venv + client deps…"
python3 -m venv .venv
source .venv/bin/activate
pip -q install --upgrade pip
if [ -f client_py/requirements.txt ]; then
  pip -q install -r client_py/requirements.txt
fi

# If Anchor workspace exists, try building so IDL is available to anchorpy
if [ -f Anchor.toml ]; then
  echo "==> anchor build (to produce target/idl)…"
  anchor build || true
fi

# Quality of life
git config --global --add safe.directory /workspaces/$(basename "$PWD")
echo "==> Setup complete."
