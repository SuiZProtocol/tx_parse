use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use thiserror::Error;

use crate::parse::{parse_transaction, ParseError};
use crate::types::{
    CoinMetadata, DynamicFieldBalanceChange, ObjectChange, ObjectChangesResponse, ObjectOwner,
    ParseResult, PastObjectResponse, TransactionBlockResponse,
};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("http transport error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("rpc error {code}: {message}")]
    Rpc {
        code: i64,
        message: String,
        data: Option<Value>,
    },
    #[error("rpc response missing result field")]
    MissingResult,
    #[error(transparent)]
    Parse(#[from] ParseError),
}

#[derive(Debug, Clone)]
pub struct TxParseClient {
    rpc_url: String,
    http: Client,
}

impl TxParseClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            http: Client::new(),
        }
    }

    pub fn with_http_client(rpc_url: impl Into<String>, http: Client) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            http,
        }
    }

    pub async fn parse_transaction(&self, digest: &str) -> Result<ParseResult, ClientError> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getTransactionBlock",
            "params": [
                digest,
                {
                    "showBalanceChanges": true,
                    "showEvents": true,
                    "showEffects": true,
                },
            ],
        });

        let response = self
            .http
            .post(&self.rpc_url)
            .json(&payload)
            .send()
            .await?;

        let rpc_response: RpcResponse<TransactionBlockResponse> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(ClientError::Rpc {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }

        let result = rpc_response.result.ok_or(ClientError::MissingResult)?;
        let parsed = parse_transaction(&result)?;

        Ok(parsed)
    }

    pub async fn get_bag_dynamic_field_balance_changes(
        &self,
        tx_digest: &str,
        bag_id: &str,
    ) -> Result<Vec<DynamicFieldBalanceChange>, ClientError> {
        // Step 1: Get transaction block with objectChanges
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_getTransactionBlock",
            "params": [
                tx_digest,
                {
                    "showObjectChanges": true,
                },
            ],
        });

        let response = self.http.post(&self.rpc_url).json(&payload).send().await?;
        let rpc_response: RpcResponse<ObjectChangesResponse> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(ClientError::Rpc {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }

        let result = rpc_response.result.ok_or(ClientError::MissingResult)?;
        let object_changes = result.object_changes.unwrap_or_default();

        // Step 2: Filter objects owned by the bag
        let bag_owned_objects: Vec<_> = object_changes
            .iter()
            .filter_map(|change| match change {
                ObjectChange::Created { object_id, version, owner }
                | ObjectChange::Mutated { object_id, version, owner, .. } => {
                    if let Some(ObjectOwner::ObjectOwner { object_owner }) = owner {
                        if object_owner == bag_id {
                            return Some((object_id.clone(), version.clone(),
                                match change {
                                    ObjectChange::Mutated { previous_version, .. } => Some(previous_version.clone()),
                                    _ => None,
                                }
                            ));
                        }
                    }
                    None
                }
                _ => None,
            })
            .collect();

        // Step 3: Process each object
        let mut balance_changes = Vec::new();
        let mut coin_metadata_cache: HashMap<String, u8> = HashMap::new();

        for (object_id, version, previous_version) in bag_owned_objects {
            match self.process_object(&object_id, &version, previous_version.as_deref(), &mut coin_metadata_cache).await {
                Ok(Some(change)) => balance_changes.push(change),
                Ok(None) => {},
                Err(e) => eprintln!("Error processing object {}: {:?}", object_id, e),
            }
        }

        Ok(balance_changes)
    }

    async fn process_object(
        &self,
        object_id: &str,
        version: &str,
        previous_version: Option<&str>,
        coin_metadata_cache: &mut HashMap<String, u8>,
    ) -> Result<Option<DynamicFieldBalanceChange>, ClientError> {
        // Get current version
        let current_obj = self.get_past_object(object_id, version).await?;

        // Get previous version
        let previous_obj = if let Some(prev_ver) = previous_version {
            Some(self.get_past_object(object_id, prev_ver).await?)
        } else {
            None
        };

        // Extract balance values
        let current_value = self.extract_balance_value(&current_obj);
        let previous_value = previous_obj.as_ref().and_then(|obj| self.extract_balance_value(obj));

        if let Some(curr_val) = current_value {
            let prev_val = previous_value.unwrap_or(0);
            let diff = curr_val as i128 - prev_val as i128;

            // Extract coin type
            let object_type = self.extract_object_type(&current_obj).unwrap_or_default();
            let coin_type = self.extract_coin_type(&object_type);

            // Get decimals
            let decimals = if let Some(&cached) = coin_metadata_cache.get(&coin_type) {
                cached
            } else {
                let dec = self.get_coin_decimals(&coin_type).await?;
                coin_metadata_cache.insert(coin_type.clone(), dec);
                dec
            };

            return Ok(Some(DynamicFieldBalanceChange {
                coin_type,
                previous_value: prev_val.to_string(),
                current_value: curr_val.to_string(),
                value_diff: diff.to_string(),
                decimals,
            }));
        }

        Ok(None)
    }

    async fn get_past_object(&self, object_id: &str, version: &str) -> Result<PastObjectResponse, ClientError> {
        // Parse version as integer
        let version_int: u64 = version.parse().unwrap_or(0);

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sui_tryGetPastObject",
            "params": [
                object_id,
                version_int,
                {
                    "showContent": true,
                },
            ],
        });

        let response = self.http.post(&self.rpc_url).json(&payload).send().await?;
        let rpc_response: RpcResponse<PastObjectResponse> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(ClientError::Rpc {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }

        rpc_response.result.ok_or(ClientError::MissingResult)
    }

    fn extract_balance_value(&self, obj: &PastObjectResponse) -> Option<u64> {
        if obj.status != "VersionFound" {
            return None;
        }

        let details = obj.details.as_ref()?;
        let content = details.content.as_ref()?;

        if content.data_type != "moveObject" {
            return None;
        }

        let fields = content.fields.as_ref()?;

        // Try direct balance field
        if let Some(balance) = fields.get("balance") {
            if let Some(val) = balance.as_u64() {
                return Some(val);
            }
            if let Some(s) = balance.as_str() {
                if let Ok(val) = s.parse::<u64>() {
                    return Some(val);
                }
            }
            if let Some(obj) = balance.as_object() {
                if let Some(val) = obj.get("value").and_then(|v| v.as_u64()) {
                    return Some(val);
                }
            }
        }

        // Try value field (could be nested object like Coin<T>)
        if let Some(value) = fields.get("value") {
            if let Some(val) = value.as_u64() {
                return Some(val);
            }
            if let Some(s) = value.as_str() {
                if let Ok(val) = s.parse::<u64>() {
                    return Some(val);
                }
            }
            if let Some(obj) = value.as_object() {
                if let Some(nested_fields) = obj.get("fields").and_then(|f| f.as_object()) {
                    if let Some(val) = nested_fields.get("balance") {
                        if let Some(num) = val.as_u64() {
                            return Some(num);
                        }
                        if let Some(s) = val.as_str() {
                            if let Ok(num) = s.parse::<u64>() {
                                return Some(num);
                            }
                        }
                    }
                    if let Some(val) = nested_fields.get("value") {
                        if let Some(num) = val.as_u64() {
                            return Some(num);
                        }
                        if let Some(s) = val.as_str() {
                            if let Ok(num) = s.parse::<u64>() {
                                return Some(num);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn extract_object_type(&self, obj: &PastObjectResponse) -> Option<String> {
        if obj.status != "VersionFound" {
            return None;
        }

        obj.details
            .as_ref()?
            .content
            .as_ref()?
            .type_
            .clone()
    }

    fn extract_coin_type(&self, type_str: &str) -> String {
        // Extract coin type from patterns like:
        // "0x2::coin::Coin<0x2::sui::SUI>"
        // "0x2::dynamic_field::Field<0x1::type_name::TypeName, 0x2::coin::Coin<0x2::sui::SUI>>"
        if let Some(captures) = regex::Regex::new(r"(?:coin::Coin|balance::Balance)<([^>]+)>")
            .ok()
            .and_then(|re| re.captures(type_str))
        {
            if let Some(coin_type) = captures.get(1) {
                return coin_type.as_str().to_string();
            }
        }

        type_str.to_string()
    }

    async fn get_coin_decimals(&self, coin_type: &str) -> Result<u8, ClientError> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "suix_getCoinMetadata",
            "params": [coin_type],
        });

        let response = self.http.post(&self.rpc_url).json(&payload).send().await?;
        let rpc_response: RpcResponse<CoinMetadata> = response.json().await?;

        if let Some(error) = rpc_response.error {
            eprintln!("Failed to get decimals for {}: {:?}", coin_type, error);
            return Ok(0);
        }

        Ok(rpc_response.result.and_then(|m| m.decimals).unwrap_or(0))
    }
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    #[allow(dead_code)]
    pub jsonrpc: String,
    #[allow(dead_code)]
    pub id: Value,
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::POST;
    use std::net::TcpListener;

    fn load_fixture_body() -> Value {
        let raw = include_str!("../../fixtures/transaction_block.json");
        serde_json::from_str(raw).expect("fixture should parse")
    }

    fn ensure_socket_permission() -> bool {
        TcpListener::bind("127.0.0.1:0").is_ok()
    }

    #[tokio::test]
    async fn requests_transaction_and_parses_response() {
        if !ensure_socket_permission() {
            eprintln!("skipping requests_transaction_and_parses_response: binding to localhost is not permitted in this environment");
            return;
        }

        let server = MockServer::start_async().await;
        let response_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": load_fixture_body(),
        });

        let mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/")
                .json_body(json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "sui_getTransactionBlock",
                    "params": [
                        "0xdead",
                        {
                            "showBalanceChanges": true,
                            "showEvents": true,
                            "showEffects": true,
                        },
                    ],
                }));
            then.status(200).json_body(response_body.clone());
        }).await;

        let client = TxParseClient::new(server.base_url());
        let result = client.parse_transaction("0xdead").await.expect("call should succeed");

        mock.assert_async().await;
        assert_eq!(result.balance_changes.len(), 5);
        assert_eq!(result.balance_changes[0].owner, "0x6f4d3a");
    }

    #[tokio::test]
    async fn bubbles_up_rpc_errors() {
        if !ensure_socket_permission() {
            eprintln!("skipping bubbles_up_rpc_errors: binding to localhost is not permitted in this environment");
            return;
        }

        let server = MockServer::start_async().await;
        let mock = server.mock_async(|when, then| {
            when.method(POST);
            then.status(200).json_body(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32000,
                    "message": "Transaction not found",
                }
            }));
        }).await;

        let client = TxParseClient::new(server.base_url());
        let err = client
            .parse_transaction("0xmissing")
            .await
            .expect_err("should return rpc error");

        mock.assert_async().await;
        match err {
            ClientError::Rpc { message, .. } => assert_eq!(message, "Transaction not found"),
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
