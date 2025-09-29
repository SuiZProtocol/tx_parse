use serde_json::Value;
use thiserror::Error;

use crate::types::{BalanceChange, ParseResult, TransactionBlockResponse};
use crate::utils::get_actual_owner;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("transaction response does not include gas usage information")]
    MissingGasUsage,
    #[error("transaction payload could not be deserialized: {0}")]
    InvalidPayload(#[from] serde_json::Error),
}

pub(crate) fn parse_transaction(response: &TransactionBlockResponse) -> Result<ParseResult, ParseError> {
    let gas_cost = response
        .effects
        .as_ref()
        .and_then(|effects| effects.gas_used.clone())
        .ok_or(ParseError::MissingGasUsage)?;

    let balance_changes = response
        .balance_changes
        .iter()
        .map(|change| BalanceChange {
            coin_type: change.coin_type.clone(),
            amount: change.amount.clone(),
            owner: get_actual_owner(change.owner.as_ref()).unwrap_or_default(),
        })
        .collect();

    Ok(ParseResult {
        balance_changes,
        gas_cost,
    })
}

pub fn parse_transaction_value(value: &Value) -> Result<ParseResult, ParseError> {
    let response: TransactionBlockResponse = serde_json::from_value(value.clone())?;
    parse_transaction(&response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TransactionBlockResponse;

    fn load_fixture() -> TransactionBlockResponse {
        let raw = include_str!("../../fixtures/transaction_block.json");
        serde_json::from_str(raw).expect("fixture should deserialize")
    }

    #[test]
    fn parses_balance_changes_and_gas_usage() {
        let response = load_fixture();
        let parsed = parse_transaction(&response).expect("parse should succeed");

        assert_eq!(parsed.balance_changes.len(), 5);
        assert_eq!(parsed.balance_changes[0].owner, "0x6f4d3a");
        assert_eq!(parsed.balance_changes[2].owner, "Shared-42");
        assert_eq!(parsed.balance_changes[3].owner, "ConsensusV2-77");
        assert_eq!(parsed.balance_changes[4].owner, "Immutable");
        assert_eq!(parsed.gas_cost.computation_cost, "100");
    }

    #[test]
    fn fails_when_gas_information_missing() {
        let mut response = load_fixture();
        response.effects = None;

        let err = parse_transaction(&response).expect_err("should fail without gas cost");
        assert!(matches!(err, ParseError::MissingGasUsage));
    }

    #[test]
    fn parses_from_json_value() {
        let raw = include_str!("../../fixtures/transaction_block.json");
        let value: Value = serde_json::from_str(raw).expect("fixture should parse");

        let parsed = parse_transaction_value(&value).expect("value parsing should succeed");
        assert_eq!(parsed.balance_changes.len(), 5);
    }
}
