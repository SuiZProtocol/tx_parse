import type { SuiTransactionBlockResponse } from "@mysten/sui/client";
import { getActualOwner } from "./utils";
import type { BalanceChange, ParseResult } from "./type";

export function parseTransaction(response: SuiTransactionBlockResponse): ParseResult {
  const gasCost = response.effects?.gasUsed;

  if (!gasCost) {
    throw new Error("Transaction response does not include gas usage information");
  }

  const balanceChanges: BalanceChange[] = (response.balanceChanges ?? []).map((change) => ({
    coinType: change.coinType,
    amount: change.amount,
    owner: getActualOwner(change.owner) ?? "",
  }));

  return {
    balanceChanges,
    gasCost,
  };
}
