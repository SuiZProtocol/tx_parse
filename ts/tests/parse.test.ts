import { describe, expect, it, vi } from "vitest";
import type { SuiClient, SuiTransactionBlockResponse } from "@mysten/sui/client";
import { TxParseClient, parseTransaction } from "../src";
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
});
