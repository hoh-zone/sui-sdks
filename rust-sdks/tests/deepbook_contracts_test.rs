use std::collections::HashMap;

use sui_sdks_rust::deepbook_v3::config::DeepBookConfig;
use sui_sdks_rust::deepbook_v3::contracts::{
    BalanceManagerContract, DeepBookContract, GovernanceContract, MarginManagerContract,
    MarginPoolContract, MarginRegistryContract, MarginTPSLContract,
};
use sui_sdks_rust::deepbook_v3::types::{BalanceManager, MarginManager};
use sui_sdks_rust::sui::transactions::Transaction;

fn new_test_config() -> DeepBookConfig {
    let mut cfg = DeepBookConfig::default();
    let mut bms = HashMap::new();
    bms.insert(
        "m1".to_string(),
        BalanceManager {
            address: "0x2".to_string(),
            trade_cap: None,
        },
    );
    let mut mms = HashMap::new();
    mms.insert(
        "mm1".to_string(),
        MarginManager {
            address: "0x3".to_string(),
            pool_key: "DEEP_SUI".to_string(),
        },
    );
    cfg.balance_managers = bms;
    cfg.margin_managers = mms;
    cfg
}

fn command_function(tx: &Transaction, idx: usize) -> String {
    let cmd = &tx.data.commands[idx];
    cmd.get("MoveCall")
        .and_then(|v| v.get("function"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

#[test]
fn deepbook_contract_method_targets() {
    let cfg = new_test_config();
    let bm = BalanceManagerContract { config: &cfg };
    let contract = DeepBookContract {
        config: &cfg,
        balance_manager: bm,
    };

    let mut tx = Transaction::new();
    contract
        .get_quote_quantity_out(&mut tx, "DEEP_SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx, 0), "get_quote_quantity_out");

    let mut tx2 = Transaction::new();
    contract
        .can_place_limit_order(&mut tx2, "DEEP_SUI", "m1", 1.0, 1.0, true, true, Some(100))
        .expect("call should succeed");
    assert_eq!(command_function(&tx2, 0), "can_place_limit_order");

    let mut tx3 = Transaction::new();
    contract
        .cancel_orders(&mut tx3, "DEEP_SUI", "m1", &[1, 2])
        .expect("call should succeed");
    let last = tx3.data.commands.len() - 1;
    assert_eq!(command_function(&tx3, last), "cancel_orders");

    let mut tx4 = Transaction::new();
    contract
        .get_quantity_out_input_fee(&mut tx4, "DEEP_SUI", 1.0, 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx4, 0), "get_quantity_out_input_fee");

    let mut tx5 = Transaction::new();
    contract
        .get_base_quantity_in(&mut tx5, "DEEP_SUI", 1.0, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx5, 0), "get_base_quantity_in");

    let mut tx6 = Transaction::new();
    contract
        .get_order_deep_required(&mut tx6, "DEEP_SUI", 1.0, 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx6, 0), "get_order_deep_required");

    let mut tx7 = Transaction::new();
    contract
        .pool_trade_params_next(&mut tx7, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx7, 0), "pool_trade_params_next");

    let mut tx8 = Transaction::new();
    let bm2 = BalanceManagerContract { config: &cfg };
    bm2.balance_manager_referral_owner(&mut tx8, "0x4")
        .expect("call should succeed");
    assert_eq!(command_function(&tx8, 0), "balance_manager_referral_owner");

    let mut tx9 = Transaction::new();
    contract
        .get_level2_range(&mut tx9, "DEEP_SUI", 1.0, 2.0, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx9, 0), "get_level2_range");

    let mut tx10 = Transaction::new();
    contract
        .get_level2_ticks_from_mid(&mut tx10, "DEEP_SUI", 10)
        .expect("call should succeed");
    assert_eq!(command_function(&tx10, 0), "get_level2_ticks_from_mid");

    let mut tx11 = Transaction::new();
    contract
        .account_exists(&mut tx11, "DEEP_SUI", "m1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx11, 0), "account_exists");

    let mut tx12 = Transaction::new();
    contract.quorum(&mut tx12, "DEEP_SUI").expect("call should succeed");
    assert_eq!(command_function(&tx12, 0), "quorum");

    let mut tx13 = Transaction::new();
    contract.pool_id(&mut tx13, "DEEP_SUI").expect("call should succeed");
    assert_eq!(command_function(&tx13, 0), "id");

    let mut tx14 = Transaction::new();
    contract
        .can_place_market_order(&mut tx14, "DEEP_SUI", "m1", 1.0, true, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx14, 0), "can_place_market_order");

    let mut tx15 = Transaction::new();
    contract
        .check_market_order_params(&mut tx15, "DEEP_SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx15, 0), "check_market_order_params");

    let mut tx16 = Transaction::new();
    contract
        .stable_pool(&mut tx16, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx16, 0), "stable_pool");

    let mut tx17 = Transaction::new();
    contract
        .registered_pool(&mut tx17, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx17, 0), "registered_pool");

    let mut tx18 = Transaction::new();
    contract
        .get_pool_referral_balances(&mut tx18, "DEEP_SUI", "0x4")
        .expect("call should succeed");
    assert_eq!(command_function(&tx18, 0), "get_pool_referral_balances");

    let mut tx19 = Transaction::new();
    contract
        .pool_referral_multiplier(&mut tx19, "DEEP_SUI", "0x4")
        .expect("call should succeed");
    assert_eq!(command_function(&tx19, 0), "pool_referral_multiplier");

    let mut tx20 = Transaction::new();
    contract
        .get_balance_manager_ids(&mut tx20, "0x1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx20, 0), "get_balance_manager_ids");

    let mut tx21 = Transaction::new();
    contract
        .check_limit_order_params(&mut tx21, "DEEP_SUI", 1.0, 1.0, 100)
        .expect("call should succeed");
    assert_eq!(command_function(&tx21, 0), "check_limit_order_params");

    let mut tx22 = Transaction::new();
    let bm3 = BalanceManagerContract { config: &cfg };
    bm3.balance_manager_referral_pool_id(&mut tx22, "0x4")
        .expect("call should succeed");
    assert_eq!(command_function(&tx22, 0), "balance_manager_referral_pool_id");

    let mut tx23 = Transaction::new();
    bm3.get_balance_manager_referral_id(&mut tx23, "m1", "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23, 0), "get_balance_manager_referral_id");

    let margin_pool = MarginPoolContract { config: &cfg };

    let mut tx24 = Transaction::new();
    margin_pool
        .get_id(&mut tx24, "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx24, 0), "id");

    let mut tx25 = Transaction::new();
    margin_pool
        .deepbook_pool_allowed(&mut tx25, "SUI", "0x5")
        .expect("call should succeed");
    assert_eq!(command_function(&tx25, 0), "deepbook_pool_allowed");

    let mut tx26 = Transaction::new();
    margin_pool
        .interest_rate(&mut tx26, "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx26, 0), "interest_rate");

    let mut tx27 = Transaction::new();
    margin_pool
        .user_supply_amount(&mut tx27, "SUI", "0x6")
        .expect("call should succeed");
    assert_eq!(command_function(&tx27, 0), "user_supply_amount");

    let margin_manager = MarginManagerContract { config: &cfg };

    let mut tx28 = Transaction::new();
    margin_manager
        .owner(&mut tx28, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx28, 0), "owner");

    let mut tx29 = Transaction::new();
    margin_manager
        .borrowed_shares(&mut tx29, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx29, 0), "borrowed_shares");

    let mut tx30 = Transaction::new();
    margin_manager
        .balance_manager(&mut tx30, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx30, 0), "balance_manager");

    let mut tx31 = Transaction::new();
    margin_manager
        .calculate_assets(&mut tx31, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx31, 0), "calculate_assets");

    let mut tx32 = Transaction::new();
    margin_manager
        .calculate_debts(&mut tx32, "mm1", "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx32, 0), "calculate_debts");

    let mut tx33 = Transaction::new();
    margin_manager
        .base_balance(&mut tx33, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx33, 0), "base_balance");

    let mut tx34 = Transaction::new();
    margin_manager
        .quote_balance(&mut tx34, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx34, 0), "quote_balance");

    let mut tx35 = Transaction::new();
    margin_manager
        .deep_balance(&mut tx35, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx35, 0), "deep_balance");

    let mut tx36 = Transaction::new();
    margin_manager
        .manager_state(&mut tx36, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx36, 0), "manager_state");

    let margin_tpsl = MarginTPSLContract { config: &cfg };

    let mut tx37 = Transaction::new();
    margin_tpsl
        .conditional_order_ids(&mut tx37, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx37, 0), "conditional_order_ids");

    let mut tx38 = Transaction::new();
    margin_tpsl
        .lowest_trigger_above_price(&mut tx38, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx38, 0), "lowest_trigger_above_price");

    let mut tx39 = Transaction::new();
    margin_tpsl
        .highest_trigger_below_price(&mut tx39, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx39, 0), "highest_trigger_below_price");

    let margin_registry = MarginRegistryContract { config: &cfg };

    let mut tx40 = Transaction::new();
    margin_registry
        .pool_enabled(&mut tx40, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx40, 0), "pool_enabled");

    let mut tx41 = Transaction::new();
    margin_registry
        .get_margin_manager_ids(&mut tx41, "0x1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx41, 0), "get_margin_manager_ids");

    let mut tx42 = Transaction::new();
    margin_registry
        .base_margin_pool_id(&mut tx42, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx42, 0), "base_margin_pool_id");

    let mut tx43 = Transaction::new();
    margin_registry
        .quote_margin_pool_id(&mut tx43, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx43, 0), "quote_margin_pool_id");

    let mut tx44 = Transaction::new();
    margin_registry
        .min_withdraw_risk_ratio(&mut tx44, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx44, 0), "min_withdraw_risk_ratio");

    let mut tx45 = Transaction::new();
    margin_registry
        .min_borrow_risk_ratio(&mut tx45, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx45, 0), "min_borrow_risk_ratio");

    let mut tx46 = Transaction::new();
    margin_registry
        .liquidation_risk_ratio(&mut tx46, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx46, 0), "liquidation_risk_ratio");

    let mut tx47 = Transaction::new();
    margin_registry
        .target_liquidation_risk_ratio(&mut tx47, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx47, 0), "target_liquidation_risk_ratio");

    let mut tx48 = Transaction::new();
    margin_registry
        .user_liquidation_reward(&mut tx48, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx48, 0), "user_liquidation_reward");

    let mut tx49 = Transaction::new();
    margin_registry
        .pool_liquidation_reward(&mut tx49, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx49, 0), "pool_liquidation_reward");

    let mut tx50 = Transaction::new();
    margin_registry
        .allowed_maintainers(&mut tx50)
        .expect("call should succeed");
    assert_eq!(command_function(&tx50, 0), "allowed_maintainers");

    let mut tx51 = Transaction::new();
    margin_registry
        .allowed_pause_caps(&mut tx51)
        .expect("call should succeed");
    assert_eq!(command_function(&tx51, 0), "allowed_pause_caps");
}

#[test]
fn governance_vote_target() {
    let cfg = new_test_config();
    let bm = BalanceManagerContract { config: &cfg };
    let governance = GovernanceContract {
        config: &cfg,
        balance_manager: bm,
    };

    let mut tx = Transaction::new();
    governance
        .vote(&mut tx, "DEEP_SUI", "m1", 7)
        .expect("vote should succeed");

    let last = tx.data.commands.len() - 1;
    assert_eq!(command_function(&tx, last), "vote");
}
