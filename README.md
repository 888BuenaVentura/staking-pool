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
cd staking-pool
yarn install   
