use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils;

#[derive(Debug, thiserror::Error)]
pub enum FaucetError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("faucet rate limited")]
    RateLimited,
    #[error("unsupported faucet network: {0}")]
    UnsupportedNetwork(String),
    #[error("faucet request failed: {0}")]
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinInfo {
    pub amount: u64,
    pub id: String,
    #[serde(rename = "transferTxDigest")]
    pub transfer_tx_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetResponseV2 {
    pub status: Value,
    #[serde(default, rename = "coins_sent")]
    pub coins_sent: Vec<CoinInfo>,
}

#[derive(Debug, Clone)]
pub struct FaucetClient {
    endpoint: String,
    client: reqwest::Client,
}

impl FaucetClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn from_network(network: &str) -> Result<Self, FaucetError> {
        Ok(Self::new(get_faucet_host(network)?))
    }

    pub async fn request_sui_from_faucet_v2(
        &self,
        recipient: &str,
        fixed_amount: Option<u64>,
    ) -> Result<FaucetResponseV2, FaucetError> {
        let mut payload = serde_json::json!({
            "FixedAmountRequest": {
                "recipient": recipient,
            }
        });
        if let Some(amount) = fixed_amount {
            payload["FixedAmountRequest"]["amount"] = Value::from(amount);
        }

        let response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(FaucetError::RateLimited);
        }

        let body: FaucetResponseV2 = response.json().await?;
        if body.status == Value::String("Success".to_string()) {
            return Ok(body);
        }

        if let Some(internal) = body
            .status
            .get("Failure")
            .and_then(|v| v.get("internal"))
            .and_then(Value::as_str)
        {
            return Err(FaucetError::Failed(internal.to_string()));
        }

        Err(FaucetError::Failed(body.status.to_string()))
    }
}

pub fn get_faucet_host(network: &str) -> Result<String, FaucetError> {
    utils::faucet_url(network)
        .map(|s| s.to_string())
        .ok_or_else(|| FaucetError::UnsupportedNetwork(network.to_string()))
}
