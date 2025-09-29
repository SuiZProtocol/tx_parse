# Project Overview

## Purpose
`@suiz/tx-parse` is a TypeScript library that wraps the official Sui SDK to fetch and summarize balance changes and gas costs for a transaction block. The public API is intentionally small so the package can be embedded in wallets, analytics dashboards, or test harnesses that need a lightweight transaction parser.

## Directory Structure
- `src/` – Runtime source code for the library. `src/index.ts` re-exports the public API surface, while `src/client.ts` contains the `TxParseClient` implementation, `src/type.ts` defines shared interfaces, and `src/utils.ts` hosts helper logic for decoding Sui ownership metadata.
- `tests/` – Vitest suites that exercise parsing against live Sui RPC nodes.
- Tooling files such as `vite.config.ts`, `tsconfig.json`, and `package.json` configure bundling, type-checking, and distribution targets.

## Key Modules
### `TxParseClient`
The `TxParseClient` class composes a `SuiClient` instance and exposes `parseTX`, which retrieves a transaction block with balance changes, events, and effects enabled. It normalizes the response into a `ParseResult` by transforming each balance change and ensuring gas usage is available. The parser relies on `getActualOwner` to convert Sui SDK ownership variants into string identifiers.

### `BalanceChange`
A minimal interface that records the coin type, signed amount, and resolved owner string for each change detected during parsing.

### `getActualOwner`
Utility that inspects the discriminated union returned by the Sui SDK and returns a human-readable owner label for address, object, shared, consensus, or immutable ownership models.

## Tooling & Build System
- **Vite library mode** emits both ES module and UMD bundles with externalized `@mysten/sui` dependency and generates `.d.ts` declarations through `vite-plugin-dts`.
- **TypeScript** compiler options target modern runtimes (`ESNext`) with strict type checking suited for bundler environments.
- **Vitest** configuration mirrors the library entry points and enables coverage reporters.

## Tests & Local Development
Vitest tests construct a real `SuiClient`, defaulting to the public mainnet RPC. Set `SUI_RPC_URL` to point at a custom node when running locally to avoid rate limits. Common npm scripts are provided for building, testing (with or without coverage), cleaning artifacts, and publishing semantic versions.

## Suggested Next Steps
- Expand the test suite with mocked fixtures or recorded responses to avoid reliance on live network calls.
- Implement parsing helpers for events, coin metadata, or programmable transaction commands to broaden coverage.
- Provide higher-level examples (CLI or docs) that showcase interpreting balance changes for common DeFi actions.
