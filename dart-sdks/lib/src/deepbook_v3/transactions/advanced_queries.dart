import '../sui/transactions.dart';
import 'config.dart';

/// 高级查询方法 for Dart SDK (P2)
class AdvancedQueriesContract {
  AdvancedQueriesContract(this.config, this.balanceManager);

  final DeepBookConfig config;
  final BalanceManagerContract balanceManager;

  /// Get account details by manager key
  Map<String, dynamic> getAccount(
    Transaction tx,
    String poolKey,
    String managerKey,
  ) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account',
      [
        tx.object(pool.address),
        balanceManager.generateProof(tx, managerKey),
      ],
      [base.type, quote.type],
    );
  }

  /// Get locked balance for an account
  Map<String, dynamic> getLockedBalance(
    Transaction tx,
    String poolKey,
    String managerKey,
  ) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::locked_balance',
      [
        tx.object(pool.address),
        balanceManager.generateProof(tx, managerKey),
      ],
      [base.type, quote.type],
    );
  }

  /// Get account order details
  Map<String, dynamic> getAccountOrderDetails(
    Transaction tx,
    String poolKey,
    String managerKey,
  ) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_account_order_details',
      [
        tx.object(pool.address),
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Check if account exists
  Map<String, dynamic> accountExists(
    Transaction tx,
    String poolKey,
    String managerKey,
  ) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account_exists',
      [
        tx.object(pool.address),
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get pool trade params (next)
  Map<String, dynamic> getPoolTradeParamsNext(
    Transaction tx,
    String poolKey,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_trade_params_next',
      [
        tx.object(pool.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get pool book params
  Map<String, dynamic> getPoolBookParams(
    Transaction tx,
    String poolKey,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_book_params',
      [
        tx.object(pool.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get quorum for voting
  Map<String, dynamic> getQuorum(
    Transaction tx,
    String poolKey,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::quorum',
      [
        tx.object(pool.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get pool ID by assets
  Map<String, dynamic> getPoolIdByAssets(
    Transaction tx,
    String baseType,
    String quoteType,
  ) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_pool_id_by_assets',
      [],
      [baseType, quoteType],
    );
  }

  /// Get balance manager IDs for an owner
  Map<String, dynamic> getBalanceManagerIds(
    Transaction tx,
    String owner,
    String registryId,
  ) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::registry::get_balance_manager_ids',
      [
        tx.pureAddress(owner),
        tx.object(registryId),
      ],
      [],
    );
  }

  /// Get all open orders for an account
  Map<String, dynamic> getAllOpenOrders(
    Transaction tx,
    String poolKey,
    String managerKey,
  ) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_all_open_orders',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        balanceManager.generateProof(tx, managerKey),
      ],
      [base.type, quote.type],
    );
  }

  /// Get referral balances for an account
  Map<String, dynamic> getPoolReferralBalances(
    Transaction tx,
    String poolKey,
    String referral,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_pool_referral_balances',
      [
        tx.object(pool.address),
        tx.object(referral),
      ],
      [base.type, quote.type],
    );
  }

  /// Get referral multiplier
  Map<String, dynamic> getReferralMultiplier(
    Transaction tx,
    String poolKey,
    String referral,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_referral_multiplier',
      [
        tx.object(pool.address),
        tx.object(referral),
      ],
      [base.type, quote.type],
    );
  }
}
