# ⚡ StellarTip — Decentralized Tip Jar on Stellar

![Stellar](https://img.shields.io/badge/Stellar-Testnet-blue)
![Soroban](https://img.shields.io/badge/Soroban-Smart%20Contract-purple)
![Status](https://img.shields.io/badge/Status-Active-green)

A decentralized tipping application built with **Soroban smart contracts** on the **Stellar blockchain**. 
StellarTip allows anyone to send XLM tips to content creators with a personal message, 
all recorded transparently on-chain.

---

## 🎯 Features

- 💸 **Send XLM Tips** — Send any amount of XLM to a creator's wallet
- 💬 **Personal Messages** — Attach a message with each tip
- 📊 **On-chain History** — All tips are permanently stored on Stellar testnet
- 🏆 **Tip Statistics** — View total received, number of tips, and biggest tip
- 🔒 **Secure Auth** — Soroban's built-in `require_auth()` for sender verification
- ⚡ **Fast & Cheap** — Stellar's low-fee infrastructure

---

## 🏗️ Project Structure
soroban-tipjar-app/
├── contracts/
│   └── tipjar/
│       ├── src/
│       │   ├── lib.rs      # Smart contract logic
│       │   └── test.rs     # Unit tests
│       └── Cargo.toml
├── Cargo.toml              # Workspace config
├── Makefile                # Build & deploy commands
└── README.md

---

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Stellar CLI
cargo install --locked stellar-cli --features opt

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build

```bash
make build
```

### Test

```bash
make test
```

### Deploy to Testnet

```bash
# Configure testnet
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Create & fund account
stellar keys generate alice --network testnet
stellar keys fund alice --network testnet

# Deploy
make deploy-testnet
```

### Initialize Contract

```bash
make invoke-init \
  CONTRACT_ID=<your-contract-id> \
  OWNER_ADDRESS=<creator-wallet-address>
```

### Send a Tip

```bash
make invoke-tip \
  CONTRACT_ID=<your-contract-id> \
  SOURCE=tipper \
  FROM=<tipper-address> \
  TOKEN=<xlm-token-address> \
  AMOUNT=100000000 \
  MESSAGE="Great content!"
```

---

## 📋 Smart Contract ID (Testnet)

| Item | Value |
|------|-------|
| Network | Stellar Testnet |
| Contract ID | `CD75ROC6QPV53EL6KFRSOVA23NCA6XHHUGPGPGJGQQFLQB56LA4CUVBB` |
| Owner Address | `GAXVHBIMXAMM2XD7C6CKCXRPNIS53SNWUCQQAQL4XUV5O6XHVHQ6FEWQ` |
| Deployed | April 2026 |
|contracts | https://stellar.expert/explorer/testnet/tx/77cbd84ee2ec0d80dcd6c82fffafb41aaa5e76279435a89b6a6039e7d98f8a57 |
| | https://lab.stellar.org/r/testnet/contract/CD75ROC6QPV53EL6KFRSOVA23NCA6XHHUGPGPGJGQQFLQB56LA4CUVBB |

---

## 📸 Testnet Screenshot

![Testnet Screenshot](screenshot1.png)
![Testnet Screenshot](screenshot2.png)

> Screenshot shows the contract deployed and initialized on Stellar testnet explorer.

---

## 🔧 Contract Functions

| Function | Description |
|----------|-------------|
| `initialize(owner)` | Set the creator/owner address |
| `send_tip(from, token, amount, message)` | Send a tip with message |
| `get_tips()` | Retrieve all tips |
| `get_total()` | Get total XLM received |
| `get_owner()` | Get owner address |

---

## 🌐 Resources

- [Stellar Developer Docs](https://developers.stellar.org)
- [Soroban Documentation](https://soroban.stellar.org)
- [Stellar Expert Testnet Explorer](https://stellar.expert/explorer/testnet/tx/0860b1c40fe7a035068345a5294f2d47081578d6b25a83b195657692499e165b)
- [Stellar Testnet Friendbot](https://friendbot.stellar.org)

---

## 📝 License

MIT License — feel free to use and modify!

---

Built with ❤️ using Soroban · Stellar Testnet

