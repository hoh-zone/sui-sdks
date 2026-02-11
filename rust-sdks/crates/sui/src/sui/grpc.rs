use std::time::Duration;
use tonic::codegen::http::Uri;
use tonic::transport::{Channel, Endpoint};

use super::utils;

#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("invalid grpc endpoint: {0}")]
    InvalidEndpoint(#[from] tonic::codegen::http::uri::InvalidUri),
    #[error("grpc transport failed: {0}")]
    Transport(#[from] tonic::transport::Error),
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub network: String,
    pub base_url: Option<String>,
    pub timeout: Duration,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            base_url: None,
            timeout: Duration::from_secs(30),
        }
    }
}

pub struct Client {
    network: String,
    endpoint: Endpoint,
    channel: Channel,
}

impl Client {
    pub async fn connect(mut opts: ClientOptions) -> Result<Self, GrpcError> {
        let target = opts
            .base_url
            .take()
            .unwrap_or_else(|| default_grpc_fullnode_url(&opts.network));
        let uri = target.parse::<Uri>()?;

        let endpoint = Endpoint::from(uri).timeout(opts.timeout);
        let channel = endpoint.connect().await?;

        Ok(Self {
            network: opts.network,
            endpoint,
            channel,
        })
    }

    pub fn network(&self) -> &str {
        &self.network
    }

    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    pub fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

pub fn default_grpc_fullnode_url(network: &str) -> String {
    utils::grpc_fullnode_url(network)
        .unwrap_or("https://fullnode.testnet.sui.io:443")
        .to_string()
}
