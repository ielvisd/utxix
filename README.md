# Utxix

**The AI editor that forges Bitcoin covenants.**

Real-time stack visualization • Instant sCrypt compilation • One-click testnet deployment • AI that generates full contracts and frontends from a single prompt.

Forked from [Zed](https://github.com/zed-industries/zed) and built for the UTXO era.

https://github.com/user-attachments/assets/your-screenshot-or-gif-here (placeholder)

## Features
- Native Rust performance (no Electron)
- GPUI-powered Bitcoin stack debugger
- Inline bytecode + fee estimation
- AI contract & dApp generator
- Built-in testnet deployment panel
- sCrypt-first language support

## Bitcoin App Wizard

Create production-ready Bitcoin dApps with a single command. The wizard scaffolds:

- **Frontend**: React, Vue, Next.js, Angular, or Svelte
- **Smart Contract**: sCrypt templates (TicTacToe, Auction, Counter, or Custom)
- **Wallet Integration**: Yours Wallet (browser extension) - no backend required
- **Battle-tested Patterns**: Custom tx builder, ANYONECANPAY_SINGLE sighash, proper fee handling

### Quick Start

1. Open Command Palette (`Cmd+Shift+P`)
2. Run "New Bitcoin App"
3. Choose framework and template
4. AI automatically opens with context to help you build

### Key Patterns Included

The wizard generates production-ready code with lessons learned from real deployments:

- **YoursDirectSigner**: Custom signer that bypasses wallet proxy bugs
- **Custom Transaction Builder**: `bindTxBuilder()` for full control over inputs/outputs
- **Fee Handling**: Manual fee UTXOs with `feePerKb(100)` for reliable relay
- **Vue Integration**: `toRaw()` pattern to avoid reactive proxy issues
- **Commit-Reveal**: Hash commitments for games with hidden moves

### Prerequisites

- [Yours Wallet](https://chromewebstore.google.com/detail/yours-wallet) browser extension
- Node.js 18+