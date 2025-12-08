# Utxix

**The AI editor that forges Bitcoin covenants.**

Real-time stack visualization • Instant sCrypt compilation • One-click testnet deployment • AI that generates full contracts and frontends from a single prompt.

## Original Repository

This project is forked from [Zed](https://github.com/zed-industries/zed), a high-performance, multiplayer code editor built in Rust using the GPUI framework.

## What We Built

Utxix extends Zed with Bitcoin-native development capabilities:

### Bitcoin App Wizard
A modal wizard that scaffolds production-ready Bitcoin dApps with:
- **Frontend frameworks**: React, Vue, Next.js, Angular, or Svelte
- **Smart contract templates**: TicTacToe, Auction, Counter, HelloWorld, or Custom
- **Wallet integration**: Yours Wallet (browser extension) - no OAuth backend required
- **Battle-tested patterns**: Custom tx builder, ANYONECANPAY_SINGLE sighash, proper fee handling

### Key Features
- Native Rust performance (no Electron)
- GPUI-powered Bitcoin stack debugger
- Inline bytecode + fee estimation
- AI contract & dApp generator with pre-loaded context
- Built-in testnet deployment panel
- sCrypt-first language support

## Architecture Overview

```
utxix/
├── crates/
│   ├── bitcoin_app_wizard/       # Bitcoin dApp scaffolding wizard
│   │   ├── src/
│   │   │   ├── bitcoin_app_wizard.rs  # Module entry, action registration
│   │   │   ├── wizard_modal.rs        # GPUI modal UI, project creation logic
│   │   │   └── templates.rs           # All generated code templates
│   │   │       ├── Contract templates (sCrypt)
│   │   │       ├── YoursDirectSigner (custom wallet signer)
│   │   │       ├── Contract service (deploy, settle, tx builder)
│   │   │       ├── Framework templates (React, Vue, etc.)
│   │   │       └── AI_RULES.md (development guidelines)
│   ├── agent_ui/                 # AI assistant panel integration
│   ├── zed/                      # Main application entry point
│   └── ...                       # Other Zed crates (gpui, workspace, etc.)
```

### Bitcoin App Wizard Flow
1. User triggers `NewBitcoinApp` action via Command Palette
2. `BitcoinAppWizard` modal presents step-by-step configuration
3. On completion, `write_scaffold()` generates project files:
   - Contract source (`.scrypt.ts`)
   - Frontend framework files
   - Wallet integration services
   - AI rules documentation
4. Opens new workspace with contract file + AI panel pre-populated

### Generated Project Structure
```
my-bitcoin-app/
├── contracts/
│   └── Contract.scrypt.ts        # sCrypt smart contract
├── src/
│   ├── components/               # UI components
│   ├── lib/
│   │   └── wallet.ts             # Wallet state management
│   └── services/
│       ├── contractService.ts    # Deploy, settle, restore
│       ├── pandaSignerService.ts # SDK signer wrapper
│       └── yoursWalletDirect.ts  # Custom signer (bypasses bugs)
├── AI_RULES.md                   # Development guidelines
└── ...
```

## Setup + Run Steps

### Prerequisites
- macOS, Linux, or Windows
- Rust toolchain (rustup)
- Node.js 18+ (for generated projects)
- [Yours Wallet](https://chromewebstore.google.com/detail/yours-wallet) browser extension

### Building Utxix

```bash
# Clone the repository
git clone https://github.com/your-org/utxix.git
cd utxix

# Build in release mode
cargo build --release

# Run the editor
cargo run --release
```

### Using the Bitcoin App Wizard

1. Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
2. Type "New Bitcoin App" and select it
3. Configure your project:
   - Enter app name
   - Choose frontend framework (React, Vue, Next.js, Angular, Svelte)
   - Select contract template (TicTacToe, Auction, Counter, Custom)
4. Select destination folder
5. AI panel opens with context-aware prompt

### Running a Generated Project

```bash
cd my-bitcoin-app

# Install dependencies
npm install

# Compile smart contract
npx scrypt-cli compile

# Start development server
npm run dev
```

## Technical Decisions

### Why Yours Wallet Instead of DotWallet OAuth?

**Problem**: DotWallet requires an OAuth backend server, adding deployment complexity and a point of failure.

**Solution**: Yours Wallet (formerly Panda Wallet) is a browser extension that provides direct signing via `window.yours` API. No backend required.

### Why a Custom Signer (YoursDirectSigner)?

**Problem**: The standard `PandaSigner` from scrypt-ts SDK has proxy object bugs that cause signing failures, especially with complex transactions.

**Solution**: `YoursDirectSigner` implements the `Signer` interface directly, routing `getSignatures()` to the wallet's native API while using SDK helpers for script building.

### Why Custom Transaction Builder (bindTxBuilder)?

**Problem**: The SDK's default transaction building conflicts with custom sighash types (`ANYONECANPAY_SINGLE`) and can produce incorrect outputs.

**Solution**: `bindTxBuilder()` gives full control over transaction construction:
- Contract UTXO at input 0
- Verified payout at output 0
- Manual fee UTXO funding
- Explicit change address

### Why ANYONECANPAY_SINGLE Sighash?

**Benefits**:
- `ANYONECANPAY`: Allows adding fee inputs without invalidating signature
- `SINGLE`: Only verifies output at index 0, simplifying settlement logic

This pattern enables:
- Winner-takes-all or split payouts
- External fee funding
- Simple output verification in contract

### Why Manual Fee Handling?

**Problem**: Provider auto-funding can:
- Override your fee rate
- Merge values into the payout output
- Use minimal fees causing slow confirmation

**Solution**: 
- Manually select fee UTXO via `getPaymentUtxos()`
- Set explicit rate with `feePerKb(100)` (~100 sat/KB)
- Always include change output

### Why toRaw() for Vue?

**Problem**: Vue's reactive proxy wraps contract instances, breaking property access (`this.player1` returns undefined).

**Solution**: Call `toRaw(contractInstance)` before passing to SDK methods to unwrap the proxy.

### Why Generate AI_RULES.md?

**Problem**: AI assistants make common mistakes (importing .scrypt.ts directly, forgetting to compile, misusing sighash).

**Solution**: Pre-generated `AI_RULES.md` with:
- Compilation workflow
- Transaction building patterns
- Common pitfalls and solutions
- Debugging tips

This file is automatically included in AI context, reducing errors.

## License

See [LICENSE-AGPL](LICENSE-AGPL) for the license (inherited from Zed).
