import type { SuiClient } from "@mysten/sui/client";
import type { ParseResult } from "./type";
import { parseTransaction } from "./parse";

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
    return parseTransaction(res);
  }
}
