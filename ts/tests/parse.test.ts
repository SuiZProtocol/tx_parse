import { describe, expect, it, vi } from "vitest";
import type { SuiClient, SuiTransactionBlockResponse } from "@mysten/sui/client";
import { TxParseClient, parseTransaction } from "../src";
import type { DynamicFieldBalanceChange } from "../src/type";
import fixture from "../../fixtures/transaction_block.json";

describe("parseTransaction", () => {
  it("parses balance changes and gas cost", () => {
    const result = parseTransaction(fixture as unknown as SuiTransactionBlockResponse);

    expect(result.balanceChanges).toHaveLength(5);
    expect(result.balanceChanges[0]).toEqual({
      coinType: "0x2::sui::SUI",
      amount: "1000",
      owner: "0x6f4d3a",
    });
    expect(result.balanceChanges[1]).toEqual({
      coinType: "0x2::other::COIN",
      amount: "-750",
      owner: "0x123456",
    });
    expect(result.balanceChanges[2]).toEqual({
      coinType: "0x2::shared::TOKEN",
      amount: "250",
      owner: "Shared-42",
    });
    expect(result.balanceChanges[3]).toEqual({
      coinType: "0x2::consensus::TOKEN",
      amount: "0",
      owner: "ConsensusV2-77",
    });
    expect(result.balanceChanges[4]).toEqual({
      coinType: "0x2::immutable::ART",
      amount: "1",
      owner: "Immutable",
    });

    expect(result.gasCost).toEqual({
      computationCost: "100",
      storageCost: "200",
      storageRebate: "50",
      nonRefundableStorageFee: "10",
    });
  });
});

describe("TxParseClient", () => {
  it("delegates fetching to the provided Sui client", async () => {
    const getTransactionBlock = vi.fn().mockResolvedValue(fixture);
    const client = new TxParseClient({
      getTransactionBlock,
    } as unknown as SuiClient);
    const digest = "0xdead";

    const result = await client.parseTX(digest);

    expect(result.balanceChanges).toHaveLength(5);
    expect(getTransactionBlock).toHaveBeenCalledWith({
      digest,
      options: {
        showBalanceChanges: true,
        showEvents: true,
        showEffects: true,
      },
    });
  });

  it("tracks bag dynamic field balance changes", async () => {
    const mockTxBlock = {
      objectChanges: [
        {
          type: "mutated",
          objectId: "0xobj1",
          version: "100",
          previousVersion: "99",
          owner: { ObjectOwner: "0xbag123" },
        },
        {
          type: "created",
          objectId: "0xobj2",
          version: "50",
          previousVersion: undefined,
          owner: { ObjectOwner: "0xbag123" },
        },
        {
          type: "mutated",
          objectId: "0xobj3",
          version: "200",
          previousVersion: "199",
          owner: { AddressOwner: "0xuser" },
        },
      ],
    };

    const mockCurrentObj1 = {
      status: "VersionFound",
      details: {
        content: {
          dataType: "moveObject",
          type: "0x2::coin::Coin<0x2::sui::SUI>",
          fields: { balance: "1000" },
        },
      },
    };

    const mockPreviousObj1 = {
      status: "VersionFound",
      details: {
        content: {
          dataType: "moveObject",
          type: "0x2::coin::Coin<0x2::sui::SUI>",
          fields: { balance: "800" },
        },
      },
    };

    const mockCurrentObj2 = {
      status: "VersionFound",
      details: {
        content: {
          dataType: "moveObject",
          type: "0x2::balance::Balance<0x2::sui::SUI>",
          fields: { value: "500" },
        },
      },
    };

    const getTransactionBlock = vi.fn().mockResolvedValue(mockTxBlock);
    const tryGetPastObject = vi
      .fn()
      .mockImplementation(({ id, version }) => {
        if (id === "0xobj1") {
          return version === 100 ? mockCurrentObj1 : mockPreviousObj1;
        }
        if (id === "0xobj2") {
          return mockCurrentObj2;
        }
        return null;
      });
    const getCoinMetadata = vi.fn().mockResolvedValue({ decimals: 9 });

    const client = new TxParseClient({
      getTransactionBlock,
      tryGetPastObject,
      getCoinMetadata,
    } as unknown as SuiClient);

    const changes = await client.getBagDynamicFieldBalanceChanges("0xtx", "0xbag123");

    expect(changes).toHaveLength(2);
    expect(changes[0]).toEqual({
      coinType: "0x2::sui::SUI",
      previousValue: "800",
      currentValue: "1000",
      valueDiff: "200",
      decimals: 9,
    });
    expect(changes[1]).toEqual({
      coinType: "0x2::sui::SUI",
      previousValue: "0",
      currentValue: "500",
      valueDiff: "500",
      decimals: 9,
    });
  });
});
