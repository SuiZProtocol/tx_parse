import { describe, it } from "vitest";
import { TxParseClient } from "../src/client";
import { getFullnodeUrl, SuiClient } from "@mysten/sui/client";

describe("Parse tx tests", () => {
  const suiClient = new SuiClient({ url: process.env.SUI_RPC_URL || getFullnodeUrl("mainnet") });

  const txParseClient = new TxParseClient(suiClient);

  it("Parse swap tx", async () => {
    const result = await txParseClient.parseTX("HskpZ6PEN3M5Jgvpa9Ykr4L1ExqePYf5DKhbRBNDsoms");
    console.log(result);
  })
})
