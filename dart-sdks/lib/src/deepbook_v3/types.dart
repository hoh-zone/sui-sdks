class BalanceManager {
  const BalanceManager({
    required this.address,
    this.tradeCap = '',
    this.depositCap = '',
    this.withdrawCap = '',
  });

  final String address;
  final String tradeCap;
  final String depositCap;
  final String withdrawCap;
}

class MarginManager {
  const MarginManager({required this.address, required this.poolKey});

  final String address;
  final String poolKey;
}

class MarginPool {
  const MarginPool({
    required this.address,
    required this.type,
  });

  final String address;
  final String type;
}

class Coin {
  const Coin({
    required this.address,
    required this.type,
    required this.scalar,
    this.priceInfoObjectId,
  });

  final String address;
  final String type;
  final double scalar;
  final String? priceInfoObjectId;
}

class Pool {
  const Pool(
      {required this.address, required this.baseCoin, required this.quoteCoin});

  final String address;
  final String baseCoin;
  final String quoteCoin;
}

enum OrderType {
  noRestriction,
  immediateOrCancel,
  fillOrKill,
  postOnly,
}

enum SelfMatchingOptions {
  selfMatchingAllowed,
  cancelTaker,
  cancelMaker,
}

class PlaceLimitOrderParams {
  const PlaceLimitOrderParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.clientOrderId,
    required this.price,
    required this.quantity,
    required this.isBid,
    this.expiration = 0,
    this.orderType = OrderType.noRestriction,
    this.selfMatchingOption = SelfMatchingOptions.selfMatchingAllowed,
    this.payWithDeep = true,
  });

  final String poolKey;
  final String balanceManagerKey;
  final String clientOrderId;
  final double price;
  final double quantity;
  final bool isBid;
  final int expiration;
  final OrderType orderType;
  final SelfMatchingOptions selfMatchingOption;
  final bool payWithDeep;
}

class PlaceMarketOrderParams {
  const PlaceMarketOrderParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.clientOrderId,
    required this.quantity,
    required this.isBid,
    this.selfMatchingOption = SelfMatchingOptions.selfMatchingAllowed,
    this.payWithDeep = true,
  });

  final String poolKey;
  final String balanceManagerKey;
  final String clientOrderId;
  final double quantity;
  final bool isBid;
  final SelfMatchingOptions selfMatchingOption;
  final bool payWithDeep;
}

class CanPlaceLimitOrderParams {
  const CanPlaceLimitOrderParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.price,
    required this.quantity,
    required this.isBid,
    required this.payWithDeep,
    required this.expireTimestamp,
  });

  final String poolKey;
  final String balanceManagerKey;
  final double price;
  final double quantity;
  final bool isBid;
  final bool payWithDeep;
  final int expireTimestamp;
}

class CanPlaceMarketOrderParams {
  const CanPlaceMarketOrderParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.quantity,
    required this.isBid,
    required this.payWithDeep,
  });

  final String poolKey;
  final String balanceManagerKey;
  final double quantity;
  final bool isBid;
  final bool payWithDeep;
}

class PlaceMarginLimitOrderParams {
  const PlaceMarginLimitOrderParams({
    required this.poolKey,
    required this.marginManagerKey,
    required this.clientOrderId,
    required this.price,
    required this.quantity,
    required this.isBid,
    this.expiration = 0,
    this.orderType = OrderType.noRestriction,
    this.selfMatchingOption = SelfMatchingOptions.selfMatchingAllowed,
    this.payWithDeep = true,
  });

  final String poolKey;
  final String marginManagerKey;
  final String clientOrderId;
  final double price;
  final double quantity;
  final bool isBid;
  final int expiration;
  final OrderType orderType;
  final SelfMatchingOptions selfMatchingOption;
  final bool payWithDeep;
}

class PlaceMarginMarketOrderParams {
  const PlaceMarginMarketOrderParams({
    required this.poolKey,
    required this.marginManagerKey,
    required this.clientOrderId,
    required this.quantity,
    required this.isBid,
    this.selfMatchingOption = SelfMatchingOptions.selfMatchingAllowed,
    this.payWithDeep = true,
  });

  final String poolKey;
  final String marginManagerKey;
  final String clientOrderId;
  final double quantity;
  final bool isBid;
  final SelfMatchingOptions selfMatchingOption;
  final bool payWithDeep;
}

class SubmitProposalParams {
  const SubmitProposalParams({
    required this.poolKey,
    required this.balanceManagerKey,
    required this.takerFee,
    required this.makerFee,
    required this.stakeRequired,
  });

  final String poolKey;
  final String balanceManagerKey;
  final double takerFee;
  final double makerFee;
  final double stakeRequired;
}

class CreatePoolAdminParams {
  const CreatePoolAdminParams({
    required this.baseCoinKey,
    required this.quoteCoinKey,
    required this.tickSize,
    required this.lotSize,
    required this.minSize,
    this.whitelisted = false,
    this.stablePool = false,
  });

  final String baseCoinKey;
  final String quoteCoinKey;
  final double tickSize;
  final double lotSize;
  final double minSize;
  final bool whitelisted;
  final bool stablePool;
}

class SetEwmaParams {
  const SetEwmaParams({
    required this.alpha,
    required this.zScoreThreshold,
    required this.additionalTakerFee,
  });

  final double alpha;
  final double zScoreThreshold;
  final double additionalTakerFee;
}

class MarginProposalParams {
  const MarginProposalParams({
    required this.takerFee,
    required this.makerFee,
    required this.stakeRequired,
  });

  final double takerFee;
  final double makerFee;
  final double stakeRequired;
}

class MarginPoolConfigParams {
  const MarginPoolConfigParams({
    required this.supplyCap,
    required this.maxUtilizationRate,
    required this.referralSpread,
    required this.minBorrow,
    this.rateLimitCapacity,
    this.rateLimitRefillRatePerMs,
    this.rateLimitEnabled,
  });

  final double supplyCap;
  final double maxUtilizationRate;
  final double referralSpread;
  final double minBorrow;
  final double? rateLimitCapacity;
  final double? rateLimitRefillRatePerMs;
  final bool? rateLimitEnabled;
}

class InterestConfigParams {
  const InterestConfigParams({
    required this.baseRate,
    required this.baseSlope,
    required this.optimalUtilization,
    required this.excessSlope,
  });

  final double baseRate;
  final double baseSlope;
  final double optimalUtilization;
  final double excessSlope;
}
