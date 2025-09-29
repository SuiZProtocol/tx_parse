# AGENTS.md

This document is the canonical set of instructions for AI coding agents working in the `tx_parse` repository. Follow the directives below when planning, editing, or testing code.

## Repository Topology

- `ts/` – TypeScript SDK built with Bun (compatible with npm/pnpm/yarn) and tested via Vitest.
- `rust/` – Rust crate providing an async JSON-RPC client and parsing helpers.
- `fixtures/transaction_block.json` – Shared fixture consumed by both stacks for deterministic tests.

## Environment Setup

### Node.js / Bun Tooling
- Preferred runtime: [Bun](https://bun.sh/). Use `bun install` and `bun run …` when possible.
- Alternate package managers (npm, pnpm, yarn) are acceptable if Bun is unavailable.

### Rust Toolchain
- Requires Rust 1.75+ with `cargo`.
- Install `rustfmt` via `rustup component add rustfmt` before formatting.

## Build & Test Commands

| Area | Install | Build | Test | Clean |
| --- | --- | --- | --- | --- |
| TypeScript (`ts/`) | `bun install` | `bun run build` | `bun run test` | `bun run clean` |
| Rust (`rust/`) | `cargo fetch` (optional prefetch) | _n/a_ | `cargo test` | _n/a_ |

> Tests do not require live network access. Rust tests mock HTTP requests; TypeScript tests rely on local fixtures.

## Coding Standards

- **TypeScript**: Prefer ES modules and existing code style. Run `bun run lint` if introduced in future; otherwise ensure Vitest passes.
- **Rust**: Format with `cargo fmt --all`. Follow async/await patterns established in the crate.
- Share parsing logic semantics across languages—changes in one implementation should be mirrored in the other when applicable.

## Commit Guidelines

- Keep commits atomic and well described.
- Update documentation (README, CHANGELOG, etc.) when altering behaviour or workflows.

## Security & Data Handling

- Do not introduce network calls in tests beyond existing mocks.
- Avoid committing secrets, access tokens, or personal data.

## Documentation

- README must remain bilingual (English primary, Chinese secondary) with clear language switching instructions.
- When adding new workflows or commands, document them in both README and this AGENTS file so other agents stay in sync.

Follow these instructions unless overridden by more specific `AGENTS.md` files in subdirectories.
