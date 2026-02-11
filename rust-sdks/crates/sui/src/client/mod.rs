use std::time::Duration;

use crate::{graphql, grpc, jsonrpc};
use crate::utils;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    JsonRpc(#[from] jsonrpc::JsonRpcError),
    #[error(transparent)]
    Grpc(#[from] grpc::GrpcError),
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub network: String,
    pub jsonrpc_url: Option<String>,
    pub graphql_url: Option<String>,
    pub grpc_url: Option<String>,
    pub grpc_timeout: Duration,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            jsonrpc_url: None,
            graphql_url: None,
            grpc_url: None,
            grpc_timeout: Duration::from_secs(30),
        }
    }
}

pub struct Client {
    pub jsonrpc: jsonrpc::Client,
    pub graphql: graphql::Client,
    pub grpc: grpc::Client,
}

impl Client {
    pub async fn new(opts: ClientOptions) -> Result<Self, ClientError> {
        let jsonrpc_url = match opts.jsonrpc_url {
            Some(url) => url,
            None => jsonrpc::default_jsonrpc_fullnode_url(&opts.network)?,
        };
        let graphql_url = opts
            .graphql_url
            .unwrap_or_else(|| default_graphql_url(&opts.network));

        let grpc = grpc::Client::connect(grpc::ClientOptions {
            network: opts.network.clone(),
            base_url: opts.grpc_url,
            timeout: opts.grpc_timeout,
        })
        .await?;

        Ok(Self {
            jsonrpc: jsonrpc::Client::new(jsonrpc_url, opts.network.clone()),
            graphql: graphql::Client::new(graphql_url, opts.network),
            grpc,
        })
    }
}

fn default_graphql_url(network: &str) -> String {
    utils::graphql_url(network)
        .unwrap_or("https://sui-testnet.mystenlabs.com/graphql")
        .to_string()
}
