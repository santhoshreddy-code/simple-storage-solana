\# SimpleStorage Solana - Anchor Program



A Solana program built with Anchor framework that mirrors the SimpleStorage contract from Ethereum — demonstrating the same logic (store a number, owner-only access control) implemented in Rust on Solana.



\## Program



`simple\_storage\_solana` - A Solana program that demonstrates:

\- Account-based state storage (Solana's equivalent of Ethereum state variables)

\- Owner-restricted write access using Anchor's `require!` macro

\- Program Derived Addresses and account initialization

\- Cross-chain comparison: same logic as Ethereum SimpleStorage but using Solana's account model



\## Architecture: Ethereum vs Solana



This project is the Solana equivalent of the Ethereum SimpleStorage contract. Key differences:



| Concept | Ethereum (Solidity) | Solana (Rust/Anchor) |

|---------|-------------------|---------------------|

| State storage | Inside the contract | Separate account (StorageData) |

| Access control | `require(msg.sender == owner)` | `require!(storage.owner == user.key())` |

| Deployment | One step (contract + storage) | Two steps (deploy program, then initialize account) |

| Identity | `msg.sender` (implicit) | `Signer<'info>` (explicit account) |

| Framework | Hardhat | Anchor |

| Language | Solidity | Rust |



\## Program Structure

programs/simple-storage-solana/src/

└── lib.rs          ← Main program with 3 instructions:

• initialize - creates storage account, sets owner

• set\_number - owner-only state update

• get\_number - read stored value



\## Data Account



```rust

pub struct StorageData {

&#x20;   pub owner: Pubkey,       // 32 bytes - who deployed/initialized

&#x20;   pub stored\_number: u64,  // 8 bytes  - the stored value

}

```



\## Tests



4 automated tests covering initialization, state changes, and access control:

simple-storage-solana

✔ should initialize storage with owner and zero

✔ should allow owner to set a new number

✔ should reject non-owner from setting a number

✔ should return the correct stored number

4 passing



\## Tech Stack



\- \*\*Rust\*\* 1.95.0

\- \*\*Solana CLI\*\* 3.1.9

\- \*\*Anchor\*\* 1.0.0

\- \*\*TypeScript\*\* for tests

\- \*\*Devnet\*\* for deployment



\## Deployed Program



| Network | Program ID | Explorer |

|---------|-----------|----------|

| Devnet | `7uexy9CdAgZseuHWxKWmatP6NajvS5LiHvWzkHyrjy7C` | \[View](https://explorer.solana.com/address/7uexy9CdAgZseuHWxKWmatP6NajvS5LiHvWzkHyrjy7C?cluster=devnet) |



> Note: Devnet deployment pending SOL airdrop. Program is built and ready to deploy.



\## Quick Start



```bash

\# Install dependencies

npm install



\# Build the program

anchor build



\# Deploy to Devnet (requires SOL)

solana program deploy target/deploy/simple\_storage\_solana.so



\# Run tests (requires local validator or Devnet)

npx ts-mocha -p ./tsconfig.json -t 1000000 tests/\*\*/\*.ts

```



\## Related Projects



\- \[simple-storage-hardhat](https://github.com/santhoshreddy-code/simple-storage-hardhat) — Ethereum version (Solidity + Hardhat)

\- \[simple-storage-frontend](https://github.com/santhoshreddy-code/simple-storage-frontend) — React dApp frontend

\- \[santoosh-token](https://github.com/santhoshreddy-code/santoosh-token) — ERC-20 token + SecureVault

