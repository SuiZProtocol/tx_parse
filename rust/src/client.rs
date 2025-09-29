use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;

use crate::parse::{parse_transaction, ParseError};
use crate::types::{ParseResult, TransactionBlockResponse};

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
