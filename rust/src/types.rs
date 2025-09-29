use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceChange {
    pub coin_type: String,
    pub amount: String,
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCostSummary {
    pub computation_cost: String,
    pub storage_cost: String,
    pub storage_rebate: String,
    pub non_refundable_storage_fee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseResult {
    pub balance_changes: Vec<BalanceChange>,
    pub gas_cost: GasCostSummary,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransactionBlockResponse {
    #[allow(dead_code)]
    pub digest: Option<String>,
    #[serde(default)]
    pub balance_changes: Vec<RawBalanceChange>,
    pub effects: Option<TransactionEffects>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawBalanceChange {
    pub coin_type: String,
    pub amount: String,
    pub owner: Option<ObjectOwner>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct TransactionEffects {
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<GasCostSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SharedOwner {
    pub initial_shared_version: String,
    #[serde(default, rename = "mutable")]
    pub _mutable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ConsensusV2Owner {
    pub start_version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum ObjectOwner {
    Immutable(String),
    AddressOwner {
        #[serde(rename = "AddressOwner")]
        address_owner: String,
    },
    ObjectOwner {
        #[serde(rename = "ObjectOwner")]
        object_owner: String,
    },
    Shared {
        #[serde(rename = "Shared")]
        shared: SharedOwner,
    },
    ConsensusV2 {
        #[serde(rename = "ConsensusV2")]
        consensus_v2: ConsensusV2Owner,
    },
    Other(Value),
}
