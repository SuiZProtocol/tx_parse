# @suiz/tx-parse

A polyglot SDK for parsing Sui blockchain transaction blocks. The repository now hosts both the original TypeScript package and a new first-class Rust crate that share fixtures and test coverage to guarantee parity.

## Features

- Extract balance changes with normalized owner metadata (Address, Object, Shared, ConsensusV2, Immutable)
- Return gas usage summaries straight from on-chain transaction effects
- Identical parsing semantics for TypeScript and Rust implementations
- Fixture-driven tests that avoid external RPC calls while protecting behaviour
- Ready for integration in Node/Bun apps or async Rust backends

## Repository Layout

| Path | Description |
| --- | --- |
| `ts/` | TypeScript SDK, build tooling (Vite) and Vitest suite |
| `rust/` | Rust crate with async JSON-RPC client and unit/integration tests |
| `fixtures/transaction_block.json` | Shared sample payload used across both test suites |

## TypeScript SDK (`ts/`)

### Install

```bash
cd ts
bun install  # or npm install / pnpm install / yarn install
```

### Build & Test

```bash
bun run build        # build ES + UMD bundles with d.ts output
bun run test         # run vitest unit tests (fixture-backed)
```

### Usage

```typescript
import { TxParseClient } from '@suiz/tx-parse';
import { SuiClient, getFullnodeUrl } from '@mysten/sui/client';

const suiClient = new SuiClient({
  url: getFullnodeUrl('mainnet'),
});

const parser = new TxParseClient(suiClient);
const result = await parser.parseTX('YOUR_TX_DIGEST');

console.log(result.balanceChanges);
console.log(result.gasCost);
```

You can also import the pure helper for offline processing:

```typescript
import { parseTransaction } from '@suiz/tx-parse';
// const rawResponse = await client.getTransactionBlock(...)
const parsed = parseTransaction(rawResponse);
```

## Rust Crate (`rust/`)

### Add to your project

```toml
# Cargo.toml
[dependencies]
tx_parse = { path = "../rust" }  # when working inside this monorepo
# or: tx_parse = "0.1" once published to crates.io
```

### Example

```rust
use tx_parse::TxParseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TxParseClient::new("https://fullnode.mainnet.sui.io:443");
    let parsed = client.parse_transaction("YOUR_TX_DIGEST").await?;

    println!("Owners: {:?}", parsed.balance_changes);
    println!("Gas: {:?}", parsed.gas_cost);
    Ok(())
}
```

Prefer offline parsing? Pass a `serde_json::Value` to `tx_parse::parse_transaction_value` and reuse the exact parsing logic without hitting an RPC endpoint.

### Tests

```bash
cd rust
cargo test
```

> The first `cargo test` run will fetch crates (`reqwest`, `httpmock`, etc.). Ensure the environment allows outbound network access.

## Cross-Language Testing Strategy

Both SDKs load the shared `fixtures/transaction_block.json` fixture. The tests assert:

- Owner variants (Address, Object, Shared, ConsensusV2, Immutable) map to consistent display strings
- Gas usage data is preserved verbatim
- Clients surface RPC transport and logical errors cleanly without hitting public Sui RPC endpoints

## Development Notes

- `.gitignore` now excludes Rust `target/` artefacts in addition to Node build output.
- `cargo fmt` requires the `rustfmt` component (`rustup component add rustfmt`).
- All previous TypeScript exports remain available; existing consumers can upgrade without code changes.

## License

MIT
