import '../../sui/transactions.dart';
import '../config.dart';
import '../types.dart';
import 'encode.dart';

class BalanceManagerContract {
  const BalanceManagerContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> createAndShareBalanceManager(Transaction tx) {
    final manager = tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::new',
      [],
      const [],
    );
    return tx.moveCall(
      '0x2::transfer::public_share_object',
      [manager],
      [
        '${config.packageIds.deepbookPackageId}::balance_manager::BalanceManager'
      ],
    );
  }

  Map<String, dynamic> createBalanceManagerWithOwner(
      Transaction tx, String ownerAddress) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::new_with_custom_owner',
      [tx.pure(_encodeAddress(ownerAddress))],
      const [],
    );
  }

  Map<String, dynamic> shareBalanceManager(Transaction tx, dynamic manager) {
    return tx.moveCall(
      '0x2::transfer::public_share_object',
      [manager],
      [
        '${config.packageIds.deepbookPackageId}::balance_manager::BalanceManager'
      ],
    );
  }

  Map<String, dynamic> depositIntoManager(Transaction tx, String managerKey,
      String coinKey, double amountToDeposit) {
    final managerId = config.getBalanceManager(managerKey).address;
    final coin = config.getCoin(coinKey);
    final depositInput = (amountToDeposit * coin.scalar).round();
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::deposit',
      [tx.object(managerId), tx.pure(encodeU64(depositInput))],
      [coin.type],
    );
  }

  Map<String, dynamic> withdrawFromManager(Transaction tx, String managerKey,
      String coinKey, double amountToWithdraw, String recipient) {
    final managerId = config.getBalanceManager(managerKey).address;
    final coin = config.getCoin(coinKey);
    final withdrawInput = (amountToWithdraw * coin.scalar).round();
    final coinObject = tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::withdraw',
      [tx.object(managerId), tx.pure(encodeU64(withdrawInput))],
      [coin.type],
    );
    return tx.transferObjects([coinObject], tx.pure(_encodeAddress(recipient)));
  }

  Map<String, dynamic> withdrawAllFromManager(
      Transaction tx, String managerKey, String coinKey, String recipient) {
    final managerId = config.getBalanceManager(managerKey).address;
    final coin = config.getCoin(coinKey);
    final withdrawalCoin = tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::withdraw_all',
      [tx.object(managerId)],
      [coin.type],
    );
    return tx
        .transferObjects([withdrawalCoin], tx.pure(_encodeAddress(recipient)));
  }

  Map<String, dynamic> checkManagerBalance(
      Transaction tx, String managerKey, String coinKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    final coin = config.getCoin(coinKey);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::balance',
      [tx.object(managerId)],
      [coin.type],
    );
  }

  Map<String, dynamic> generateProof(Transaction tx, String managerKey) {
    final manager = config.getBalanceManager(managerKey);
    if (manager.tradeCap.isNotEmpty) {
      return generateProofAsTrader(tx, manager.address, manager.tradeCap);
    }
    return generateProofAsOwner(tx, manager.address);
  }

  Map<String, dynamic> generateProofAsOwner(Transaction tx, String managerId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::generate_proof_as_owner',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> generateProofAsTrader(
      Transaction tx, String managerId, String tradeCapId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::generate_proof_as_trader',
      [tx.object(managerId), tx.object(tradeCapId)],
      const [],
    );
  }

  Map<String, dynamic> mintTradeCap(Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::mint_trade_cap',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> mintDepositCap(Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::mint_deposit_cap',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> mintWithdrawalCap(Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::mint_withdraw_cap',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> depositWithCap(Transaction tx, String managerKey,
      String coinKey, double amountToDeposit) {
    final manager = config.getBalanceManager(managerKey);
    final coin = config.getCoin(coinKey);
    final depositInput = (amountToDeposit * coin.scalar).round();
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::deposit_with_cap',
      [
        tx.object(manager.address),
        tx.object(manager.depositCap),
        tx.pure(encodeU64(depositInput))
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> withdrawWithCap(Transaction tx, String managerKey,
      String coinKey, double amountToWithdraw) {
    final manager = config.getBalanceManager(managerKey);
    final coin = config.getCoin(coinKey);
    final withdrawAmount = (amountToWithdraw * coin.scalar).round();
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::withdraw_with_cap',
      [
        tx.object(manager.address),
        tx.object(manager.withdrawCap),
        tx.pure(encodeU64(withdrawAmount))
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> setBalanceManagerReferral(
      Transaction tx, String managerKey, String referral, dynamic tradeCap) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::set_balance_manager_referral',
      [tx.object(managerId), tx.object(referral), tradeCap],
      const [],
    );
  }

  Map<String, dynamic> unsetBalanceManagerReferral(
      Transaction tx, String managerKey, String poolKey, dynamic tradeCap) {
    final managerId = config.getBalanceManager(managerKey).address;
    final poolId = config.getPool(poolKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::unset_balance_manager_referral',
      [tx.object(managerId), tx.pure(encodeU128(poolId)), tradeCap],
      const [],
    );
  }

  Map<String, dynamic> registerBalanceManager(
      Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::register_balance_manager',
      [tx.object(managerId), tx.object(config.packageIds.registryId)],
      const [],
    );
  }

  Map<String, dynamic> owner(Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::owner',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> id(Transaction tx, String managerKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::id',
      [tx.object(managerId)],
      const [],
    );
  }

  Map<String, dynamic> balanceManagerReferralOwner(
      Transaction tx, String referralId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::balance_manager_referral_owner',
      [tx.object(referralId)],
      const [],
    );
  }

  Map<String, dynamic> balanceManagerReferralPoolId(
      Transaction tx, String referralId) {
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::balance_manager_referral_pool_id',
      [tx.object(referralId)],
      const [],
    );
  }

  Map<String, dynamic> getBalanceManagerReferralId(
      Transaction tx, String managerKey, String poolKey) {
    final managerId = config.getBalanceManager(managerKey).address;
    final poolId = config.getPool(poolKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::get_balance_manager_referral_id',
      [tx.object(managerId), tx.pure(encodeU128(poolId))],
      const [],
    );
  }

  Map<String, dynamic> revokeTradeCap(
      Transaction tx, String managerKey, String tradeCapId) {
    final managerId = config.getBalanceManager(managerKey).address;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::balance_manager::revoke_trade_cap',
      [tx.object(managerId), tx.pure(encodeU128(tradeCapId))],
      const [],
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

  Map<String, dynamic> modifyOrder(Transaction tx, String poolKey,
      String balanceManagerKey, String orderId, double newQuantity) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);
    final inputQuantity = (newQuantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::modify_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU128(orderId)),
        tx.pure(encodeU64(inputQuantity)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> cancelAllOrders(
      Transaction tx, String poolKey, String balanceManagerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::cancel_all_orders',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> withdrawSettledAmounts(
      Transaction tx, String poolKey, String balanceManagerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::withdraw_settled_amounts',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> withdrawSettledAmountsPermissionless(
      Transaction tx, String poolKey, String balanceManagerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::withdraw_settled_amounts_permissionless',
      [
        tx.object(pool.address),
        tx.object(manager.address),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getOrders(
      Transaction tx, String poolKey, List<String> orderIds) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_orders',
      [tx.object(pool.address), tx.pure(encodeVecU128(orderIds))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> accountOpenOrders(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account_open_orders',
      [tx.object(pool.address), tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getLevel2Range(Transaction tx, String poolKey,
      double priceLow, double priceHigh, bool isBid) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_level2_range',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(
            ((priceLow * floatScalar * quote.scalar) / base.scalar).round())),
        tx.pure(encodeU64(
            ((priceHigh * floatScalar * quote.scalar) / base.scalar).round())),
        tx.pure(encodeBool(isBid)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getLevel2TicksFromMid(
      Transaction tx, String poolKey, int tickFromMid) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_level2_ticks_from_mid',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(tickFromMid)),
        tx.object('0x6')
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> vaultBalances(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::vault_balances',
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

  Map<String, dynamic> whitelisted(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::whitelisted',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> burnDeep(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::burn_deep',
      [tx.object(pool.address), tx.object(config.packageIds.deepTreasuryId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> claimRebates(
      Transaction tx, String poolKey, String balanceManagerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::claim_rebates',
      [tx.object(pool.address), tx.object(manager.address), proof],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> mintReferral(
      Transaction tx, String poolKey, double multiplier) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final adjustedNumber = (multiplier * floatScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::mint_referral',
      [tx.object(pool.address), tx.pure(encodeU64(adjustedNumber))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> updatePoolReferralMultiplier(
      Transaction tx, String poolKey, String referral, double multiplier) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final adjustedNumber = (multiplier * floatScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::update_pool_referral_multiplier',
      [
        tx.object(pool.address),
        tx.object(referral),
        tx.pure(encodeU64(adjustedNumber))
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> claimPoolReferralRewards(
      Transaction tx, String poolKey, String referral) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::claim_pool_referral_rewards',
      [tx.object(pool.address), tx.object(referral)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> updatePoolAllowedVersions(
      Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::update_pool_allowed_versions',
      [tx.object(pool.address), tx.object(config.packageIds.registryId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> addDeepPricePoint(
      Transaction tx, String targetPoolKey, String referencePoolKey) {
    final targetPool = config.getPool(targetPoolKey);
    final referencePool = config.getPool(referencePoolKey);
    final targetBase = config.getCoin(targetPool.baseCoin);
    final targetQuote = config.getCoin(targetPool.quoteCoin);
    final referenceBase = config.getCoin(referencePool.baseCoin);
    final referenceQuote = config.getCoin(referencePool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::add_deep_price_point',
      [
        tx.object(targetPool.address),
        tx.object(referencePool.address),
        tx.object('0x6'),
      ],
      [
        targetBase.type,
        targetQuote.type,
        referenceBase.type,
        referenceQuote.type
      ],
    );
  }

  Map<String, dynamic> poolTradeParams(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_trade_params',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> poolBookParams(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_book_params',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> account(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final managerId = config.getBalanceManager(managerKey).address;

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::account',
      [tx.object(pool.address), tx.object(managerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> lockedBalance(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final managerId = config.getBalanceManager(managerKey).address;

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::locked_balance',
      [tx.object(pool.address), tx.object(managerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getPoolDeepPrice(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_order_deep_price',
      [tx.object(pool.address)],
      [base.type, quote.type],
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

  Map<String, dynamic> poolReferralMultiplier(
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

  Map<String, dynamic> stablePool(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::stable_pool',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> registeredPool(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::registered_pool',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuoteQuantityOutInputFee(
      Transaction tx, String poolKey, double baseQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_out_input_fee',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((baseQuantity * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getBaseQuantityOutInputFee(
      Transaction tx, String poolKey, double quoteQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_base_quantity_out_input_fee',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((quoteQuantity * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuantityOutInputFee(Transaction tx, String poolKey,
      double baseQuantity, double quoteQuantity) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quantity_out_input_fee',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((baseQuantity * base.scalar).round())),
        tx.pure(encodeU64((quoteQuantity * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getBaseQuantityIn(Transaction tx, String poolKey,
      double targetQuoteQuantity, bool payWithDeep) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_base_quantity_in',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((targetQuoteQuantity * quote.scalar).round())),
        tx.pure(encodeBool(payWithDeep)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getQuoteQuantityIn(Transaction tx, String poolKey,
      double targetBaseQuantity, bool payWithDeep) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_quote_quantity_in',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((targetBaseQuantity * base.scalar).round())),
        tx.pure(encodeBool(payWithDeep)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getAccountOrderDetails(
      Transaction tx, String poolKey, String managerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(managerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_account_order_details',
      [tx.object(pool.address), tx.object(manager.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getOrderDeepRequired(
      Transaction tx, String poolKey, double baseQuantity, double price) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final inputPrice =
        ((price * floatScalar * quote.scalar) / base.scalar).round();
    final inputQuantity = (baseQuantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_order_deep_required',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(inputQuantity)),
        tx.pure(encodeU64(inputPrice))
      ],
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

  Map<String, dynamic> canPlaceMarketOrder(
      Transaction tx, CanPlaceMarketOrderParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quantity = (params.quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::can_place_market_order',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.pure(encodeU64(quantity)),
        tx.pure(encodeBool(params.isBid)),
        tx.pure(encodeBool(params.payWithDeep)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> checkLimitOrderParams(Transaction tx, String poolKey,
      double price, double quantity, bool isBid) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final inputPrice =
        ((price * floatScalar * quote.scalar) / base.scalar).round();
    final inputQuantity = (quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::check_limit_order_params',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(inputPrice)),
        tx.pure(encodeU64(inputQuantity)),
        tx.pure(encodeBool(isBid)),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> checkMarketOrderParams(
      Transaction tx, String poolKey, double quantity, bool isBid) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final inputQuantity = (quantity * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::check_market_order_params',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(inputQuantity)),
        tx.pure(encodeBool(isBid)),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> poolTradeParamsNext(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::pool_trade_params_next',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> quorum(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::quorum',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> id(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::id',
      [tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> createPermissionlessPool(
      Transaction tx,
      String baseCoinKey,
      String quoteCoinKey,
      double tickSize,
      double lotSize,
      double minSize) {
    final base = config.getCoin(baseCoinKey);
    final quote = config.getCoin(quoteCoinKey);
    final adjustedTickSize =
        ((tickSize * floatScalar * quote.scalar) / base.scalar).round();
    final adjustedLotSize = (lotSize * base.scalar).round();
    final adjustedMinSize = (minSize * base.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::create_permissionless_pool',
      [
        tx.object(config.packageIds.registryId),
        tx.pure(encodeU64(adjustedTickSize)),
        tx.pure(encodeU64(adjustedLotSize)),
        tx.pure(encodeU64(adjustedMinSize)),
      ],
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

class GovernanceContract {
  const GovernanceContract(this.config, this.balanceManager);

  final DeepBookConfig config;
  final BalanceManagerContract balanceManager;

  Map<String, dynamic> stake(Transaction tx, String poolKey,
      String balanceManagerKey, double stakeAmount) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);
    final stakeInput = (stakeAmount * deepScalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::stake',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU64(stakeInput)),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> unstake(
      Transaction tx, String poolKey, String balanceManagerKey) {
    final pool = config.getPool(poolKey);
    final manager = config.getBalanceManager(balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::unstake',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> submitProposal(
      Transaction tx, SubmitProposalParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final proof = balanceManager.generateProof(tx, params.balanceManagerKey);

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::submit_proposal',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        proof,
        tx.pure(encodeU64((params.takerFee * floatScalar).round())),
        tx.pure(encodeU64((params.makerFee * floatScalar).round())),
        tx.pure(encodeU64((params.stakeRequired * deepScalar).round())),
      ],
      [base.type, quote.type],
    );
  }

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

  Map<String, dynamic> returnBaseAsset(Transaction tx, String poolKey,
      double borrowAmount, dynamic baseCoinInput, dynamic flashLoan) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final baseCoinReturn = tx.splitCoins(baseCoinInput,
        [tx.pure(encodeU64((borrowAmount * base.scalar).round()))]);

    tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::return_flashloan_base',
      [tx.object(pool.address), baseCoinReturn, flashLoan],
      [base.type, quote.type],
    );

    return baseCoinInput;
  }

  Map<String, dynamic> borrowQuoteAsset(
      Transaction tx, String poolKey, double borrowAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final amount = (borrowAmount * quote.scalar).round();

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::borrow_flashloan_quote',
      [tx.object(pool.address), tx.pure(encodeU64(amount))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> returnQuoteAsset(Transaction tx, String poolKey,
      double borrowAmount, dynamic quoteCoinInput, dynamic flashLoan) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final quoteCoinReturn = tx.splitCoins(quoteCoinInput,
        [tx.pure(encodeU64((borrowAmount * quote.scalar).round()))]);

    tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::return_flashloan_quote',
      [tx.object(pool.address), quoteCoinReturn, flashLoan],
      [base.type, quote.type],
    );

    return quoteCoinInput;
  }
}

class MarginManagerContract {
  const MarginManagerContract(this.config);

  final DeepBookConfig config;

  Map<String, dynamic> newMarginManager(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::new',
      [
        tx.object(pool.address),
        tx.object(config.packageIds.registryId),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> depositBase(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::deposit',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.pure(encodeU64((amount * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, base.type],
    );
  }

  Map<String, dynamic> depositQuote(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::deposit',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.pure(encodeU64((amount * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, quote.type],
    );
  }

  Map<String, dynamic> depositDeep(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final deep = config.getCoin('DEEP');
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::deposit',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.pure(encodeU64((amount * deep.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, deep.type],
    );
  }

  Map<String, dynamic> withdrawBase(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::withdraw',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.pure(encodeU64((amount * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, base.type],
    );
  }

  Map<String, dynamic> withdrawQuote(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::withdraw',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.pure(encodeU64((amount * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, quote.type],
    );
  }

  Map<String, dynamic> withdrawDeep(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final deep = config.getCoin('DEEP');
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::withdraw',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.pure(encodeU64((amount * deep.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type, deep.type],
    );
  }

  Map<String, dynamic> borrowBase(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::borrow_base',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.pure(encodeU64((amount * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> borrowQuote(
      Transaction tx, String managerKey, double amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::borrow_quote',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(quoteMarginPool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.pure(encodeU64((amount * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> repayBase(
      Transaction tx, String managerKey, double? amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::repay_base',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        amount != null
            ? tx.pure([1, ...encodeU64((amount * base.scalar).round())])
            : tx.pure([0]),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> repayQuote(
      Transaction tx, String managerKey, double? amount) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::repay_quote',
      [
        tx.object(manager.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(quoteMarginPool.address),
        amount != null
            ? tx.pure([1, ...encodeU64((amount * quote.scalar).round())])
            : tx.pure([0]),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> liquidate(Transaction tx, String managerAddress,
      String poolKey, bool debtIsBase, dynamic repayCoin) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    final marginPool = debtIsBase ? baseMarginPool : quoteMarginPool;
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::liquidate',
      [
        tx.object(managerAddress),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(marginPool.address),
        tx.object(pool.address),
        repayCoin,
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> owner(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::owner',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> deepbookPool(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::deepbook_pool',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> balanceManager(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::balance_manager',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> calculateAssets(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::calculate_assets',
      [tx.object(marginManagerId), tx.object(pool.address)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> managerState(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::manager_state',
      [
        tx.object(marginManagerId),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(pool.address),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> baseBalance(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::base_balance',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> quoteBalance(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::quote_balance',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> deepBalance(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::deep_balance',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> borrowedShares(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::borrowed_shares',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> hasBaseDebt(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::has_base_debt',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> getMarginAccountOrderDetails(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    final bm = tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::balance_manager',
      [tx.object(manager.address)],
      [base.type, quote.type],
    );

    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::get_account_order_details',
      [tx.object(pool.address), bm],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> marginPoolId(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::margin_pool_id',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> calculateDebts(
      Transaction tx, String poolKey, String coinKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final debtCoin = config.getCoin(coinKey);
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::calculate_debts',
      [
        tx.object(marginManagerId),
        tx.object(marginPool.address),
        tx.object('0x6')
      ],
      [base.type, quote.type, debtCoin.type],
    );
  }

  Map<String, dynamic> setMarginManagerReferral(
      Transaction tx, String managerKey, String referral) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::set_margin_manager_referral',
      [tx.object(manager.address), tx.object(referral)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> unsetMarginManagerReferral(
      Transaction tx, String managerKey, String poolKey) {
    final manager = config.getMarginManager(managerKey);
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::unset_margin_manager_referral',
      [tx.object(manager.address), tx.pure(encodeU128(pool.address))],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> newMarginManagerWithInitializer(
      Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::new_with_initializer',
      [
        tx.object(pool.address),
        tx.object(config.packageIds.registryId),
        tx.object(config.packageIds.marginRegistryId),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> shareMarginManager(
      Transaction tx, String poolKey, dynamic manager, dynamic initializer) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::share',
      [manager, initializer],
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

  Map<String, dynamic> newCondition(Transaction tx, String poolKey,
      bool triggerBelowPrice, double triggerPrice) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final inputPrice =
        ((triggerPrice * floatScalar * quote.scalar) / base.scalar).round();
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::tpsl::new_condition',
      [tx.pure(encodeBool(triggerBelowPrice)), tx.pure(encodeU64(inputPrice))],
      const [],
    );
  }

  Map<String, dynamic> cancelAllConditionalOrders(
      Transaction tx, String marginManagerKey) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::cancel_all_conditional_orders',
      [tx.object(manager.address), tx.object('0x6')],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> cancelConditionalOrder(
      Transaction tx, String marginManagerKey, String conditionalOrderId) {
    final manager = config.getMarginManager(marginManagerKey);
    final pool = config.getPool(manager.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::cancel_conditional_order',
      [
        tx.object(manager.address),
        tx.pure(encodeU64(int.parse(conditionalOrderId))),
        tx.object('0x6')
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> executeConditionalOrders(Transaction tx,
      String managerAddress, String poolKey, int maxOrdersToExecute) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::execute_conditional_orders',
      [
        tx.object(managerAddress),
        tx.object(pool.address),
        tx.object(base.priceInfoObjectId),
        tx.object(quote.priceInfoObjectId),
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU64(maxOrdersToExecute)),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> conditionalOrderIds(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::conditional_order_ids',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> conditionalOrder(Transaction tx, String poolKey,
      String marginManagerId, String conditionalOrderId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::conditional_order',
      [
        tx.object(marginManagerId),
        tx.pure(encodeU64(int.parse(conditionalOrderId)))
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> lowestTriggerAbovePrice(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::lowest_trigger_above_price',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> highestTriggerBelowPrice(
      Transaction tx, String poolKey, String marginManagerId) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);

    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_manager::highest_trigger_below_price',
      [tx.object(marginManagerId)],
      [base.type, quote.type],
    );
  }
}

class SwapParams {
  const SwapParams({
    required this.poolKey,
    required this.amount,
    this.minOut = 0.0,
    this.deepAmount = 0.0,
  });
  final String poolKey;
  final double amount;
  final double minOut;
  final double deepAmount;
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
  final String poolKey;
  final String balanceManagerKey;
  final String tradeCap;
  final String depositCap;
  final String withdrawCap;
  final double amount;
  final double minOut;
}

class SwapMethods {
  const SwapMethods(this.config);
  final DeepBookConfig config;

  Map<String, dynamic> swapExactBaseForQuote(
      Transaction tx, SwapParams params) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_base_for_quote',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((params.amount * base.scalar).round())),
        tx.pure(encodeU64((params.deepAmount * deepScalar).round())),
        tx.pure(encodeU64((params.minOut * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> swapExactQuoteForBase(
      Transaction tx, SwapParams params) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quote_for_base',
      [
        tx.object(pool.address),
        tx.pure(encodeU64((params.amount * quote.scalar).round())),
        tx.pure(encodeU64((params.deepAmount * deepScalar).round())),
        tx.pure(encodeU64((params.minOut * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> swapExactQuantity(
      Transaction tx, SwapParams params, bool isBaseToCoin) {
    final pool = config.getPool(params.poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final minOutScalar = isBaseToCoin ? quote.scalar : base.scalar;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quantity',
      [
        tx.object(pool.address),
        tx.pure(encodeU64(
            isBaseToCoin ? (params.amount * base.scalar).round() : 0)),
        tx.pure(encodeU64(
            isBaseToCoin ? 0 : (params.amount * quote.scalar).round())),
        tx.pure(encodeU64((params.deepAmount * deepScalar).round())),
        tx.pure(encodeU64((params.minOut * minOutScalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> swapExactBaseForQuoteWithManager(
      Transaction tx, SwapWithManagerParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_base_for_quote_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64((params.amount * base.scalar).round())),
        tx.pure(encodeU64((params.minOut * quote.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> swapExactQuoteForBaseWithManager(
      Transaction tx, SwapWithManagerParams params) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quote_for_base_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64((params.amount * quote.scalar).round())),
        tx.pure(encodeU64((params.minOut * base.scalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> swapExactQuantityWithManager(
      Transaction tx, SwapWithManagerParams params, bool isBaseToCoin) {
    final pool = config.getPool(params.poolKey);
    final manager = config.getBalanceManager(params.balanceManagerKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final minOutScalar = isBaseToCoin ? quote.scalar : base.scalar;
    return tx.moveCall(
      '${config.packageIds.deepbookPackageId}::pool::swap_exact_quantity_with_manager',
      [
        tx.object(pool.address),
        tx.object(manager.address),
        tx.object(params.tradeCap),
        tx.object(params.depositCap),
        tx.object(params.withdrawCap),
        tx.pure(encodeU64(
            isBaseToCoin ? (params.amount * base.scalar).round() : 0)),
        tx.pure(encodeU64(
            isBaseToCoin ? 0 : (params.amount * quote.scalar).round())),
        tx.pure(encodeU64((params.minOut * minOutScalar).round())),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
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
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::supply',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(supplierCap),
        tx.pure(encodeU64((amountToDeposit * coin.scalar).round())),
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
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::withdraw',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(supplierCap),
        amountToWithdraw != null
            ? tx.pure(
                [1, ...encodeU64((amountToWithdraw * coin.scalar).round())])
            : tx.pure([0]),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> mintSupplierCap(Transaction tx) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::mint_supplier_cap',
      [tx.object(config.packageIds.marginRegistryId), tx.object('0x6')],
      const [],
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

  Map<String, dynamic> getId(Transaction tx, String coinKey) {
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::id',
      [tx.object(marginPool.address)],
      [marginPool.type],
    );
  }

  Map<String, dynamic> deepbookPoolAllowed(
      Transaction tx, String coinKey, String deepbookPoolId) {
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::deepbook_pool_allowed',
      [tx.object(marginPool.address), tx.pure(encodeU128(deepbookPoolId))],
      [marginPool.type],
    );
  }

  Map<String, dynamic> userSupplyShares(
      Transaction tx, String coinKey, String supplierCapId) {
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::user_supply_shares',
      [tx.object(marginPool.address), tx.pure(encodeU128(supplierCapId))],
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
        tx.pure(encodeU128(supplierCapId)),
        tx.object('0x6')
      ],
      [marginPool.type],
    );
  }
}

class MarginLiquidationsContract {
  const MarginLiquidationsContract(this.config);
  final DeepBookConfig config;

  Map<String, dynamic> createLiquidationVault(
      Transaction tx, String liquidationAdminCap) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::create_liquidation_vault',
      [tx.object(liquidationAdminCap)],
      const [],
    );
  }

  Map<String, dynamic> deposit(Transaction tx, String vaultId,
      String liquidationAdminCap, String coinKey, double amount) {
    final coin = config.getCoin(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::deposit',
      [
        tx.object(vaultId),
        tx.object(liquidationAdminCap),
        tx.pure(encodeU64((amount * coin.scalar).round())),
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> withdraw(Transaction tx, String vaultId,
      String liquidationAdminCap, String coinKey, double amount) {
    final coin = config.getCoin(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::withdraw',
      [
        tx.object(vaultId),
        tx.object(liquidationAdminCap),
        tx.pure(encodeU64((amount * coin.scalar).round())),
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> balance(Transaction tx, String vaultId, String coinKey) {
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::balance',
      [tx.object(vaultId)],
      [config.getCoin(coinKey).type],
    );
  }

  Map<String, dynamic> liquidateBase(Transaction tx, String vaultId,
      String managerAddress, String poolKey, double? repayAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::liquidate_base',
      [
        tx.object(vaultId),
        tx.object(managerAddress),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(pool.address),
        tx.pure(repayAmount != null
            ? [1, ...encodeU64((repayAmount * base.scalar).round())]
            : [0]),
        tx.object('0x6'),
      ],
      [base.type, quote.type],
    );
  }

  Map<String, dynamic> liquidateQuote(Transaction tx, String vaultId,
      String managerAddress, String poolKey, double? repayAmount) {
    final pool = config.getPool(poolKey);
    final base = config.getCoin(pool.baseCoin);
    final quote = config.getCoin(pool.quoteCoin);
    final baseMarginPool = config.getMarginPool(pool.baseCoin);
    final quoteMarginPool = config.getMarginPool(pool.quoteCoin);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::liquidation_vault::liquidate_quote',
      [
        tx.object(vaultId),
        tx.object(managerAddress),
        tx.object(config.packageIds.marginRegistryId),
        tx.object(baseMarginPool.address),
        tx.object(quoteMarginPool.address),
        tx.object(pool.address),
        tx.pure(repayAmount != null
            ? [1, ...encodeU64((repayAmount * quote.scalar).round())]
            : [0]),
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

  Map<String, dynamic> createInterestConfig(Transaction tx, double baseRate,
      double baseSlope, double optimalUtilization, double excessSlope) {
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

  Map<String, dynamic> createMarginPool(Transaction tx, String coinKey,
      dynamic poolConfig, String marginMaintainerCap) {
    final coin = config.getCoin(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::create_margin_pool',
      [
        tx.object(config.packageIds.marginRegistryId),
        poolConfig,
        tx.object(marginMaintainerCap),
        tx.object('0x6')
      ],
      [coin.type],
    );
  }

  Map<String, dynamic> updateInterestParams(Transaction tx, String coinKey,
      String marginPoolCap, dynamic interestConfig) {
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::update_interest_params',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        interestConfig,
        tx.object(marginPoolCap),
        tx.object('0x6')
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> updateMarginPoolConfig(Transaction tx, String coinKey,
      String marginPoolCap, dynamic marginPoolConfig) {
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::update_margin_pool_config',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        marginPoolConfig,
        tx.object(marginPoolCap),
        tx.object('0x6')
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> enableDeepbookPoolForLoan(Transaction tx,
      String deepbookPoolKey, String coinKey, String marginPoolCap) {
    final deepbookPool = config.getPool(deepbookPoolKey);
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::enable_deepbook_pool_for_loan',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(deepbookPool.address)),
        tx.object(marginPoolCap),
        tx.object('0x6'),
      ],
      [marginPool.type],
    );
  }

  Map<String, dynamic> disableDeepbookPoolForLoan(Transaction tx,
      String deepbookPoolKey, String coinKey, String marginPoolCap) {
    final deepbookPool = config.getPool(deepbookPoolKey);
    final marginPool = config.getMarginPool(coinKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_pool::disable_deepbook_pool_for_loan',
      [
        tx.object(marginPool.address),
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(deepbookPool.address)),
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
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::liquidation_risk_ratio',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> minBorrowRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::min_borrow_risk_ratio',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> minWithdrawRiskRatio(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::min_withdraw_risk_ratio',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> targetLiquidationRiskRatio(
      Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::target_liquidation_risk_ratio',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> userLiquidationReward(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::user_liquidation_reward',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> poolLiquidationReward(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::pool_liquidation_reward',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
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

  Map<String, dynamic> getDeepbookPoolMarginPoolIds(
      Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::get_deepbook_pool_margin_pool_ids',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> baseMarginPoolId(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::base_margin_pool_id',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
    );
  }

  Map<String, dynamic> quoteMarginPoolId(Transaction tx, String poolKey) {
    final pool = config.getPool(poolKey);
    return tx.moveCall(
      '${config.packageIds.marginPackageId}::margin_registry::quote_margin_pool_id',
      [
        tx.object(config.packageIds.marginRegistryId),
        tx.pure(encodeU128(pool.address))
      ],
      const [],
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
