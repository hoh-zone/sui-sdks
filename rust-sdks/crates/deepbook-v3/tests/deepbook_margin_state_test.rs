use base64::Engine as _;
use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;

use deepbook_v3::client::DeepBookClient;
use deepbook_v3::config::DeepBookConfig;
use deepbook_v3::types::MarginManager;
use sui::jsonrpc;

fn b64_u64(v: u64) -> String {
    base64::engine::general_purpose::STANDARD.encode(v.to_le_bytes())
}

fn b64_u8(v: u8) -> String {
    base64::engine::general_purpose::STANDARD.encode([v])
}

fn b64_addr(fill: u8) -> String {
    base64::engine::general_purpose::STANDARD.encode(vec![fill; 32])
}

fn state_return_values(seed: u64) -> serde_json::Value {
    json!([
        {"bcs": b64_addr((seed % 255) as u8 + 1)},
        {"bcs": b64_addr(((seed + 1) % 255) as u8 + 1)},
        {"bcs": b64_u64(seed + 2)},
        {"bcs": b64_u64(seed + 3)},
        {"bcs": b64_u64(seed + 4)},
        {"bcs": b64_u64(seed + 5)},
        {"bcs": b64_u64(seed + 6)},
        {"bcs": b64_u64(seed + 7)},
        {"bcs": b64_u8(9)},
        {"bcs": b64_u64(seed + 10)},
        {"bcs": b64_u8(10)},
        {"bcs": b64_u64(seed + 12)},
        {"bcs": b64_u64(seed + 13)},
        {"bcs": b64_u64(seed + 14)}
    ])
}

#[tokio::test]
async fn deepbook_margin_manager_state_methods() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("\"method\":\"sui_dryRunTransactionBlock\"");
        then.status(200).header("content-type", "application/json").json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "commandResults": [
                    {"returnValues": state_return_values(100)},
                    {"returnValues": state_return_values(200)}
                ]
            }
        }));
    });

    let mut cfg = DeepBookConfig::default();
    cfg.margin_managers.insert(
        "mm1".to_string(),
        MarginManager {
            address: "0x3".to_string(),
            pool_key: "DEEP_SUI".to_string(),
        },
    );
    cfg.margin_managers.insert(
        "mm2".to_string(),
        MarginManager {
            address: "0x8".to_string(),
            pool_key: "DEEP_SUI".to_string(),
        },
    );
    if let Some(deep) = cfg.coins.get_mut("DEEP") {
        deep.price_info_object_id = Some("0x9".to_string());
    }
    if let Some(sui) = cfg.coins.get_mut("SUI") {
        sui.price_info_object_id = Some("0xa".to_string());
    }

    let rpc = jsonrpc::Client::new(server.url("/"), "testnet");
    let client = DeepBookClient::new(rpc, cfg);

    let state = client
        .get_margin_manager_state("mm1", 6)
        .await
        .expect("get_margin_manager_state");
    assert!(state.get("managerId").is_some());
    assert!(state.get("riskRatio").is_some());
    assert!(state.get("currentPrice").is_some());

    let states = client
        .get_margin_manager_states(&["mm1", "mm2"], 6)
        .await
        .expect("get_margin_manager_states");
    assert!(states.as_object().map(|o| !o.is_empty()).unwrap_or(false));
}

