import '../sui/transactions.dart';
import 'config.dart';

/// 清算功能 for Dart SDK (P2)
class LiquidationContract {
  LiquidationContract(this.config);

  final DeepBookConfig config;

  /// Force liquidate an under-collateralized margin position
  Map<String, dynamic> forceLiquidate(
    Transaction tx,
    String marginManagerKey,
    String liquidatorKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::force_liquidate',
      [
        tx.object(manager.address),
        tx.object(liquidatorKey),
      ],
      [base.type, quote.type],
    );
  }

  /// Calculate liquidation amount for a position
  Map<String, dynamic> calculateLiquidation(
    Transaction tx,
    String marginManagerKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::calculate_liquidation',
      [
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Check if position is over-collateralized
  Map<String, dynamic> isOverCollateralized(
    Transaction tx,
    String marginManagerKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::is_over_collateralized',
      [
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get the liquidation price for a position
  Map<String, dynamic> getLiquidationPrice(
    Transaction tx,
    String marginManagerKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::get_liquidation_price',
      [
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get available collateral for liquidation
  Map<String, dynamic> getCollateral(
    Transaction tx,
    String marginManagerKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::get_collateral',
      [
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  /// Get debt information for a position
  Map<String, dynamic> getDebt(
    Transaction tx,
    String marginManagerKey,
  ) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::get_debt',
      [
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }
}
