import { SuiClient, type GasCostSummary } from "@mysten/sui/client";
import type { BalanceChange } from "./type";
import { getActualOwner } from "./utils"

export interface ParseResult {
  balanceChanges: BalanceChange[];
  gasCost: GasCostSummary;
}

export class TxParseClient {
  constructor(private readonly suiClient: SuiClient) {}

  async parseTX(tx: string): Promise<ParseResult> {
    const res = await this.suiClient.getTransactionBlock({
      digest: tx,
      options: {
        showBalanceChanges: true,
        showEvents: true,
        showEffects: true,
      },
    });

    console.log(JSON.stringify(res, null, 2));

    const rowBalanceChanges = res.balanceChanges;
    const balanceChanges: BalanceChange[] = [];

    if (rowBalanceChanges) {
      for (const rowBalanceChange of rowBalanceChanges) {
        const balanceChange: BalanceChange = {
          coinType: rowBalanceChange.coinType,
          amount: rowBalanceChange.amount,
          owner: getActualOwner(rowBalanceChange.owner) || "",
        };
        balanceChanges.push(balanceChange);
      }
    }

    if (!res.effects) {
      console.error("No effects found");
      throw new Error("No effects found");
    }

    return {
      balanceChanges,
      gasCost: res.effects.gasUsed,
    };
  }
}
