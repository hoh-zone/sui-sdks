import '../../sui/transactions.dart';
import '../config.dart';
import '../types.dart';
import 'encode.dart';

class BalanceManagerContract {
  const BalanceManagerContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> generateProof(Transaction tx, String managerKey) {
    final manager = config.getBalanceManager(managerKey);
    if (manager.tradeCap.isNotEmpty) {
      return tx.moveCall(
        '${config.packageIds.deepbookPackageId}::balance_manager::generate_proof_as_trader',
        [tx.object(manager.address), tx.object(manager.tradeCap)],
        const <String>[],
      );
}

class MarginLiquidationsContract {
  const MarginLiquidationsContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> createLiquidationVault(Transaction tx, String liquidationAdminCap) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::create_liquidation_vault',
      [tx.object(liquidationAdminCap)],
      const [],
    );
  }

  Map<String, dynamic> balance(Transaction tx, String vaultId, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::balance',
      [tx.object(vaultId)],
      [config.getCoin(coinKey).type],
    );
  }

  Map<String, dynamic> liquidateBase(Transaction tx, String vaultId, String managerAddress, String poolKey, double? repayAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    final repayAmountArg = repayAmount != null ? [1, ...encodeU64((repayAmount * base.scalar).round())] : [0];

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::liquidate_base',
      [
        tx.object(vaultId),
        tx.object(managerAddress),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(pool.address),
        tx.pure(repayAmountArg),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> liquidateQuote(Transaction tx, String vaultId, String managerAddress, String poolKey, double? repayAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    final repayAmountArg = repayAmount != null ? [1, ...encodeU64((repayAmount * quote.scalar).round())] : [0];

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::liquidate_quote',
      [
        tx.object(vaultId),
        tx.object(managerAddress),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(pool.address),
        tx.pure(repayAmountArg),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }
}

class MarginMaintainerContract {
  const MarginMaintainerContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> createMarginPoolConfig(
      Transaction tx,
      String coinKey,
      double supplyCap,
      double maxUtilizationRate,
      double referralSpread,
      double minBorrow) {
    final coin = config.getCoin(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::protocol_config::new_margin_pool_config',
      [
        tx.pure(encodeU64((supplyCap * coin.scalar).round())),
        tx.pure(encodeU64((maxUtilizationRate * floatScalar).round())),
        tx.pure(encodeU64((referralSpread * floatScalar).round())),
        tx.pure(encodeU64((minBorrow * coin.scalar).round())),
      ],
      const [],
    );
  }

  Map<String, dynamic> createInterestConfig(Transaction tx, double baseRate, double baseSlope, double optimalUtilization, double excessSlope) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::protocol_config::new_interest_config',
      [
        tx.pure(encodeU64((baseRate * floatScalar).round())),
        tx.pure(encodeU64((baseSlope * floatScalar).round())),
        tx.pure(encodeU64((optimalUtilization * floatScalar).round())),
        tx.pure(encodeU64((excessSlope * floatScalar).round())),
      ],
      const [],
    );
  }

  Map<String, dynamic> createMarginPool(Transaction tx, String coinKey, dynamic poolConfig, String marginMaintainerCap) {
    final coin = config.getCoin(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::create_margin_pool',
      [
        tx.object(config.packageIds.marginRegistryId),
        poolConfig,
        tx.object(marginMaintainerCap),
        tx.object('0x6'),
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> updateInterestParams(Transaction tx, String coinKey, String marginPoolCap, dynamic interestConfig) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::update_interest_params',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        interestConfig,
        tx.object(marginPoolCap),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> updateMarginPoolConfig(Transaction tx, String coinKey, String marginPoolCap, dynamic marginPoolConfig) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::update_margin_pool_config',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        marginPoolConfig,
        tx.object(marginPoolCap),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }
}

class MarginRegistryContract {
  const MarginRegistryContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> poolEnabled(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::pool_enabled',
      [tx.object(config.packageIds.marginRegistryId), tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getMarginPoolId(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::get_margin_pool_id',
      [tx.object(config.packageIds.marginRegistryId)],
      [config.getCoin(coinKey).type],
    );
  }

  Map<String, dynamic> getMarginManagerIds(Transaction tx, String owner) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::get_margin_manager_ids',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(_encodeAddress(owner))
      ],
      const [],
    );
  }

  Map<String, dynamic> liquidationRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::liquidation_risk_ratio',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> minBorrowRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::min_borrow_risk_ratio',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> minWithdrawRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::min_withdraw_risk_ratio',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> targetLiquidationRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::target_liquidation_risk_ratio',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> userLiquidationReward(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::user_liquidation_reward',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> poolLiquidationReward(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::pool_liquidation_reward',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> allowedMaintainers(Transaction tx) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::allowed_maintainers',
      [tx.object(config.packageIds.marginRegistryId)],
      const [],
    );
  }

  Map<String, dynamic> allowedPauseCaps(Transaction tx) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::allowed_pause_caps',
      [tx.object(config.packageIds.marginRegistryId)],
      const [],
    );
  }

  Map<String, dynamic> baseMarginPoolId(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::base_margin_pool_id',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> quoteMarginPoolId(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::quote_margin_pool_id',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> getDeepbookPoolMarginPoolIds(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::get_deepbook_pool_margin_pool_ids',
      [tx.object(config.packageIds.marginRegistryId), tx.pure(_encodeId(pool.address))],
      const [],
    );
  }

  Map<String, dynamic> getMarginManagerById(Transaction tx, String marginManagerId) {
    final pool = config.getPool(config.getMarginManager(marginManagerKey).poolKey;
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::id',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> baseMarginPoolById(Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::base_margin_pool_id',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> quoteMarginPoolById(Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::quote_margin_pool_id',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> marginPoolId(Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::margin_pool_id',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }
}

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::generate_proof_as_owner',
      [tx.object(manager.address)],
      const <String>[],
    );
  }
}

class DeepBookContract {
  const DeepBookContract(this.config, this.balanceManager);

  final DeepBookConfig config;
  final BalanceManagerContract balanceManager;

  Map<String, dynamic> placeLimitOrder(
      Transaction tx, PlaceLimitOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price =
        ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();
    final expiration =
        params.expiration == 0 ? maxTimestamp : params.expiration;
    final proof = balanceManager.generateProof(tx, params.balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::place_limit_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.orderType.index]),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(price)),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.pure(encodeU64(expiration)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> placeMarketOrder(
      Transaction tx, PlaceMarketOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (params.quantity * base.scalar).round();
    final proof = balanceManager.generateProof(tx, params.balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::place_market_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> cancelOrder(Transaction tx, String poolKey,
      String balanceManagerKey, String orderId) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::cancel_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU128(orderId)),
        tx.object('0x6')
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> cancelOrders(Transaction tx, String poolKey,
      String balanceManagerKey, List<String> orderIds) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::cancel_orders',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeVecU128(orderIds)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuoteQuantityOut(
      Transaction tx, String poolKey, double baseQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (baseQuantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_out',
      [tx.object(pool.address), tx.pure(encodeU64(quantity)), tx.object('0x6')],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getBaseQuantityOut(
      Transaction tx, String poolKey, double quoteQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (quoteQuantity * quote.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_base_quantity_out',
      [tx.object(pool.address), tx.pure(encodeU64(quantity)), tx.object('0x6')],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuantityOut(Transaction tx, String poolKey,
      double baseQuantity, double quoteQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final bq = (baseQuantity * base.scalar).round();
    final qq = (quoteQuantity * quote.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quantity_out',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(bq)),
        tx.pure(encodeU64(qq)),
        tx.object('0x6')
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> midPrice(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::mid_price',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getOrder(
      Transaction tx, String poolKey, String orderId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_order',
      [tx.object(pool.address), tx.pure(encodeU128(orderId))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> canPlaceLimitOrder(
      Transaction tx, CanPlaceLimitOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price =
        ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::can_place_limit_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.pure(encodeU64(price)),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.pure(encodeU64(params.expireTimestamp)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }
}

class GovernanceContract {
  const GovernanceContract(this.config, this.balanceManager);

  final DeepBookConfig config;
  final BalanceManagerContract balanceManager;

  Map<String, dynamic> vote(Transaction tx, String poolKey,
      String balanceManagerKey, String proposalId) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::vote',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU128(proposalId))
      ],
      [base.type, quote.type],
    );
  }
}

class FlashLoanContract {
  const FlashLoanContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> borrowBaseAsset(
      Transaction tx, String poolKey, double borrowAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final amount = (borrowAmount * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::borrow_flashloan_base',
      [tx.object(pool.address), tx.pure(encodeU64(amount))],
      [base.type, quote.type],
    );
  }
}

class MarginManagerContract {
  const MarginManagerContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> getMarginAccountOrderDetails(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final bm = tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::balance_manager',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId)
      ],
      [base.type, quote.type],
    );

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_account_order_details',
      [tx.object(pool.address), bm],
      [base.type, quote.type],
    );
  }
}

class PoolProxyContract {
  const PoolProxyContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> placeLimitOrder(
      Transaction tx, PlaceMarginLimitOrderParams params) {
    final manager = config.getMarginManager(params.marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price =
        ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();
    final expiration =
        params.expiration == 0 ? maxTimestamp : params.expiration;

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::pool_proxy::place_limit_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.orderType.index]),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(price)),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.pure(encodeU64(expiration)),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> placeMarketOrder(
      Transaction tx, PlaceMarginMarketOrderParams params) {
    final manager = config.getMarginManager(params.marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (params.quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::pool_proxy::place_market_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }
}

class MarginTPSLContract {
  const MarginTPSLContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> cancelAllConditionalOrders(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::cancel_all_conditional_orders',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId)
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> conditionalOrder(
      Transaction tx, String marginManagerKey, String orderId) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::conditional_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU128(orderId)),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> conditionalOrderIds(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::conditional_order_ids',
      [tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> highestTriggerBelowPrice(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::highest_trigger_below_price',
      [tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> lowestTriggerAbovePrice(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::lowest_trigger_above_price',
      [tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> conditionalOrderFilled(
      Transaction tx, String marginManagerKey, String orderId) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::conditional_order_filled',
      [tx.object(manager.address), tx.pure(encodeU128(orderId))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> placeReduceOnlyLimitOrder(
      Transaction tx, PlaceMarginLimitOrderParams params) {
    final manager = config.getMarginManager(params.marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price =
        ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();
    final expiration =
        params.expiration == 0 ? maxTimestamp : params.expiration;

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::place_reduce_only_limit_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.orderType.index]),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(price)),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.pure(encodeU64(expiration)),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> placeReduceOnlyMarketOrder(
      Transaction tx, PlaceMarginMarketOrderParams params) {
    final manager = config.getMarginManager(params.marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (params.quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::place_reduce_only_market_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU64(int.parse(params.clientOrderId))),
        tx.pure([params.selfMatchingOption.index]),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getMarginAccountOrderDetailsFromPool(
      Transaction tx, String poolKey, String managerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final bm = tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_account_order_details',
      [tx.object(pool.address), tx.pure(_encodeId(managerId))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> balanceManagerFromMarginManager(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::balance_manager',
      [tx.pure(_encodeId(marginManagerId))],
      [base.type, quote.type],
    );
  }
}

class SwapParams {
  const SwapParams(this.poolKey, this.amount, this.minOut, this.deepAmount);

  final String poolKey;
  final double amount;
  final double minOut;
  final double deepAmount;

  SwapParams(
      {required this.poolKey,
      required this.amount,
      this.minOut = 0.0,
      this.deepAmount = 0.0,
    });
}

class SwapWithManagerParams {
  const SwapWithManagerParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.tradeCap,
    required this.depositCap,
    required this.withdrawCap,
    required this.amount,
    required this.minOut,
  });
}

class SwapMethods {
  const SwapMethods(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> swapExactBaseForQuote(
      Transaction tx, SwapParams params) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final minQuoteInput = (params.minOut * quote.scalar).round();
    final baseAmountInput = (params.amount * base.scalar).round();
    final deepAmountInput = (params.deepAmount * deepScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_base_for_quote',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(baseAmountInput)),
        tx.pure(encodeU64(deepAmountInput)),
        tx.pure(encodeU64(minQuoteInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }

  Map<String, dynamic> swapExactQuoteForBase(
      Transaction tx, SwapParams params) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final minBaseInput = (params.minOut * base.scalar).round();
    final quoteAmountInput = (params.amount * quote.scalar).round();
    final deepAmountInput = (params.deepAmount * deepScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quote_for_base',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(quoteAmountInput)),
        tx.pure(encodeU64(deepAmountInput)),
        tx.pure(encodeU64(minBaseInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }

  Map<String, dynamic> swapExactQuantity(
      Transaction tx, SwapParams params, bool isBaseToCoin) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final minOutInput =
        (params.minOut * (isBaseToCoin ? quote.scalar : base.scalar)).round();
    final baseAmountInput =
        isBaseToCoin ? (params.amount * base.scalar).round() : 0;
    final quoteAmountInput =
        isBaseToCoin ? 0 : (params.amount * quote.scalar).round();
    final deepAmountInput = (params.deepAmount * deepScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quantity',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(baseAmountInput)),
        tx.pure(encodeU64(quoteAmountInput)),
        tx.pure(encodeU64(deepAmountInput)),
        tx.pure(encodeU64(minOutInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }

  Map<String, dynamic> swapExactBaseForQuoteWithManager(
      Transaction tx, SwapWithManagerParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final baseCoin = config.getCoin(pool.baseCoin);
    final quoteCoin = config.getCoin(pool.quoteCoin);
    final minQuoteInput = (params.minOut * quoteCoin.scalar).round();
    final baseAmountInput = (params.amount * baseCoin.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_base_for_quote_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64(baseAmountInput)),
        tx.pure(encodeU64(minQuoteInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }

  Map<String, dynamic> swapExactQuoteForBaseWithManager(
      Transaction tx, SwapWithManagerParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final baseCoin = config.getCoin(pool.baseCoin);
    final quoteCoin = config.getCoin(pool.quoteCoin);
    final minBaseInput = (params.minOut * baseCoin.scalar).round();
    final quoteAmountInput = (params.amount * quoteCoin.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quote_for_base_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64(quoteAmountInput)),
        tx.pure(encodeU64(minBaseInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }

  Map<String, dynamic> swapExactQuantityWithManager(
      Transaction tx, SwapWithManagerParams params, bool isBaseToCoin) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final baseCoin = config.getCoin(pool.baseCoin);
    final quoteCoin = config.getCoin(pool.quoteCoin);
    final minOutInput =
        (params.minOut * (isBaseToCoin ? quoteCoin.scalar : baseCoin.scalar))
            .round();
    final baseAmountInput =
        isBaseToCoin ? (params.amount * baseCoin.scalar).round() : 0;
    final quoteAmountInput =
        isBaseToCoin ? 0 : (params.amount * quoteCoin.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quantity_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64(baseAmountInput)),
        tx.pure(encodeU64(quoteAmountInput)),
        tx.pure(encodeU64(minOutInput)),
        tx.object('0x6'),
      ],
      [baseCoin.type, quoteCoin.type],
    );
  }
}

class MarginPoolContract {
  const MarginPoolContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> supplyToMarginPool(Transaction tx, String coinKey,
      String supplierCap, double amountToDeposit, String? referralId) {
    final marginPool = config.getMarginPool(coinKey);
    final coin = config.getCoin(coinKey);
    final depositInput = (amountToDeposit * coin.scalar).round();

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::supply',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(supplierCap),
        tx.pure(encodeU64(depositInput)),
        referralId != null
            ? tx.pure([1, ...encodeU128(referralId)])
            : tx.pure([0]),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> withdrawFromMarginPool(Transaction tx, String coinKey,
      String supplierCap, double? amountToWithdraw) {
    final marginPool = config.getMarginPool(coinKey);
    final coin = config.getCoin(coinKey);
    final withdrawInput = amountToWithdraw != null
        ? (amountToWithdraw * coin.scalar).round()
        : null;

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::withdraw',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(supplierCap),
        withdrawInput != null
            ? tx.pure([1, ...encodeU64(withdrawInput)])
            : tx.pure([0]),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> mintSupplierCap(Transaction tx) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::mint_supplier_cap',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      const [],
    );
  }

  Map<String, dynamic> mintSupplyReferral(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::mint_supply_referral',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> withdrawReferralFees(
      Transaction tx, String coinKey, String referralId) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::withdraw_referral_fees',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(referralId),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> totalSupply(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::total_supply',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> totalBorrow(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::total_borrow',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> borrowShares(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::borrow_shares',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> supplyShares(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::supply_shares',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> supplyCap(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::supply_cap',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> minBorrow(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::min_borrow',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> interestRate(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::interest_rate',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> maxUtilizationRate(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::max_utilization_rate',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> userSupplyAmount(
      Transaction tx, String coinKey, String supplierCapId) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::user_supply_amount',
      [
        tx.object(marginPool.address),
        tx.pure(_encodeId(supplierCapId)),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> lastUpdateTimestamp(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::last_update_timestamp',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> protocolSpread(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::protocol_spread',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> deepbookPoolAllowed(
      Transaction tx, String coinKey, String deepbookPoolId) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::deepbook_pool_allowed',
      [tx.object(marginPool.address), tx.pure(_encodeId(deepbookPoolId))],
      [marginPool.type],
    );
  }

  Map<String, dynamic> getId(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::id',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }
}
