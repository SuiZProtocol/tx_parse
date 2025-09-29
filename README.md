# @suiz/tx-parse

<details open>
<summary><strong>English</strong> · <a href="#%E4%B8%AD%E6%96%87">中文</a></summary>

A polyglot SDK for parsing Sui blockchain transaction blocks. The repository now hosts both the original TypeScript package and a new first-class Rust crate that share fixtures and test coverage to guarantee parity.

## Features

- Extract balance changes with normalized owner metadata (Address, Object, Shared, ConsensusV2, Immutable)
- Return gas usage summaries straight from on-chain transaction effects
- **Track dynamic field balance changes within Sui Bags** with automatic decimal handling
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

#### Basic Transaction Parsing

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

#### Track Bag Dynamic Field Balance Changes

```typescript
const changes = await parser.getBagDynamicFieldBalanceChanges(
  'YOUR_TX_DIGEST',
  'YOUR_BAG_ID'
);

changes.forEach(change => {
  console.log(`Coin: ${change.coinType}`);
  console.log(`Previous: ${change.previousValue}`);
  console.log(`Current: ${change.currentValue}`);
  console.log(`Difference: ${change.valueDiff}`);
  console.log(`Decimals: ${change.decimals}`);
});
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

#### Basic Transaction Parsing

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

#### Track Bag Dynamic Field Balance Changes

```rust
use tx_parse::TxParseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TxParseClient::new("https://fullnode.mainnet.sui.io:443");
    let changes = client
        .get_bag_dynamic_field_balance_changes("YOUR_TX_DIGEST", "YOUR_BAG_ID")
        .await?;

    for change in changes {
        println!("Coin Type: {}", change.coin_type);
        println!("Previous: {}", change.previous_value);
        println!("Current: {}", change.current_value);
        println!("Difference: {}", change.value_diff);
        println!("Decimals: {}", change.decimals);
    }
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

</details>

---

## 中文

<p>切换语言：<strong>中文</strong> · <a href="#@suiz/tx-parse">English</a></p>

多语言 Sui 区块链交易块解析 SDK。本仓库同时维护最初的 TypeScript 包和全新的 Rust Crate，两者共享测试夹具并保持功能一致性。

## 核心特性

- 解析余额变更并标准化所有者信息（Address、Object、Shared、ConsensusV2、Immutable）
- 直接返回链上交易效果中的 Gas 消耗信息
- **追踪 Sui Bag 中动态字段的余额变化**，自动处理精度（decimals）
- TypeScript 与 Rust 实现共享一致的解析语义
- 基于夹具的测试，无需访问外部 RPC，同时保障行为稳定
- 适用于 Node/Bun 应用或异步 Rust 后端

## 仓库结构

| 路径 | 说明 |
| --- | --- |
| `ts/` | TypeScript SDK、构建工具链（Vite）以及 Vitest 测试 |
| `rust/` | Rust 异步 JSON-RPC 客户端及单元/集成测试 |
| `fixtures/transaction_block.json` | 两套测试共享的示例负载 |

## TypeScript SDK（`ts/`）

### 安装

```bash
cd ts
bun install  # 或使用 npm / pnpm / yarn
```

### 构建与测试

```bash
bun run build        # 构建 ES + UMD Bundle 与声明文件
bun run test         # 运行基于夹具的 Vitest 测试
```

### 使用示例

#### 基础交易解析

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

#### 追踪 Bag 动态字段余额变化

```typescript
const changes = await parser.getBagDynamicFieldBalanceChanges(
  'YOUR_TX_DIGEST',
  'YOUR_BAG_ID'
);

changes.forEach(change => {
  console.log(`币种: ${change.coinType}`);
  console.log(`之前: ${change.previousValue}`);
  console.log(`当前: ${change.currentValue}`);
  console.log(`差值: ${change.valueDiff}`);
  console.log(`精度: ${change.decimals}`);
});
```

离线解析时可以使用纯函数：

```typescript
import { parseTransaction } from '@suiz/tx-parse';
// const rawResponse = await client.getTransactionBlock(...)
const parsed = parseTransaction(rawResponse);
```

## Rust Crate（`rust/`）

### 项目依赖示例

```toml
# Cargo.toml
[dependencies]
tx_parse = { path = "../rust" }  # 在此 monorepo 中引用
# 或：tx_parse = "0.1"（发布到 crates.io 后）
```

### 代码示例

#### 基础交易解析

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

#### 追踪 Bag 动态字段余额变化

```rust
use tx_parse::TxParseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TxParseClient::new("https://fullnode.mainnet.sui.io:443");
    let changes = client
        .get_bag_dynamic_field_balance_changes("YOUR_TX_DIGEST", "YOUR_BAG_ID")
        .await?;

    for change in changes {
        println!("币种类型: {}", change.coin_type);
        println!("之前值: {}", change.previous_value);
        println!("当前值: {}", change.current_value);
        println!("差值: {}", change.value_diff);
        println!("精度: {}", change.decimals);
    }
    Ok(())
}
```

如需离线解析，可向 `tx_parse::parse_transaction_value` 传入 `serde_json::Value`，重用相同逻辑而无需访问 RPC。

### 测试命令

```bash
cd rust
cargo test
```

> 首次执行 `cargo test` 会下载依赖（如 `reqwest`、`httpmock` 等），请确保环境允许访问外网。

## 跨语言测试策略

两套 SDK 均加载共享的 `fixtures/transaction_block.json` 夹具，重点验证：

- 所有者枚举（Address、Object、Shared、ConsensusV2、Immutable）映射到一致的展示字符串
- Gas 使用数据保持原样
- 客户端在无需访问公共 Sui RPC 的情况下，清晰地暴露 RPC 传输与逻辑错误

## 开发提示

- `.gitignore` 同时忽略 Rust `target/` 与 Node 构建产物
- `cargo fmt` 依赖 `rustfmt` 组件（通过 `rustup component add rustfmt` 安装）
- 旧版 TypeScript 导出仍然可用，现有项目无需修改代码即可升级

## 许可证

MIT
