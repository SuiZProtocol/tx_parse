import type { SuiClient } from "@mysten/sui/client";
import type { ParseResult, DynamicFieldBalanceChange } from "./type";
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

  async getBagDynamicFieldBalanceChanges(
    txDigest: string,
    bagId: string
  ): Promise<DynamicFieldBalanceChange[]> {
    // Step 1: Get transaction block with objectChanges
    const txBlock = await this.suiClient.getTransactionBlock({
      digest: txDigest,
      options: {
        showObjectChanges: true,
      },
    });

    if (!txBlock.objectChanges) {
      return [];
    }

    // Step 2: Filter objects owned by the bag
    const bagOwnedObjects = txBlock.objectChanges.filter((change) => {
      if (change.type === "created" || change.type === "mutated") {
        return change.owner && typeof change.owner === "object" && "ObjectOwner" in change.owner && change.owner.ObjectOwner === bagId;
      }
      return false;
    });

    // Step 3: For each object, get previous and current versions
    const balanceChanges: DynamicFieldBalanceChange[] = [];
    const coinMetadataCache = new Map<string, number>();

    for (const objChange of bagOwnedObjects) {
      if (objChange.type !== "created" && objChange.type !== "mutated") continue;

      const objectId = objChange.objectId;
      const version = objChange.version;
      const previousVersion = objChange.previousVersion;

      try {
        // Get current version content
        const currentObj = await this.suiClient.tryGetPastObject({
          id: objectId,
          version: parseInt(version),
          options: {
            showContent: true,
          },
        });

        // Get previous version content if exists
        let previousObj = null;
        if (previousVersion) {
          previousObj = await this.suiClient.tryGetPastObject({
            id: objectId,
            version: parseInt(previousVersion),
            options: {
              showContent: true,
            },
          });
        }

        // Extract balance values
        const currentValue = this.extractBalanceValue(currentObj);
        const previousValue = previousObj ? this.extractBalanceValue(previousObj) : "0";

        if (currentValue !== null && previousValue !== null) {
          try {
            const currentBigInt = BigInt(currentValue);
            const previousBigInt = BigInt(previousValue);
            const diff = currentBigInt - previousBigInt;

            const objectType = this.extractObjectType(currentObj) || "";
            const coinType = this.extractCoinType(objectType);

            // Fetch decimals for this coin type
            let decimals = 0;
            if (coinMetadataCache.has(coinType)) {
              decimals = coinMetadataCache.get(coinType)!;
            } else {
              decimals = await this.getCoinDecimals(coinType);
              coinMetadataCache.set(coinType, decimals);
            }

            balanceChanges.push({
              coinType,
              previousValue: previousValue,
              currentValue: currentValue,
              valueDiff: diff.toString(),
              decimals,
            });
          } catch (e) {
            console.error(`Failed to parse BigInt for object ${objectId}:`, { currentValue, previousValue, error: e });
          }
        }
      } catch (error) {
        console.error(`Error processing object ${objectId}:`, error);
      }
    }

    return balanceChanges;
  }

  private extractBalanceValue(obj: any): string | null {
    try {
      if (obj?.status === "VersionFound" && obj.details?.content?.dataType === "moveObject") {
        const fields = obj.details.content.fields;

        // Direct balance field
        if (fields?.balance !== undefined && fields?.balance !== null) {
          if (typeof fields.balance === 'object') {
            // Balance<T> type with nested value
            if (fields.balance.value !== undefined) {
              return String(fields.balance.value);
            }
          } else {
            return String(fields.balance);
          }
        }

        // Direct value field (could be nested object like Coin<T>)
        if (fields?.value !== undefined && fields?.value !== null) {
          if (typeof fields.value === 'object') {
            // Value is an object (like Coin<T>), try to extract balance from it
            if (fields.value.fields?.balance !== undefined) {
              return String(fields.value.fields.balance);
            }
            // Or it might have its own value field
            if (fields.value.fields?.value !== undefined) {
              return String(fields.value.fields.value);
            }
          } else {
            return String(fields.value);
          }
        }
      }
    } catch (error) {
      console.error("Error extracting balance value:", error);
    }
    return null;
  }

  private extractObjectType(obj: any): string | null {
    try {
      if (obj?.status === "VersionFound" && obj.details?.content?.dataType === "moveObject") {
        return obj.details.content.type;
      }
    } catch (error) {
      console.error("Error extracting object type:", error);
    }
    return null;
  }

  private extractCoinType(type: string): string {
    // Extract coin type from patterns like:
    // "0x2::coin::Coin<0x2::sui::SUI>"
    // "0x2::dynamic_field::Field<0x1::type_name::TypeName, 0x2::coin::Coin<0x2::sui::SUI>>"
    try {
      // Find the last occurrence of Coin< or Balance<
      const coinMatch = type.match(/(?:coin::Coin|balance::Balance)<([^>]+)>/);
      if (coinMatch && coinMatch[1]) {
        return coinMatch[1];
      }
    } catch (error) {
      console.error("Error extracting coin type:", error);
    }
    return type; // Return original type if parsing fails
  }

  private async getCoinDecimals(coinType: string): Promise<number> {
    try {
      const metadata = await this.suiClient.getCoinMetadata({ coinType });
      return metadata?.decimals ?? 0;
    } catch (error) {
      console.error(`Failed to get decimals for ${coinType}:`, error);
      return 0;
    }
  }
}
