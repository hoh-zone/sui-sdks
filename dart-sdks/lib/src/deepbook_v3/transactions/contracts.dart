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

  Map<String, dynamic> placeLimitOrder(Transaction tx, PlaceLimitOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price = ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();
    final expiration = params.expiration == 0 ? maxTimestamp : params.expiration;
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

  Map<String, dynamic> placeMarketOrder(Transaction tx, PlaceMarketOrderParams params) {
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

  Map<String, dynamic> cancelOrder(Transaction tx, String poolKey, String balanceManagerKey, String orderId) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::cancel_order',
      [tx.object(pool.address), tx.object(manager.address), proof, tx.pure(encodeU128(orderId)), tx.object('0x6')],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> cancelOrders(Transaction tx, String poolKey, String balanceManagerKey, List<String> orderIds) {
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

  Map<String, dynamic> getQuoteQuantityOut(Transaction tx, String poolKey, double baseQuantity) {
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

  Map<String, dynamic> getBaseQuantityOut(Transaction tx, String poolKey, double quoteQuantity) {
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

  Map<String, dynamic> getQuantityOut(Transaction tx, String poolKey, double baseQuantity, double quoteQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final bq = (baseQuantity * base.scalar).round();
    final qq = (quoteQuantity * quote.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quantity_out',
      [tx.object(pool.address), tx.pure(encodeU64(bq)), tx.pure(encodeU64(qq)), tx.object('0x6')],
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

  Map<String, dynamic> getOrder(Transaction tx, String poolKey, String orderId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_order',
      [tx.object(pool.address), tx.pure(encodeU128(orderId))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> canPlaceLimitOrder(Transaction tx, CanPlaceLimitOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price = ((params.price * floatScalar * quote.scalar) / base.scalar).round();
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

  Map<String, dynamic> vote(Transaction tx, String poolKey, String balanceManagerKey, String proposalId) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::vote',
      [tx.object(pool.address), tx.object(manager.address), proof, tx.pure(encodeU128(proposalId))],
      [base.type, quote.type],
    );
  }
}

class FlashLoanContract {
  const FlashLoanContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> borrowBaseAsset(Transaction tx, String poolKey, double borrowAmount) {
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

  Map<String, dynamic> getMarginAccountOrderDetails(Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final bm = tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::balance_manager',
      [tx.object(manager.address), tx.object(config.packageIds.marginRegistryId)],
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

  Map<String, dynamic> placeLimitOrder(Transaction tx, PlaceMarginLimitOrderParams params) {
    final manager = config.getMarginManager(params.marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final price = ((params.price * floatScalar * quote.scalar) / base.scalar).round();
    final quantity = (params.quantity * base.scalar).round();
    final expiration = params.expiration == 0 ? maxTimestamp : params.expiration;

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

  Map<String, dynamic> placeMarketOrder(Transaction tx, PlaceMarginMarketOrderParams params) {
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

  Map<String, dynamic> cancelAllConditionalOrders(Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::cancel_all_conditional_orders',
      [tx.object(manager.address), tx.object(config.packageIds.marginRegistryId)],
      [base.type, quote.type],
    );
  }
}
