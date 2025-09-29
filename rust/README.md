# tx_parse (Rust)

Rust crate for parsing Sui transaction blocks with the same semantics as the TypeScript SDK that lives in this repository.

## Features

- Extract balance changes with normalized owner strings (address, object, shared, consensus, immutable)
- Return gas usage as provided by `sui_getTransactionBlock`
- Async JSON-RPC client with error handling helpers
- Fixture-driven tests that avoid hitting public RPC endpoints

## Usage

```toml
[dependencies]
tx_parse = { git = "https://github.com/SuiZProtocol/tx_parse", package = "tx_parse" }
```

```rust
use tx_parse::TxParseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TxParseClient::new("https://fullnode.mainnet.sui.io:443");
    let parsed = client.parse_transaction("YOUR_TX_DIGEST").await?;
    println!("Gas: {:?}", parsed.gas_cost);
    Ok(())
}
```

## License

MIT
