# @suiz/tx-parse

A TypeScript library for parsing Sui blockchain transactions with a focus on balance changes and gas costs.

## Features

- Parse Sui transaction blocks to extract balance changes
- Calculate gas costs from transaction effects
- Handle different types of Sui object ownership (AddressOwner, ObjectOwner, Shared, etc.)
- TypeScript support with full type definitions
- Available in both ES modules and UMD formats

## Installation

```bash
npm install @suiz/tx-parse
# or
yarn add @suiz/tx-parse
# or
bun add @suiz/tx-parse
```

## Usage

```typescript
import { TxParseClient } from '@suiz/tx-parse';
import { SuiClient, getFullnodeUrl } from '@mysten/sui/client';

// Initialize Sui client
const suiClient = new SuiClient({ 
  url: getFullnodeUrl('mainnet') 
});

// Create parser instance
const txParser = new TxParseClient(suiClient);

// Parse a transaction
const result = await txParser.parseTX('YOUR_TRANSACTION_DIGEST');

console.log('Balance Changes:', result.balanceChanges);
console.log('Gas Cost:', result.gasCost);
```

## API

### TxParseClient

The main client class for parsing transactions.

```typescript
class TxParseClient {
  constructor(suiClient: SuiClient)
  parseTX(tx: string): Promise<ParseResult>
}
```

### ParseResult

The result of parsing a transaction.

```typescript
interface ParseResult {
  balanceChanges: BalanceChange[];
  gasCost: GasCostSummary;
}
```

### BalanceChange

Represents a balance change in the transaction.

```typescript
interface BalanceChange {
  coinType: string;
  amount: string;
  owner: string;
}
```

## Development

### Prerequisites

- Node.js >= 14
- Bun (recommended) or npm/yarn

### Setup

```bash
# Install dependencies
bun install
```

### Commands

```bash
# Build the library
bun run build

# Run tests
bun run test

# Run tests with coverage
bun run test:coverage

# Clean build artifacts
bun run clean

# Publish versions
bun run publish:patch  # Patch version (0.0.x)
bun run publish:minor  # Minor version (0.x.0)
bun run publish:major  # Major version (x.0.0)
```

### Testing

Tests use Vitest and can be configured with environment variables:

```bash
# Use custom RPC URL (defaults to mainnet)
SUI_RPC_URL=https://your-rpc-url bun run test
```

## License

MIT