import { SuiClient } from "@mysten/sui/client";
import { TxParseClient } from "./src/client";

async function testMainnet() {
  const client = new SuiClient({
    url: "https://fullnode.mainnet.sui.io:443",
  });

  const txParseClient = new TxParseClient(client);

  const txDigest = "J5BzQREx52w3t75bFSZAy3uRpGne543vx251ZDf6LKmR";
  const bagId = "0x64ac48a57c8dfb3f69d5b0956be0c6727267978a11a53659c71f77c13c58aaad";

  console.log("Testing bag dynamic field balance changes on Sui mainnet...");
  console.log(`Transaction: ${txDigest}`);
  console.log(`Bag ID: ${bagId}`);
  console.log();

  try {
    const changes = await txParseClient.getBagDynamicFieldBalanceChanges(
      txDigest,
      bagId
    );

    console.log(`Found ${changes.length} dynamic field balance changes:\n`);

    if (changes.length === 0) {
      console.log("No balance changes found for this bag in this transaction.");
    } else {
      changes.forEach((change, index) => {
        const prevFormatted = (Number(change.previousValue) / Math.pow(10, change.decimals)).toFixed(change.decimals);
        const currFormatted = (Number(change.currentValue) / Math.pow(10, change.decimals)).toFixed(change.decimals);
        const diffFormatted = (Number(change.valueDiff) / Math.pow(10, change.decimals)).toFixed(change.decimals);

        console.log(`Change #${index + 1}:`);
        console.log(`  Coin Type: ${change.coinType}`);
        console.log(`  Decimals: ${change.decimals}`);
        console.log(`  Previous Value: ${change.previousValue} (${prevFormatted})`);
        console.log(`  Current Value: ${change.currentValue} (${currFormatted})`);
        console.log(`  Difference: ${change.valueDiff} (${diffFormatted})`);
        console.log();
      });
    }
  } catch (error) {
    console.error("Error testing mainnet:", error);
    throw error;
  }
}

testMainnet();