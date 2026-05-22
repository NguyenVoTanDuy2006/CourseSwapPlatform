# CourseSwap - Anonymous Course Slot Exchange

## Problem
Students face significant difficulties and risks when manually exchanging course slots, including the potential for scams or losing slots to public bots during the hand-over process.

## Solution
A decentralized course slot exchange platform built on Smart Contracts to ensure **Atomic Swaps**: a trade only executes if both parties provide the exact course information required by the other, eliminating counterparty risk.

## Why Stellar
Stellar Soroban offers ultra-low transaction fees and a robust framework for **Atomic Swaps**, enabling secure, near-instant, and anonymous transfers of course ownership on a public ledger.

## Target User
University students who need to adjust their class schedules, swap sections, or exchange courses during peak registration periods.

## Live Demo
- **Network**: Stellar Testnet
- **Contract ID**: `CAT2HAZFNSCNCG4OGII7GX4SELLGIW2NMDCLPCNCNKCMZWOJR76J3SSZ`
- **Transaction**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/420e32cfa14bfd383364f54f1f9872d1f08ffe37aa638b160790be85baf6e3f3)

## How to Run
1. **Clone**: `git clone https://github.com/yourname/course-swap.git`
2. **Build**: 
   ```bash
   cd project/contracts/course-swap
   stellar contract build
   ```
3. **Test**: 
   ```bash
   cargo test -p course-swap
   ```
4. **Deploy**:
   ```bash
   stellar contract deploy \
     --wasm project/target/wasm32v1-none/release/course_swap.wasm \
     --source admin \
     --network testnet
   ```
5. **Frontend**: 
   Open `project/index.html` in your browser (Freighter Wallet extension required).

## Tech Stack
- **Smart Contract**: Rust / Soroban SDK v25
- **Frontend**: HTML5 / JavaScript / @stellar/stellar-sdk
- **Wallet**: Freighter
- **Network**: Stellar Testnet

## Team
- **Nguyen Vo Tan Duy** | [nguyenvotanduy2006@gmail.com](mailto:nguyenvotanduy2006@gmail.com) | HCM - University of Science - Year 2
