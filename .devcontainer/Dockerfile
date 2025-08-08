FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

# System packages often needed by Solana CLI, Anchor builds, and Python deps
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
 && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev libudev-dev libsqlite3-dev \
    curl git ca-certificates jq unzip python3-venv python3-dev \
 && apt-get clean && rm -rf /var/lib/apt/lists/*
