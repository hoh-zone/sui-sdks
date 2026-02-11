use base64::Engine as _;
use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;

use deepbook_v3::client::DeepBookClient;
use deepbook_v3::config::DeepBookConfig;
use deepbook_v3::types::{BalanceManager, MarginManager};
use sui::sui::jsonrpc;

fn b64_u64(v: u64) -> String {
    base64::engine::general_purpose::STANDARD.encode(v.to_le_bytes())
}

fn b64_vec_u64(values: &[u64]) -> String {
    let mut out = Vec::with_capacity(1 + values.len() * 8);
    out.push(values.len() as u8);
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
    base64::engine::general_purpose::STANDARD.encode(out)
}

fn b64_vecset_addresses(addrs: &[&str]) -> String {
    let mut out = Vec::with_capacity(1 + addrs.len() * 32);
    out.push(addrs.len() as u8);
    for addr in addrs {
        let mut hex_str = addr.trim_start_matches("0x").to_string();
        if hex_str.len() % 2 == 1 {
            hex_str = format!("0{hex_str}");
        }
        let mut bytes = hex::decode(hex_str).expect("valid hex address");
        if bytes.len() < 32 {
            let mut padded = vec![0u8; 32 - bytes.len()];
            padded.extend_from_slice(&bytes);
            bytes = padded;
        }
        assert_eq!(bytes.len(), 32, "address must fit 32 bytes");
        out.extend_from_slice(&bytes);
    }
    base64::engine::general_purpose::STANDARD.encode(out)
}

#[tokio::test]
async fn deepbook_client_quantity_and_whitelist_methods() {
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
                    {"returnValues": [
                        {"bcs": b64_u64(100)},
                        {"bcs": b64_u64(200)},
                        {"bcs": b64_u64(300)}
                    ]},
                    {"returnValues": [{"bcs": base64::engine::general_purpose::STANDARD.encode([1u8])}]}
                ]
            }
        }));
    });

    let mut cfg = DeepBookConfig::default();
    cfg.balance_managers.insert(
        "m1".to_string(),
        BalanceManager {
            address: "0x2".to_string(),
            trade_cap: None,
        },
    );
    cfg.margin_managers.insert(
        "mm1".to_string(),
        MarginManager {
            address: "0x3".to_string(),
            pool_key: "DEEP_SUI".to_string(),
        },
    );
    if let Some(sui) = cfg.coins.get_mut("SUI") {
        sui.price_info_object_id = Some("0x99".to_string());
    }

    let rpc = jsonrpc::Client::new(server.url("/"), "testnet");
    let client = DeepBookClient::new(rpc, cfg);

    let q = client
        .get_quote_quantity_out("DEEP_SUI", 1.0)
        .await
        .expect("get_quote_quantity_out");
    assert!(q.get("deepRequired").is_some());

    let q2 = client
        .get_base_quantity_out("DEEP_SUI", 1.0)
        .await
        .expect("get_base_quantity_out");
    assert!(q2.get("deepRequired").is_some());

    let q3 = client
        .get_quantity_out("DEEP_SUI", 1.0, 1.0)
        .await
        .expect("get_quantity_out");
    assert!(q3.get("deepRequired").is_some());

    let mid = client.mid_price("DEEP_SUI").await.expect("mid_price");
    assert!(mid > 0.0);

    let b = client.whitelisted("DEEP_SUI").await.expect("whitelisted");
    // This mock returns u64 bytes in index 0, so b is false in this baseline test.
    assert!(!b);

    let bal = client
        .check_manager_balance("m1", "SUI")
        .await
        .expect("check_manager_balance");
    assert!(bal.get("balance").is_some());

    let raw_order = client.get_order("DEEP_SUI", 1).await.expect("get_order");
    assert!(!raw_order.is_empty());

    let normalized_order = client
        .get_order_normalized("DEEP_SUI", 1u128 << 80)
        .await
        .expect("get_order_normalized");
    assert!(normalized_order.get("normalizedPrice").is_some());
    assert!(normalized_order.get("rawOrderBcs").is_some());

    let raw_orders = client
        .get_orders("DEEP_SUI", &[1, 2])
        .await
        .expect("get_orders");
    assert!(!raw_orders.is_empty());

    let open_orders = client
        .account_open_orders("DEEP_SUI", "m1")
        .await
        .expect("account_open_orders");
    assert!(!open_orders.is_empty());

    let vault = client
        .vault_balances("DEEP_SUI")
        .await
        .expect("vault_balances");
    assert!(!vault.is_empty());

    let pool_id = client
        .get_pool_id_by_assets("0x2::sui::SUI", "0x2::sui::SUI")
        .await
        .expect("get_pool_id_by_assets");
    assert!(!pool_id.is_empty());

    let trade_params = client
        .pool_trade_params("DEEP_SUI")
        .await
        .expect("pool_trade_params");
    assert!(!trade_params.is_empty());

    let book_params = client
        .pool_book_params("DEEP_SUI")
        .await
        .expect("pool_book_params");
    assert!(!book_params.is_empty());

    let account = client.account("DEEP_SUI", "m1").await.expect("account");
    assert!(!account.is_empty());

    let locked = client
        .locked_balance("DEEP_SUI", "m1")
        .await
        .expect("locked_balance");
    assert!(!locked.is_empty());

    let deep_price = client
        .get_pool_deep_price("DEEP_SUI")
        .await
        .expect("get_pool_deep_price");
    assert!(!deep_price.is_empty());

    let referral_owner = client
        .balance_manager_referral_owner("0x4")
        .await
        .expect("balance_manager_referral_owner");
    assert!(!referral_owner.is_empty());

    let price_info_age = client
        .get_price_info_object_age("SUI")
        .await
        .expect("get_price_info_object_age");
    assert!(price_info_age > 0);

    let no_price_info_age = client
        .get_price_info_object_age("DEEP")
        .await
        .expect("get_price_info_object_age for missing");
    assert_eq!(no_price_info_age, -1);

    let price_info_obj = client
        .get_price_info_object("SUI")
        .await
        .expect("get_price_info_object");
    assert!(!price_info_obj.is_empty());

    let price_info_objs = client
        .get_price_info_objects(&["SUI"])
        .await
        .expect("get_price_info_objects");
    assert!(price_info_objs.get("SUI").is_some());

    let qif = client
        .get_quote_quantity_out_input_fee("DEEP_SUI", 1.0)
        .await
        .expect("get_quote_quantity_out_input_fee");
    assert!(qif.get("result").is_some());

    let bif = client
        .get_base_quantity_out_input_fee("DEEP_SUI", 1.0)
        .await
        .expect("get_base_quantity_out_input_fee");
    assert!(bif.get("result").is_some());

    let qout_if = client
        .get_quantity_out_input_fee("DEEP_SUI", 1.0, 1.0)
        .await
        .expect("get_quantity_out_input_fee");
    assert!(qout_if.get("result").is_some());

    let base_in = client
        .get_base_quantity_in("DEEP_SUI", 1.0, true)
        .await
        .expect("get_base_quantity_in");
    assert!(base_in > 0);

    let quote_in = client
        .get_quote_quantity_in("DEEP_SUI", 1.0, true)
        .await
        .expect("get_quote_quantity_in");
    assert!(quote_in > 0);

    let account_details = client
        .get_account_order_details("DEEP_SUI", "m1")
        .await
        .expect("get_account_order_details");
    assert!(!account_details.is_empty());

    let order_deep_required = client
        .get_order_deep_required("DEEP_SUI", 1.0, 1.0)
        .await
        .expect("get_order_deep_required");
    assert!(order_deep_required > 0);

    let trade_params_next = client
        .pool_trade_params_next("DEEP_SUI")
        .await
        .expect("pool_trade_params_next");
    assert!(!trade_params_next.is_empty());

    let level2_range = client
        .get_level2_range("DEEP_SUI", 1.0, 2.0, true)
        .await
        .expect("get_level2_range");
    assert!(!level2_range.is_empty());

    let level2_ticks = client
        .get_level2_ticks_from_mid("DEEP_SUI", 10)
        .await
        .expect("get_level2_ticks_from_mid");
    assert!(!level2_ticks.is_empty());

    let account_exists = client
        .account_exists("DEEP_SUI", "m1")
        .await
        .expect("account_exists");
    assert!(!account_exists);

    let quorum = client.quorum("DEEP_SUI").await.expect("quorum");
    assert!(!quorum.is_empty());

    let pool_id = client.pool_id("DEEP_SUI").await.expect("pool_id");
    assert!(!pool_id.is_empty());

    let stable = client.stable_pool("DEEP_SUI").await.expect("stable_pool");
    assert!(!stable);

    let registered = client
        .registered_pool("DEEP_SUI")
        .await
        .expect("registered_pool");
    assert!(!registered);

    let can_limit = client
        .can_place_limit_order("DEEP_SUI", "m1", 1.0, 1.0, true, true, 100)
        .await
        .expect("can_place_limit_order");
    assert!(!can_limit);

    let can_market = client
        .can_place_market_order("DEEP_SUI", "m1", 1.0, true, true)
        .await
        .expect("can_place_market_order");
    assert!(!can_market);

    let valid_market = client
        .check_market_order_params("DEEP_SUI", 1.0)
        .await
        .expect("check_market_order_params");
    assert!(!valid_market);

    let valid_limit = client
        .check_limit_order_params("DEEP_SUI", 1.0, 1.0, 100)
        .await
        .expect("check_limit_order_params");
    assert!(!valid_limit);

    let manager_ids = client
        .get_balance_manager_ids("0x1")
        .await
        .expect("get_balance_manager_ids");
    assert!(!manager_ids.is_empty());

    let referral_balances = client
        .get_pool_referral_balances("DEEP_SUI", "0x4")
        .await
        .expect("get_pool_referral_balances");
    assert!(referral_balances.get("base").is_some());

    let referral_pool_id = client
        .balance_manager_referral_pool_id("0x4")
        .await
        .expect("balance_manager_referral_pool_id");
    assert!(!referral_pool_id.is_empty());

    let referral_multiplier = client
        .pool_referral_multiplier("DEEP_SUI", "0x4")
        .await
        .expect("pool_referral_multiplier");
    assert!(referral_multiplier > 0.0);

    let referral_id = client
        .get_balance_manager_referral_id("m1", "DEEP_SUI")
        .await
        .expect("get_balance_manager_referral_id");
    assert!(!referral_id.is_empty());

    let (is_bid, price, order_id) = client.decode_order_id(1u128 << 80);
    assert!(is_bid);
    assert!(price > 0);
    assert_eq!(order_id, 0);

    let margin_pool_id = client
        .get_margin_pool_id("SUI")
        .await
        .expect("get_margin_pool_id");
    assert!(!margin_pool_id.is_empty());

    let deepbook_pool_margin_pool_ids = client
        .get_deepbook_pool_margin_pool_ids("DEEP_SUI")
        .await
        .expect("get_deepbook_pool_margin_pool_ids");
    assert!(!deepbook_pool_margin_pool_ids.is_empty());

    let pool_allowed = client
        .is_deepbook_pool_allowed("SUI", "0x5")
        .await
        .expect("is_deepbook_pool_allowed");
    assert!(!pool_allowed);

    let total_supply = client
        .get_margin_pool_total_supply("SUI", 6)
        .await
        .expect("get_margin_pool_total_supply");
    assert!(!total_supply.is_empty());

    let supply_shares = client
        .get_margin_pool_supply_shares("SUI", 6)
        .await
        .expect("get_margin_pool_supply_shares");
    assert!(!supply_shares.is_empty());

    let total_borrow = client
        .get_margin_pool_total_borrow("SUI", 6)
        .await
        .expect("get_margin_pool_total_borrow");
    assert!(!total_borrow.is_empty());

    let borrow_shares = client
        .get_margin_pool_borrow_shares("SUI", 6)
        .await
        .expect("get_margin_pool_borrow_shares");
    assert!(!borrow_shares.is_empty());

    let last_update = client
        .get_margin_pool_last_update_timestamp("SUI")
        .await
        .expect("get_margin_pool_last_update_timestamp");
    assert!(last_update > 0);

    let supply_cap = client
        .get_margin_pool_supply_cap("SUI", 6)
        .await
        .expect("get_margin_pool_supply_cap");
    assert!(!supply_cap.is_empty());

    let max_utilization = client
        .get_margin_pool_max_utilization_rate("SUI")
        .await
        .expect("get_margin_pool_max_utilization_rate");
    assert!(max_utilization > 0.0);

    let protocol_spread = client
        .get_margin_pool_protocol_spread("SUI")
        .await
        .expect("get_margin_pool_protocol_spread");
    assert!(protocol_spread > 0.0);

    let min_borrow = client
        .get_margin_pool_min_borrow("SUI", 6)
        .await
        .expect("get_margin_pool_min_borrow");
    assert!(!min_borrow.is_empty());

    let interest_rate = client
        .get_margin_pool_interest_rate("SUI")
        .await
        .expect("get_margin_pool_interest_rate");
    assert!(interest_rate > 0.0);

    let user_supply_shares = client
        .get_user_supply_shares("SUI", "0x6", 6)
        .await
        .expect("get_user_supply_shares");
    assert!(!user_supply_shares.is_empty());

    let user_supply_amount = client
        .get_user_supply_amount("SUI", "0x6", 6)
        .await
        .expect("get_user_supply_amount");
    assert!(!user_supply_amount.is_empty());

    let margin_owner = client
        .get_margin_manager_owner("mm1")
        .await
        .expect("get_margin_manager_owner");
    assert!(!margin_owner.is_empty());

    let margin_deepbook_pool = client
        .get_margin_manager_deepbook_pool("mm1")
        .await
        .expect("get_margin_manager_deepbook_pool");
    assert!(!margin_deepbook_pool.is_empty());

    let margin_pool_id = client
        .get_margin_manager_margin_pool_id("mm1")
        .await
        .expect("get_margin_manager_margin_pool_id");
    assert!(!margin_pool_id.is_empty());

    let borrowed_shares = client
        .get_margin_manager_borrowed_shares("mm1")
        .await
        .expect("get_margin_manager_borrowed_shares");
    assert!(borrowed_shares.get("baseShares").is_some());
    assert!(borrowed_shares.get("quoteShares").is_some());

    let borrowed_base = client
        .get_margin_manager_borrowed_base_shares("mm1")
        .await
        .expect("get_margin_manager_borrowed_base_shares");
    assert!(!borrowed_base.is_empty());

    let borrowed_quote = client
        .get_margin_manager_borrowed_quote_shares("mm1")
        .await
        .expect("get_margin_manager_borrowed_quote_shares");
    assert!(!borrowed_quote.is_empty());

    let has_base_debt = client
        .get_margin_manager_has_base_debt("mm1")
        .await
        .expect("get_margin_manager_has_base_debt");
    assert!(!has_base_debt);

    let manager_id = client
        .get_margin_manager_balance_manager_id("mm1")
        .await
        .expect("get_margin_manager_balance_manager_id");
    assert!(!manager_id.is_empty());

    let margin_assets = client
        .get_margin_manager_assets("mm1", 6)
        .await
        .expect("get_margin_manager_assets");
    assert!(margin_assets.get("baseAsset").is_some());
    assert!(margin_assets.get("quoteAsset").is_some());

    let margin_debts = client
        .get_margin_manager_debts("mm1", 6)
        .await
        .expect("get_margin_manager_debts");
    assert!(margin_debts.get("baseDebt").is_some());
    assert!(margin_debts.get("quoteDebt").is_some());

    let base_balance = client
        .get_margin_manager_base_balance("mm1", 6)
        .await
        .expect("get_margin_manager_base_balance");
    assert!(!base_balance.is_empty());

    let quote_balance = client
        .get_margin_manager_quote_balance("mm1", 6)
        .await
        .expect("get_margin_manager_quote_balance");
    assert!(!quote_balance.is_empty());

    let deep_balance = client
        .get_margin_manager_deep_balance("mm1", 6)
        .await
        .expect("get_margin_manager_deep_balance");
    assert!(!deep_balance.is_empty());

    // commandResults[1][0] exists in mock payload, enough for baseline parser test.
    let raw_margin = client
        .get_margin_account_order_details("mm1")
        .await
        .expect("get_margin_account_order_details");
    assert!(!raw_margin.is_empty());
}

#[tokio::test]
async fn deepbook_client_tpsl_and_margin_registry_methods() {
    let mut cfg = DeepBookConfig::default();
    cfg.margin_managers.insert(
        "mm1".to_string(),
        MarginManager {
            address: "0x3".to_string(),
            pool_key: "DEEP_SUI".to_string(),
        },
    );

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200).json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "commandResults": [ { "returnValues": [ { "bcs": b64_vec_u64(&[11, 22]) } ] } ] }
        }));
    });
    let client = DeepBookClient::new(jsonrpc::Client::new(server.url("/"), "testnet"), cfg.clone());
    let order_ids = client
        .get_conditional_order_ids("mm1")
        .await
        .expect("get_conditional_order_ids");
    assert_eq!(order_ids, vec!["11".to_string(), "22".to_string()]);

    let conditional_order = client
        .get_conditional_order("mm1", 11)
        .await
        .expect("get_conditional_order");
    assert!(!conditional_order.is_empty());

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200).json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "commandResults": [ { "returnValues": [ { "bcs": base64::engine::general_purpose::STANDARD.encode([1u8]) } ] } ] }
        }));
    });
    let client = DeepBookClient::new(jsonrpc::Client::new(server.url("/"), "testnet"), cfg.clone());
    let enabled = client
        .is_pool_enabled_for_margin("DEEP_SUI")
        .await
        .expect("is_pool_enabled_for_margin");
    assert!(enabled);

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200).json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "commandResults": [ { "returnValues": [ { "bcs": b64_u64(1000) } ] } ] }
        }));
    });
    let client = DeepBookClient::new(jsonrpc::Client::new(server.url("/"), "testnet"), cfg.clone());
    let min_withdraw = client
        .get_min_withdraw_risk_ratio("DEEP_SUI")
        .await
        .expect("get_min_withdraw_risk_ratio");
    assert!(min_withdraw > 0.0);

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200).json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "commandResults": [ { "returnValues": [ { "bcs": base64::engine::general_purpose::STANDARD.encode([1u8; 32]) } ] } ] }
        }));
    });
    let client = DeepBookClient::new(jsonrpc::Client::new(server.url("/"), "testnet"), cfg.clone());
    let base_margin_pool_id = client
        .get_base_margin_pool_id("DEEP_SUI")
        .await
        .expect("get_base_margin_pool_id");
    assert!(!base_margin_pool_id.is_empty());

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/");
        then.status(200).json_body(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": { "commandResults": [ { "returnValues": [ { "bcs": b64_vecset_addresses(&["0x1", "0x2"]) } ] } ] }
        }));
    });
    let client = DeepBookClient::new(jsonrpc::Client::new(server.url("/"), "testnet"), cfg);
    let maintainers = client
        .get_allowed_maintainers()
        .await
        .expect("get_allowed_maintainers");
    assert_eq!(maintainers.len(), 2);
}
