# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- `bun run build` - Build the library for distribution (creates UMD and ES modules)
- `bun run clean` - Remove dist directory
- `bun run test` - Run tests using Vitest
- `bun run test:coverage` - Run tests with coverage reporting
- `bun run publish:patch|minor|major` - Version bump and publish to npm

## Project Architecture

This is a TypeScript library for parsing Sui blockchain transactions. The project uses Vite for building and Vitest for testing.

### Core Components

- `TxParseClient` (src/client.ts) - Main client class that wraps SuiClient and provides transaction parsing functionality
- `ParseResult` interface - Returns balance changes and gas costs from parsed transactions
- `BalanceChange` interface (src/type.ts) - Represents coin balance changes with coinType, amount, and owner
- `getActualOwner` utility (src/utils.ts) - Handles different types of Sui object ownership (AddressOwner, ObjectOwner, Shared, ConsensusV2, Immutable)

### Key Dependencies

- `@mysten/sui` - Core Sui SDK for blockchain interactions
- External dependency that must be provided by consuming applications

### Build Configuration

The library builds to both ES modules and UMD format using Vite, with TypeScript declarations generated via vite-plugin-dts. The build externalizes @mysten/sui to avoid bundling it.

### Testing

Tests are located in `tests/` directory and use Vitest. The test suite includes transaction parsing tests that require a SUI_RPC_URL environment variable or defaults to mainnet.
