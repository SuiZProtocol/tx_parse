import type { GasCostSummary } from "@mysten/sui/client";

export interface BalanceChange {
  coinType: string;
  amount: string;
  owner: string;
}

export interface ParseResult {
  balanceChanges: BalanceChange[];
  gasCost: GasCostSummary;
}

export interface DynamicFieldBalanceChange {
  coinType: string;
  previousValue: string;
  currentValue: string;
  valueDiff: string;
  decimals: number;
}
