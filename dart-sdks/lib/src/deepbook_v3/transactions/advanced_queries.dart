import '../../sui/transactions.dart';
import '../config.dart';
import 'contracts.dart';

class AdvancedQueriesContract {
  const AdvancedQueriesContract(this.config, this.balanceManager);

  final DeepBookConfig config;
  final BalanceManagerContract balanceManager;

  Map<String, dynamic> getAccount(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account',
      [tx.object(pool.address), tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getLockedBalance(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::locked_balance',
      [tx.object(pool.address), tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> accountExists(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account_exists',
      [tx.object(pool.address), tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getPoolTradeParamsNext(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_trade_params_next',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getPoolBookParams(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_book_params',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuorum(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::quorum',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getPoolIdByAssets(
      Transaction tx, String baseType, String quoteType) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_pool_id_by_asset',
      [tx.object(config.packageIds.registryId)],
      [baseType, quoteType],
    );
  }

  Map<String, dynamic> getBalanceManagerIds(Transaction tx, String owner) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::registry::get_balance_manager_ids',
      [tx.object(config.packageIds.registryId), tx.pure(_encodeAddress(owner))],
      const [],
    );
  }

  Map<String, dynamic> getPoolReferralBalances(
      Transaction tx, String poolKey, String referral) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_pool_referral_balances',
      [tx.object(pool.address), tx.object(referral)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getReferralMultiplier(
      Transaction tx, String poolKey, String referral) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_referral_multiplier',
      [tx.object(pool.address), tx.object(referral)],
      [base.type, quote.type],
    );
  }

  List<int> _encodeAddress(String address) {
    final hex = address.startsWith('0x') ? address.substring(2) : address;
    final bytes = <int>[];
    for (var i = 0; i < hex.length; i += 2) {
      bytes.add(int.parse(hex.substring(i, i + 2), radix: 16));
    }
    return bytes;
  }
}
