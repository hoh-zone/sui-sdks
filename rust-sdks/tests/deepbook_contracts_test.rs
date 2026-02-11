use std::collections::HashMap;

use sui_sdks_rust::deepbook_v3::config::DeepBookConfig;
use sui_sdks_rust::deepbook_v3::contracts::{
    BalanceManagerContract, DeepBookAdminContract, DeepBookContract, FlashLoanContract,
    GovernanceContract, MarginAdminContract, MarginLiquidationsContract, MarginMaintainerContract,
    MarginManagerContract, MarginPoolContract, MarginRegistryContract, MarginTPSLContract,
    PoolProxyContract,
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

    let mut tx23b = Transaction::new();
    bm3.create_balance_manager(&mut tx23b)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23b, 0), "new");

    let mut tx23c = Transaction::new();
    bm3.deposit(&mut tx23c, "m1", "SUI", "0x20")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23c, 0), "deposit");

    let mut tx23c2 = Transaction::new();
    bm3.deposit_with_cap(&mut tx23c2, "m1", "SUI", "0x201", "0x20")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23c2, 0), "deposit_with_cap");

    let mut tx23d = Transaction::new();
    bm3.withdraw(&mut tx23d, "m1", "SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23d, 0), "withdraw");

    let mut tx23d2 = Transaction::new();
    bm3.withdraw_with_cap(&mut tx23d2, "m1", "SUI", "0x202", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23d2, 0), "withdraw_with_cap");

    let mut tx23e = Transaction::new();
    bm3.withdraw_all(&mut tx23e, "m1", "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23e, 0), "withdraw_all");

    let mut tx23f = Transaction::new();
    bm3.mint_trade_cap(&mut tx23f, "m1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23f, 0), "mint_trade_cap");

    let mut tx23g = Transaction::new();
    bm3.mint_deposit_cap(&mut tx23g, "m1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23g, 0), "mint_deposit_cap");

    let mut tx23h = Transaction::new();
    bm3.mint_withdraw_cap(&mut tx23h, "m1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23h, 0), "mint_withdraw_cap");

    let mut tx23i = Transaction::new();
    bm3.register_balance_manager(&mut tx23i, "m1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23i, 0), "register_balance_manager");

    let mut tx23j = Transaction::new();
    bm3.owner(&mut tx23j, "m1").expect("call should succeed");
    assert_eq!(command_function(&tx23j, 0), "owner");

    let mut tx23k = Transaction::new();
    bm3.id(&mut tx23k, "m1").expect("call should succeed");
    assert_eq!(command_function(&tx23k, 0), "id");

    let mut tx23l = Transaction::new();
    bm3.set_balance_manager_referral(&mut tx23l, "m1", "0x4")
        .expect("call should succeed");
    let last23l = tx23l.data.commands.len() - 1;
    assert_eq!(command_function(&tx23l, last23l), "set_balance_manager_referral");

    let mut tx23m = Transaction::new();
    bm3.unset_balance_manager_referral(&mut tx23m, "m1", "DEEP_SUI")
        .expect("call should succeed");
    let last23m = tx23m.data.commands.len() - 1;
    assert_eq!(
        command_function(&tx23m, last23m),
        "unset_balance_manager_referral"
    );

    let mut tx23n = Transaction::new();
    contract
        .place_limit_order(
            &mut tx23n, "DEEP_SUI", "m1", 1, 0, 0, 1.0, 1.0, true, true, Some(100),
        )
        .expect("call should succeed");
    let last23n = tx23n.data.commands.len() - 1;
    assert_eq!(command_function(&tx23n, last23n), "place_limit_order");

    let mut tx23o = Transaction::new();
    contract
        .place_market_order(&mut tx23o, "DEEP_SUI", "m1", 1, 0, 1.0, true, true)
        .expect("call should succeed");
    let last23o = tx23o.data.commands.len() - 1;
    assert_eq!(command_function(&tx23o, last23o), "place_market_order");

    let mut tx23p = Transaction::new();
    contract
        .modify_order(&mut tx23p, "DEEP_SUI", "m1", 1, 1.0)
        .expect("call should succeed");
    let last23p = tx23p.data.commands.len() - 1;
    assert_eq!(command_function(&tx23p, last23p), "modify_order");

    let mut tx23q = Transaction::new();
    contract
        .cancel_order(&mut tx23q, "DEEP_SUI", "m1", 1)
        .expect("call should succeed");
    let last23q = tx23q.data.commands.len() - 1;
    assert_eq!(command_function(&tx23q, last23q), "cancel_order");

    let mut tx23r = Transaction::new();
    contract
        .cancel_all_orders(&mut tx23r, "DEEP_SUI", "m1")
        .expect("call should succeed");
    let last23r = tx23r.data.commands.len() - 1;
    assert_eq!(command_function(&tx23r, last23r), "cancel_all_orders");

    let mut tx23s = Transaction::new();
    contract
        .withdraw_settled_amounts(&mut tx23s, "DEEP_SUI", "m1")
        .expect("call should succeed");
    let last23s = tx23s.data.commands.len() - 1;
    assert_eq!(command_function(&tx23s, last23s), "withdraw_settled_amounts");

    let mut tx23t = Transaction::new();
    contract
        .withdraw_settled_amounts_permissionless(&mut tx23t, "DEEP_SUI", "m1")
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx23t, 0),
        "withdraw_settled_amounts_permissionless"
    );

    let mut tx23u = Transaction::new();
    contract
        .claim_rebates(&mut tx23u, "DEEP_SUI", "m1")
        .expect("call should succeed");
    let last23u = tx23u.data.commands.len() - 1;
    assert_eq!(command_function(&tx23u, last23u), "claim_rebates");

    let mut tx23u2 = Transaction::new();
    contract
        .mid_price(&mut tx23u2, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23u2, 0), "mid_price");

    let mut tx23u3 = Transaction::new();
    contract
        .whitelisted(&mut tx23u3, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23u3, 0), "whitelisted");

    let mut tx23u4 = Transaction::new();
    contract
        .create_permissionless_pool(&mut tx23u4, "DEEP", "SUI", 1.0, 1.0, 1.0, "0x203")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23u4, 0), "create_permissionless_pool");

    let mut tx23v = Transaction::new();
    contract
        .update_pool_allowed_versions(&mut tx23v, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23v, 0), "update_pool_allowed_versions");

    let mut tx23w = Transaction::new();
    contract
        .add_deep_price_point(&mut tx23w, "DEEP_SUI", "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23w, 0), "add_deep_price_point");

    let mut tx23x = Transaction::new();
    contract
        .mint_referral(&mut tx23x, "DEEP_SUI", 1.25)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23x, 0), "mint_referral");

    let mut tx23y = Transaction::new();
    contract
        .update_pool_referral_multiplier(&mut tx23y, "DEEP_SUI", "0x30", 1.5)
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx23y, 0),
        "update_pool_referral_multiplier"
    );

    let mut tx23z = Transaction::new();
    contract
        .claim_pool_referral_rewards(&mut tx23z, "DEEP_SUI", "0x30")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23z, 0), "claim_pool_referral_rewards");

    let mut tx23aa = Transaction::new();
    contract
        .burn_deep(&mut tx23aa, "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx23aa, 0), "burn_deep");

    let mut tx23ab = Transaction::new();
    contract
        .swap_exact_base_for_quote(&mut tx23ab, "DEEP_SUI", "0x31", "0x32", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23ab, 0), "swap_exact_base_for_quote");

    let mut tx23ac = Transaction::new();
    contract
        .swap_exact_quote_for_base(&mut tx23ac, "DEEP_SUI", "0x33", "0x32", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23ac, 0), "swap_exact_quote_for_base");

    let mut tx23ad = Transaction::new();
    contract
        .swap_exact_quantity(&mut tx23ad, "DEEP_SUI", "0x31", "0x33", "0x32", 1.0, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx23ad, 0), "swap_exact_quantity");

    let mut tx23ae = Transaction::new();
    contract
        .swap_exact_base_for_quote_with_manager(
            &mut tx23ae, "DEEP_SUI", "m1", "0x34", "0x35", "0x36", "0x31", 1.0,
        )
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx23ae, 0),
        "swap_exact_base_for_quote_with_manager"
    );

    let mut tx23af = Transaction::new();
    contract
        .swap_exact_quote_for_base_with_manager(
            &mut tx23af, "DEEP_SUI", "m1", "0x34", "0x35", "0x36", "0x33", 1.0,
        )
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx23af, 0),
        "swap_exact_quote_for_base_with_manager"
    );

    let mut tx23ag = Transaction::new();
    contract
        .swap_exact_quantity_with_manager(
            &mut tx23ag, "DEEP_SUI", "m1", "0x34", "0x35", "0x36", "0x31", "0x33", 1.0, true,
        )
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx23ag, 0),
        "swap_exact_quantity_with_manager"
    );

    let margin_pool = MarginPoolContract { config: &cfg };

    let mut tx24a = Transaction::new();
    margin_pool
        .mint_supplier_cap(&mut tx24a)
        .expect("call should succeed");
    assert_eq!(command_function(&tx24a, 0), "mint_supplier_cap");

    let mut tx24b = Transaction::new();
    margin_pool
        .mint_supply_referral(&mut tx24b, "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx24b, 0), "mint_supply_referral");

    let mut tx24c = Transaction::new();
    margin_pool
        .withdraw_referral_fees(&mut tx24c, "SUI", "0x204")
        .expect("call should succeed");
    assert_eq!(command_function(&tx24c, 0), "withdraw_referral_fees");

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

    let mut tx36b = Transaction::new();
    margin_manager
        .borrow_base(&mut tx36b, "mm1", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx36b, 0), "borrow_base");

    let mut tx36c = Transaction::new();
    margin_manager
        .borrow_quote(&mut tx36c, "mm1", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx36c, 0), "borrow_quote");

    let mut tx36d = Transaction::new();
    margin_manager
        .repay_base(&mut tx36d, "mm1", Some(1.0))
        .expect("call should succeed");
    assert_eq!(command_function(&tx36d, 0), "repay_base");

    let mut tx36e = Transaction::new();
    margin_manager
        .repay_quote(&mut tx36e, "mm1", None)
        .expect("call should succeed");
    assert_eq!(command_function(&tx36e, 0), "repay_quote");

    let mut tx36f = Transaction::new();
    margin_manager
        .set_margin_manager_referral(&mut tx36f, "mm1", "0x205")
        .expect("call should succeed");
    assert_eq!(command_function(&tx36f, 0), "set_margin_manager_referral");

    let mut tx36g = Transaction::new();
    margin_manager
        .unset_margin_manager_referral(&mut tx36g, "mm1", "DEEP_SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx36g, 0), "unset_margin_manager_referral");

    let margin_tpsl = MarginTPSLContract { config: &cfg };

    let mut tx37 = Transaction::new();
    margin_tpsl
        .conditional_order_ids(&mut tx37, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx37, 0), "conditional_order_ids");

    let mut tx37b = Transaction::new();
    margin_tpsl
        .conditional_order(&mut tx37b, "mm1", 1)
        .expect("call should succeed");
    assert_eq!(command_function(&tx37b, 0), "conditional_order");

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

    let mut tx39b = Transaction::new();
    margin_tpsl
        .new_condition(&mut tx39b, "DEEP_SUI", true, 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx39b, 0), "new_condition");

    let mut tx39c = Transaction::new();
    margin_tpsl
        .new_pending_limit_order(&mut tx39c, "DEEP_SUI", 1, 0, 0, 1.0, 1.0, true, true, Some(100))
        .expect("call should succeed");
    assert_eq!(command_function(&tx39c, 0), "new_pending_limit_order");

    let mut tx39d = Transaction::new();
    margin_tpsl
        .new_pending_market_order(&mut tx39d, "DEEP_SUI", 1, 0, 1.0, true, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx39d, 0), "new_pending_market_order");

    let mut tx39e = Transaction::new();
    margin_tpsl
        .add_conditional_limit_order(
            &mut tx39e,
            "mm1",
            1,
            true,
            1.0,
            2,
            0,
            0,
            1.0,
            1.0,
            true,
            true,
            Some(100),
        )
        .expect("call should succeed");
    let last39e = tx39e.data.commands.len() - 1;
    assert_eq!(command_function(&tx39e, last39e), "add_conditional_order");

    let mut tx39f = Transaction::new();
    margin_tpsl
        .add_conditional_market_order(&mut tx39f, "mm1", 1, true, 1.0, 2, 0, 1.0, true, true)
        .expect("call should succeed");
    let last39f = tx39f.data.commands.len() - 1;
    assert_eq!(command_function(&tx39f, last39f), "add_conditional_order");

    let mut tx39g = Transaction::new();
    margin_tpsl
        .cancel_all_conditional_orders(&mut tx39g, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx39g, 0), "cancel_all_conditional_orders");

    let mut tx39h = Transaction::new();
    margin_tpsl
        .cancel_conditional_order(&mut tx39h, "mm1", 1)
        .expect("call should succeed");
    assert_eq!(command_function(&tx39h, 0), "cancel_conditional_order");

    let mut tx39i = Transaction::new();
    margin_tpsl
        .execute_conditional_orders(&mut tx39i, "0x3", "DEEP_SUI", 10)
        .expect("call should succeed");
    assert_eq!(command_function(&tx39i, 0), "execute_conditional_orders");

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

    let pool_proxy = PoolProxyContract { config: &cfg };

    let mut tx52 = Transaction::new();
    pool_proxy
        .place_limit_order(
            &mut tx52, "mm1", 1, 0, 0, 1.0, 1.0, true, true, 100,
        )
        .expect("call should succeed");
    assert_eq!(command_function(&tx52, 0), "place_limit_order");

    let mut tx53 = Transaction::new();
    pool_proxy
        .place_market_order(&mut tx53, "mm1", 1, 0, 1.0, true, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx53, 0), "place_market_order");

    let mut tx53b = Transaction::new();
    pool_proxy
        .place_reduce_only_limit_order(&mut tx53b, "mm1", 1, 0, 0, 1.0, 1.0, true, true, 100)
        .expect("call should succeed");
    assert_eq!(command_function(&tx53b, 0), "place_reduce_only_limit_order");

    let mut tx53c = Transaction::new();
    pool_proxy
        .place_reduce_only_market_order(&mut tx53c, "mm1", 1, 0, 1.0, true, true)
        .expect("call should succeed");
    assert_eq!(command_function(&tx53c, 0), "place_reduce_only_market_order");

    let mut tx54 = Transaction::new();
    pool_proxy
        .modify_order(&mut tx54, "mm1", 1, 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx54, 0), "modify_order");

    let mut tx55 = Transaction::new();
    pool_proxy
        .cancel_order(&mut tx55, "mm1", 1)
        .expect("call should succeed");
    assert_eq!(command_function(&tx55, 0), "cancel_order");

    let mut tx56 = Transaction::new();
    pool_proxy
        .cancel_orders(&mut tx56, "mm1", &[1, 2])
        .expect("call should succeed");
    assert_eq!(command_function(&tx56, 0), "cancel_orders");

    let mut tx57 = Transaction::new();
    pool_proxy
        .cancel_all_orders(&mut tx57, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx57, 0), "cancel_all_orders");

    let mut tx58 = Transaction::new();
    pool_proxy
        .withdraw_settled_amounts(&mut tx58, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx58, 0), "withdraw_settled_amounts");

    let mut tx59 = Transaction::new();
    pool_proxy
        .stake(&mut tx59, "mm1", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx59, 0), "stake");

    let mut tx60 = Transaction::new();
    pool_proxy
        .unstake(&mut tx60, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx60, 0), "unstake");

    let mut tx61 = Transaction::new();
    pool_proxy
        .submit_proposal(&mut tx61, "mm1", 0.1, 0.01, 10.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx61, 0), "submit_proposal");

    let mut tx62 = Transaction::new();
    pool_proxy
        .vote(&mut tx62, "mm1", "0x4")
        .expect("call should succeed");
    assert_eq!(command_function(&tx62, 0), "vote");

    let mut tx63 = Transaction::new();
    pool_proxy
        .claim_rebate(&mut tx63, "mm1")
        .expect("call should succeed");
    assert_eq!(command_function(&tx63, 0), "claim_rebate");

    let mut tx64 = Transaction::new();
    pool_proxy
        .withdraw_margin_settled_amounts(&mut tx64, "DEEP_SUI", "0x3")
        .expect("call should succeed");
    assert_eq!(command_function(&tx64, 0), "withdraw_settled_amounts_permissionless");

    let liquidations = MarginLiquidationsContract { config: &cfg };

    let mut tx65 = Transaction::new();
    liquidations
        .create_liquidation_vault(&mut tx65, "0x9")
        .expect("call should succeed");
    assert_eq!(command_function(&tx65, 0), "create_liquidation_vault");

    let mut tx66 = Transaction::new();
    liquidations
        .deposit(&mut tx66, "0x10", "0x9", "SUI", "0x11")
        .expect("call should succeed");
    assert_eq!(command_function(&tx66, 0), "deposit");

    let mut tx67 = Transaction::new();
    liquidations
        .withdraw(&mut tx67, "0x10", "0x9", "SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx67, 0), "withdraw");

    let mut tx68 = Transaction::new();
    liquidations
        .liquidate_base(&mut tx68, "0x10", "0x3", "DEEP_SUI", Some(1.0))
        .expect("call should succeed");
    assert_eq!(command_function(&tx68, 0), "liquidate_base");

    let mut tx69 = Transaction::new();
    liquidations
        .liquidate_quote(&mut tx69, "0x10", "0x3", "DEEP_SUI", None)
        .expect("call should succeed");
    assert_eq!(command_function(&tx69, 0), "liquidate_quote");

    let mut tx70 = Transaction::new();
    liquidations
        .balance(&mut tx70, "0x10", "SUI")
        .expect("call should succeed");
    assert_eq!(command_function(&tx70, 0), "balance");

    let flash = FlashLoanContract { config: &cfg };

    let mut tx71 = Transaction::new();
    flash
        .borrow_base_asset(&mut tx71, "DEEP_SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx71, 0), "borrow_flashloan_base");

    let mut tx72 = Transaction::new();
    flash
        .return_base_asset(&mut tx72, "DEEP_SUI", "0x12", "0x13")
        .expect("call should succeed");
    assert_eq!(command_function(&tx72, 0), "return_flashloan_base");

    let mut tx73 = Transaction::new();
    flash
        .borrow_quote_asset(&mut tx73, "DEEP_SUI", 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx73, 0), "borrow_flashloan_quote");

    let mut tx74 = Transaction::new();
    flash
        .return_quote_asset(&mut tx74, "DEEP_SUI", "0x12", "0x13")
        .expect("call should succeed");
    assert_eq!(command_function(&tx74, 0), "return_flashloan_quote");

    let maintainer = MarginMaintainerContract { config: &cfg };

    let mut tx75 = Transaction::new();
    maintainer
        .new_interest_config(&mut tx75, 0.1, 0.2, 0.8, 0.5)
        .expect("call should succeed");
    assert_eq!(command_function(&tx75, 0), "new_interest_config");

    let mut tx76 = Transaction::new();
    maintainer
        .new_margin_pool_config(&mut tx76, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    assert_eq!(command_function(&tx76, 0), "new_margin_pool_config");

    let mut tx77 = Transaction::new();
    maintainer
        .new_margin_pool_config_with_rate_limit(&mut tx77, "SUI", 10.0, 0.9, 0.1, 1.0, 5.0, 0.1, true)
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx77, 0),
        "new_margin_pool_config_with_rate_limit"
    );

    let mut tx78 = Transaction::new();
    let mp = maintainer
        .new_margin_pool_config(&mut tx78, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    let ic = maintainer
        .new_interest_config(&mut tx78, 0.1, 0.2, 0.8, 0.5)
        .expect("call should succeed");
    maintainer
        .new_protocol_config(&mut tx78, mp, ic)
        .expect("call should succeed");
    assert_eq!(command_function(&tx78, 2), "new_protocol_config");

    let mut tx79 = Transaction::new();
    let mp2 = maintainer
        .new_margin_pool_config(&mut tx79, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    let ic2 = maintainer
        .new_interest_config(&mut tx79, 0.1, 0.2, 0.8, 0.5)
        .expect("call should succeed");
    let pc = maintainer
        .new_protocol_config(&mut tx79, mp2, ic2)
        .expect("call should succeed");
    maintainer
        .create_margin_pool(&mut tx79, "SUI", pc, "0x14")
        .expect("call should succeed");
    assert_eq!(command_function(&tx79, 3), "create_margin_pool");

    let mut tx80 = Transaction::new();
    maintainer
        .enable_deepbook_pool_for_loan(&mut tx80, "DEEP_SUI", "SUI", "0x15")
        .expect("call should succeed");
    assert_eq!(command_function(&tx80, 0), "enable_deepbook_pool_for_loan");

    let mut tx81 = Transaction::new();
    maintainer
        .disable_deepbook_pool_for_loan(&mut tx81, "DEEP_SUI", "SUI", "0x15")
        .expect("call should succeed");
    assert_eq!(command_function(&tx81, 0), "disable_deepbook_pool_for_loan");

    let mut tx82 = Transaction::new();
    let ic3 = maintainer
        .new_interest_config(&mut tx82, 0.1, 0.2, 0.8, 0.5)
        .expect("call should succeed");
    maintainer
        .update_interest_params(&mut tx82, "SUI", "0x15", ic3)
        .expect("call should succeed");
    assert_eq!(command_function(&tx82, 1), "update_interest_params");

    let mut tx83 = Transaction::new();
    let mpc = maintainer
        .new_margin_pool_config(&mut tx83, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    maintainer
        .update_margin_pool_config(&mut tx83, "SUI", "0x15", mpc)
        .expect("call should succeed");
    assert_eq!(command_function(&tx83, 1), "update_margin_pool_config");

    let deepbook_admin = DeepBookAdminContract { config: &cfg };

    let mut tx84 = Transaction::new();
    deepbook_admin
        .create_pool_admin(&mut tx84, "DEEP", "SUI", 1.0, 1.0, 1.0, true, false, "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx84, 0), "create_pool_admin");

    let mut tx85 = Transaction::new();
    deepbook_admin
        .unregister_pool_admin(&mut tx85, "DEEP_SUI", "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx85, 0), "unregister_pool_admin");

    let mut tx86 = Transaction::new();
    deepbook_admin
        .update_allowed_versions(&mut tx86, "DEEP_SUI", "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx86, 0), "update_allowed_versions");

    let mut tx87 = Transaction::new();
    deepbook_admin
        .enable_version(&mut tx87, 1, "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx87, 0), "enable_version");

    let mut tx88 = Transaction::new();
    deepbook_admin
        .disable_version(&mut tx88, 1, "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx88, 0), "disable_version");

    let mut tx89 = Transaction::new();
    deepbook_admin
        .set_treasury_address(&mut tx89, "0xb", "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx89, 0), "set_treasury_address");

    let mut tx90 = Transaction::new();
    deepbook_admin
        .add_stable_coin(&mut tx90, "SUI", "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx90, 0), "add_stablecoin");

    let mut tx91 = Transaction::new();
    deepbook_admin
        .remove_stable_coin(&mut tx91, "SUI", "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx91, 0), "remove_stablecoin");

    let mut tx92 = Transaction::new();
    deepbook_admin
        .adjust_tick_size(&mut tx92, "DEEP_SUI", 1.0, "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx92, 0), "adjust_tick_size_admin");

    let mut tx93 = Transaction::new();
    deepbook_admin
        .adjust_min_lot_size(&mut tx93, "DEEP_SUI", 1.0, 1.0, "0xa")
        .expect("call should succeed");
    assert_eq!(command_function(&tx93, 0), "adjust_min_lot_size_admin");

    let margin_admin = MarginAdminContract { config: &cfg };

    let mut tx94 = Transaction::new();
    margin_admin
        .mint_maintainer_cap(&mut tx94, "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx94, 0), "mint_maintainer_cap");

    let mut tx95 = Transaction::new();
    margin_admin
        .revoke_maintainer_cap(&mut tx95, "0xc", "0xd")
        .expect("call should succeed");
    assert_eq!(command_function(&tx95, 0), "revoke_maintainer_cap");

    let mut tx96 = Transaction::new();
    let mpc2 = maintainer
        .new_margin_pool_config(&mut tx96, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    margin_admin
        .register_deepbook_pool(&mut tx96, "DEEP_SUI", "0xc", mpc2)
        .expect("call should succeed");
    assert_eq!(command_function(&tx96, 1), "register_deepbook_pool");

    let mut tx97 = Transaction::new();
    margin_admin
        .enable_deepbook_pool(&mut tx97, "DEEP_SUI", "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx97, 0), "enable_deepbook_pool");

    let mut tx98 = Transaction::new();
    margin_admin
        .disable_deepbook_pool(&mut tx98, "DEEP_SUI", "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx98, 0), "disable_deepbook_pool");

    let mut tx98b = Transaction::new();
    let mpc2b = maintainer
        .new_margin_pool_config(&mut tx98b, "SUI", 10.0, 0.9, 0.1, 1.0)
        .expect("call should succeed");
    margin_admin
        .update_risk_params(&mut tx98b, "DEEP_SUI", "0xc", mpc2b)
        .expect("call should succeed");
    assert_eq!(command_function(&tx98b, 1), "update_risk_params");

    let mut tx99 = Transaction::new();
    margin_admin
        .enable_version(&mut tx99, 1, "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx99, 0), "enable_version");

    let mut tx100 = Transaction::new();
    margin_admin
        .disable_version(&mut tx100, 1, "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx100, 0), "disable_version");

    let mut tx101 = Transaction::new();
    margin_admin
        .mint_pause_cap(&mut tx101, "0xc")
        .expect("call should succeed");
    assert_eq!(command_function(&tx101, 0), "mint_pause_cap");

    let mut tx102 = Transaction::new();
    margin_admin
        .revoke_pause_cap(&mut tx102, "0xc", "0xe")
        .expect("call should succeed");
    assert_eq!(command_function(&tx102, 0), "revoke_pause_cap");

    let mut tx103 = Transaction::new();
    margin_admin
        .disable_version_pause_cap(&mut tx103, 1, "0xe")
        .expect("call should succeed");
    assert_eq!(command_function(&tx103, 0), "disable_version_pause_cap");

    let mut tx103b = Transaction::new();
    margin_admin
        .admin_withdraw_default_referral_fees(&mut tx103b, "SUI", "0xc")
        .expect("call should succeed");
    assert_eq!(
        command_function(&tx103b, 0),
        "admin_withdraw_default_referral_fees"
    );
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
