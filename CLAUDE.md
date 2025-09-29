# CLAUDE.md

This file documents the key workflows for AI coding agents working in this repository.

## Development Commands

### TypeScript SDK (`ts/`)
- `bun run build` – Build ES + UMD bundles and declaration files
- `bun run test` – Run Vitest suite against fixtures
- `bun run clean` – Remove `dist/`

### Rust Crate (`rust/`)
- `cargo test` – Run unit + integration tests (uses local HTTP mocks)
- `cargo fmt` – Format sources (requires `rustup component add rustfmt`)

## Project Architecture

The repo is now a two-language SDK for parsing Sui transactions:

- `ts/` contains the original TypeScript implementation built with Vite. Core exports: `TxParseClient`, `parseTransaction`, parsing types.
- `rust/` holds the async Rust client (`TxParseClient`) plus helper functions `parse_transaction` and `parse_transaction_value`.
- `fixtures/transaction_block.json` is shared by both stacks for deterministic tests.

### Core Parsing Logic

- Address, Object, Shared, ConsensusV2, and Immutable owners are normalised to stable strings in both languages.
- Gas usage is forwarded unchanged from `effects.gasUsed`.
- Error conditions (missing `effects.gasUsed`, RPC errors) surface as explicit exceptions/results.

### Testing

- TypeScript tests mock the Sui client and rely on the shared fixture.
- Rust tests include pure parsing checks plus HTTP-mocked integration coverage.
- Neither suite requires live network access.
