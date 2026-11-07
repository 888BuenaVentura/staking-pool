# staking-pool

Solana Anchor staking pool: SOL staking with rewards.

## Overview

A staking pool protocol on Solana where users deposit SOL to earn rewards. Built with the Anchor framework.

## Features

- Deposit and withdraw SOL
- Reward accrual for staked SOL
- On-chain state management via Anchor

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust |
| Framework | Anchor |
| Chain | Solana |
| Testing | Anchor tests (Rust) + JS scripts |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (version specified in `rust-toolchain.toml`)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### Install

```bash
git clone https://github.com/888BuenaVentura/staking-pool.git

Build
anchor build

Deploy (localnet)
anchor test

Deploy (devnet)
anchor deploy --provider.cluster devnet

Project Structure
├── programs/
│   └── staking_pool/      # Anchor program (Rust)
├── scripts/               # Deploy & test scripts
├── Anchor.toml            # Anchor config
├── Cargo.toml             # Rust workspace
└── package.json           # JS dependencies   

License
MIT

### Instrucciones

1. Abre `README.md` en tu repo `staking-pool`
2. Reemplaza todo el contenido con lo de arriba
3. Haz `git add . && git commit -m "Update README" && git push`
4. Si puedes, sube un screenshot de la terminal mostrando `anchor test` pasando → guárdalo como `screenshot.png` en la raíz del repo y agrega esta línea al final del README:

```markdown
## Screenshot

| Test passing |
|:---:|
| ![Test](screenshot.png) |   
cd staking-pool
yarn install   
