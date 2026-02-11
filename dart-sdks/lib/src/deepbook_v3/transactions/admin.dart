import '../sui/transactions.dart';
import 'config.dart';

/// 管理员操作 for Dart SDK (P3)
class AdminContract {
  AdminContract(this.config);

  final DeepBookConfig config;

  /// Create a new pool with admin privileges
  Map<String, dynamic> createPoolAdmin(
    Transaction tx,
    String baseCoinKey,
    String quoteCoinKey,
    double tickSize,
    double lotSize,
    double minSize,
  ) {
    final base = config.getCoin(baseCoinKey);
    final quote = config.getCoin(quoteCoinKey);
    final adjustedTickSize =
        (tickSize * floatScalar * quote.scalar / base.scalar).round();
    final adjustedLotSize = (lotSize * base.scalar).round();
    final adjustedMinSize = (minSize * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::create_pool_admin',
      [
        tx.pureU64(adjustedTickSize),
        tx.pureU64(adjustedLotSize),
        tx.pureU64(adjustedMinSize),
      ],
      [base.type, quote.type],
    );
  }

  /// Set the tick size for a pool
  Map<String, dynamic> setTickSize(
    Transaction tx,
    String poolKey,
    double tickSize,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final adjustedTickSize =
        (tickSize * floatScalar * quote.scalar / base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::set_tick_size',
      [
        tx.pureU64(adjustedTickSize),
      ],
      [base.type, quote.type],
    );
  }

  /// Set the lot size for a pool
  Map<String, dynamic> setLotSize(
    Transaction tx,
    String poolKey,
    double lotSize,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final adjustedLotSize = (lotSize * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::set_lot_size',
      [
        tx.pureU64(adjustedLotSize),
      ],
      [base.type, quote.type],
    );
  }

  /// Set base price point
  Map<String, dynamic> setBasePricePoint(
    Transaction tx,
    String poolKey,
    double price,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final adjustedPrice =
        (price * floatScalar * quote.scalar / base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::set_base_price_point',
      [
        tx.pureU64(adjustedPrice),
      ],
      [base.type, quote.type],
    );
  }

  /// Set quote price point
  Map<String, dynamic> setQuotePricePoint(
    Transaction tx,
    String poolKey,
    double price,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final adjustedPrice =
        (price * floatScalar * quote.scalar / base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::set_quote_price_point',
      [
        tx.pureU64(adjustedPrice),
      ],
      [base.type, quote.type],
    );
  }

  /// Update allowed pool versions
  Map<String, dynamic> updateAllowedVersions(
    Transaction tx,
    String poolKey,
    String registryId,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::update_pool_allowed_versions',
      [
        tx.object(pool.address),
        tx.object(registryId),
      ],
      [base.type, quote.type],
    );
  }

  /// Withdraw all assets from a pool
  Map<String, dynamic> withdrawAll(
    Transaction tx,
    String poolKey,
    String recipient,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::withdraw_all',
      [
        tx.object(pool.address),
        tx.object(recipient),
      ],
      [base.type, quote.type],
    );
  }

  /// Remove a pool
  Map<String, dynamic> removePool(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::remove_pool',
      [
        tx.object(pool.address),
        tx.object(config.packageIds.registryId),
      ],
      [base.type, quote.type],
    );
  }

  /// Burn DEEP tokens
  Map<String, dynamic> burnDeep(
    Transaction tx,
    String poolKey,
  ) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::burn_deep',
      [
        tx.object(pool.address),
        tx.object(config.packageIds.deepTreasuryId),
      ],
      [base.type, quote.type],
    );
  }
}
