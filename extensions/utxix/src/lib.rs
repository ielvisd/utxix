use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

struct UtxixExtension;

impl zed::Extension for UtxixExtension {
    fn new() -> Self {
        UtxixExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "covenant" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "time-locked escrow".to_string(),
                    new_text: "time-locked escrow with 2-of-3 multisig".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "hash-locked payment".to_string(),
                    new_text: "hash-locked payment channel".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "tic-tac-toe game".to_string(),
                    new_text: "tic-tac-toe game with timeout escrow".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "NFT auction".to_string(),
                    new_text: "NFT auction with bid escrow".to_string(),
                    run_command: true,
                },
            ]),
            "explain" => Ok(vec![]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "covenant" => {
                if args.is_empty() {
                    return Err("Please describe the covenant you want to generate (e.g., 'time-locked escrow')".to_string());
                }

                let description = args.join(" ");
                let text = generate_covenant_output(&description);

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("sCrypt Covenant: {}", description),
                    }],
                    text,
                })
            }
            "explain" => {
                if args.is_empty() {
                    return Err("Please provide sCrypt code to explain".to_string());
                }

                let code = args.join(" ");
                let text = generate_explain_output(&code);

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "sCrypt Explanation".to_string(),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

fn generate_covenant_output(description: &str) -> String {
    format!(
        r#"## Generated sCrypt Covenant Template

**Request:** {description}

```typescript
import {{ prop, method, SmartContract, PubKey, Sig, ByteString, hash256, assert }} from 'scrypt-ts';

/**
 * {description}
 * 
 * This is a template covenant. Modify the logic below to match your requirements.
 */
export class CustomCovenant extends SmartContract {{
    // Immutable properties (set at deployment, cannot change)
    @prop()
    readonly owner: PubKey;
    
    @prop()
    readonly lockUntilHeight: bigint;

    // Mutable state (can change between transactions)
    @prop(true)
    stateData: ByteString;

    constructor(owner: PubKey, lockUntilHeight: bigint) {{
        super(...arguments);
        this.owner = owner;
        this.lockUntilHeight = lockUntilHeight;
        this.stateData = ByteString('');
    }}

    /**
     * Main unlocking method - customize this logic
     */
    @method()
    public unlock(sig: Sig) {{
        // Verify signature from owner
        assert(this.checkSig(sig, this.owner), 'Invalid signature');
        
        // Check timelock (block height must be >= lockUntilHeight)
        assert(this.ctx.locktime >= this.lockUntilHeight, 'Timelock not expired');
    }}

    /**
     * Alternative unlock path - add your custom conditions
     */
    @method()
    public alternateUnlock(preimage: ByteString) {{
        // Hash-lock example: reveal preimage to unlock
        assert(hash256(preimage) == this.stateData, 'Invalid preimage');
    }}
}}
```

---

## sCrypt Covenant Guide (AI Context)

Use this reference when refining the contract above:

### Property Decorators
- `@prop()` - Immutable state, set at deployment
- `@prop(true)` - Mutable state, can change between transactions

### Method Rules
- All logic MUST be in `@method()` functions
- Use `assert(condition, 'message')` for validation (compiles to Bitcoin Script)
- Methods ending with `public` are unlocking conditions

### Bitcoin Script Context
- `this.ctx.locktime` - Current block height (for timelocks)
- `this.ctx.sequence` - Input sequence number
- `this.checkSig(sig, pubkey)` - Verify ECDSA signature
- `this.checkMultiSig(sigs, pubkeys)` - M-of-N multisig

### Crypto Functions
- `hash256(data)` - Double SHA256 (Bitcoin's standard)
- `hash160(data)` - RIPEMD160(SHA256(x)) for addresses
- `sha256(data)` - Single SHA256

### Critical Constraints
- NO unbounded loops (max ~10k ops per transaction)
- NO floating point - use `bigint` only
- State size affects transaction fees
- All paths must explicitly `assert()` their conditions

---

Refine this template for: **{description}**

Ask me to add: win detection, ZK proofs, oracle integration, or perceptron AI opponent.
"#,
        description = description
    )
}

fn generate_explain_output(code: &str) -> String {
    format!(
        r#"## sCrypt Code Explanation Request

**Code to explain:**
```typescript
{code}
```

---

## Explanation Context

When explaining sCrypt code, consider:

### How sCrypt Compiles to Bitcoin Script
1. `@prop()` values become part of the locking script
2. `@method()` functions define spending conditions
3. `assert()` statements compile to `OP_VERIFY` or conditional opcodes
4. All arithmetic uses `bigint` → Script's numeric stack

### Common Patterns to Identify
- **Signature checks** → `OP_CHECKSIG` / `OP_CHECKMULTISIG`
- **Hash locks** → `OP_HASH256` + `OP_EQUAL`
- **Timelocks** → `OP_CHECKLOCKTIMEVERIFY` / `OP_CHECKSEQUENCEVERIFY`
- **State transitions** → Output contains updated contract state

### What to Explain
1. What spending conditions does this code enforce?
2. What Bitcoin Script opcodes will this compile to?
3. What are the security assumptions?
4. What edge cases should be tested?

---

Please explain the code above in plain English, then show the approximate Bitcoin Script opcodes it compiles to.
"#,
        code = code
    )
}

zed::register_extension!(UtxixExtension);
