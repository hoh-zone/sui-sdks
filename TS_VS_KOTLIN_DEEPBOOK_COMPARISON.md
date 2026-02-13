# TypeScript SDK vs Kotlin SDK - DeepBook V3 方法对比

## 整体对比

| 指标 | TypeScript SDK | Kotlin SDK | 差距 |
|------|---------------|------------|------|
| 交易构建文件数 | 13 | 1 | -12 |
| 总方法数 | ~280+ | ~15 | ~265 |
| **覆盖率** | **100%** | **~5%** | **-95%** |

---

## TypeScript SDK 交易模块

| 文件 | 方法数 | Kotlin状态 |
|------|--------|-----------|
| balanceManager.ts | ~20 | ❌ 缺失 |
| deepbook.ts | ~80 | ⚠️ 部分 (~5) |
| deepbookAdmin.ts | ~10 | ❌ 缺失 |
| flashLoans.ts | ~5 | ❌ 缺失 |
| governance.ts | ~15 | ❌ 缺失 |
| marginAdmin.ts | ~40 | ❌ 缺失 |
| marginLiquidations.ts | ~20 | ❌ 缺失 |
| marginMaintainer.ts | ~15 | ❌ 缺失 |
| marginManager.ts | ~30 | ❌ 缺失 |
| marginPool.ts | ~25 | ❌ 缺失 |
| marginRegistry.ts | ~10 | ❌ 缺失 |
| marginTPSL.ts | ~10 | ❌ 缺失 |
| poolProxy.ts | ~5 | ❌ 缺失 |

---

## 详细方法对比

### 1. BalanceManagerContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| createAndShareBalanceManager | ✅ | ❌ | 创建并共享余额管理器 |
| createBalanceManagerWithOwner | ✅ | ❌ | 使用自定义所有者创建 |
| shareBalanceManager | ✅ | ❌ | 共享余额管理器 |
| depositIntoManager | ✅ | ✅ | 存入资金 |
| withdrawFromManager | ✅ | ✅ | 提取资金 |
| withdrawAllFromManager | ✅ | ❌ | 提取所有资金 |
| checkManagerBalance | ✅ | ❌ | 检查余额 |
| generateProof | ✅ | ❌ | 生成证明 |
| generateProofAsOwner | ✅ | ❌ | 作为所有者生成证明 |
| generateProofAsTrader | ✅ | ❌ | 作为交易者生成证明 |
| **覆盖率** | **100%** | **20%** | **2/10** |

---

### 2. DeepBookContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| placeLimitOrder | ✅ | ✅ | 下限价单 |
| placeMarketOrder | ✅ | ✅ | 下市价单 |
| placeReduceOnlyLimitOrder | ✅ | ❌ | 仅减仓限价单 |
| placeReduceOnlyMarketOrder | ✅ | ❌ | 仅减仓市价单 |
| modifyOrder | ✅ | ❌ | 修改订单 |
| cancelOrder | ✅ | ✅ | 取消订单 |
| cancelOrders | ✅ | ❌ | 批量取消订单 |
| cancelAllOrders | ✅ | ❌ | 取消所有订单 |
| canPlaceLimitOrder | ✅ | ❌ | 检查限价单参数 |
| canPlaceMarketOrder | ✅ | ❌ | 检查市价单参数 |
| accountOpenOrders | ✅ | ❌ | 获取账户未完成订单 |
| getOrder | ✅ | ❌ | 获取订单详情 |
| getOrders | ✅ | ❌ | 批量获取订单 |
| account | ✅ | ❌ | 获取账户信息 |
| accountExists | ✅ | ❌ | 检查账户存在 |
| balance | ✅ | ❌ | 获取余额 |
| baseBalance | ✅ | ❌ | 获取基础币余额 |
| quoteBalance | ✅ | ❌ | 获取报价币余额 |
| deepBalance | ✅ | ❌ | 获取DEEP余额 |
| vaultBalances | ✅ | ❌ | 获取金库余额 |
| withdrawSettledAmounts | ✅ | ❌ | 提取已结算金额 |
| withdrawSettledAmountsPermissionless | ✅ | ❌ | 无权限提取 |
| withdrawMarginSettledAmounts | ✅ | ❌ | 保证金提取 |
| getLevel2Range | ✅ | ❌ | 获取Level2范围 |
| getLevel2TicksFromMid | ✅ | ❌ | 获取中间价tick |
| midPrice | ✅ | ❌ | 获取中间价 |
| whitelisted | ✅ | ❌ | 检查白名单 |
| getQuoteQuantityOut | ✅ | ❌ | 计算输出报价数量 |
| getBaseQuantityOut | ✅ | ❌ | 计算输出基础数量 |
| getQuantityOut | ✅ | ❌ | 计算输出数量 |
| burnDeep | ✅ | ❌ | 销毁DEEP |
| poolTradeParams | ✅ | ❌ | 获取池交易参数 |
| poolBookParams | ✅ | ❌ | 获取池订单簿参数 |
| claimRebates | ✅ | ❌ | 领取返佣 |
| addDeepPricePoint | ✅ | ❌ | 添加DEEP价格点 |
| mintReferral | ✅ | ❌ | 铸造推荐 |
| claimPoolReferralRewards | ✅ | ❌ | 领取池推荐奖励 |
| updatePoolAllowedVersions | ✅ | ❌ | 更新池允许版本 |
| getId | ✅ | ❌ | 获取ID |
| getPoolIdByAssets | ✅ | ❌ | 根据资产获取池ID |
| **覆盖率** | **100%** | **6%** | **4/~65** |

---

### 3. Swap相关方法

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| swapExactBaseForQuote | ✅ | ❌ | 用基础币换报价币 |
| swapExactQuoteForBase | ✅ | ❌ | 用报价币换基础币 |
| swapExactQuantity | ✅ | ❌ | 精确数量交换 |
| swapExactBaseForQuoteWithManager | ✅ | ❌ | 使用管理器交换 |
| swapExactQuoteForBaseWithManager | ✅ | ❌ | 使用管理器交换 |
| swapExactQuantityWithManager | ✅ | ❌ | 使用管理器精确交换 |
| getQuoteQuantityIn | ✅ | ❌ | 获取输入报价数量 |
| getBaseQuantityIn | ✅ | ❌ | 获取输入基础数量 |
| getQuoteQuantityOutInputFee | ✅ | ❌ | 获取输出报价数量含手续费 |
| getBaseQuantityOutInputFee | ✅ | ❌ | 获取输出基础数量含手续费 |
| getQuantityOutInputFee | ✅ | ❌ | 获取输出数量含手续费 |
| **覆盖率** | **100%** | **0%** | **0/11** |

---

### 4. DeepBookAdminContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| createPoolAdmin | ✅ | ❌ | 创建池管理员 |
| unregisterPoolAdmin | ✅ | ❌ | 注销池管理员 |
| createPermissionlessPool | ✅ | ❌ | 创建无需许可池 |
| registerDeepbookPool | ✅ | ❌ | 注册池 |
| enableDeepbookPool | ✅ | ❌ | 启用池 |
| disableDeepbookPool | ✅ | ❌ | 禁用池 |
| enableDeepbookPoolForLoan | ✅ | ❌ | 启用借贷池 |
| disableDeepbookPoolForLoan | ✅ | ❌ | 禁用借贷池 |
| allowedPauseCaps | ✅ | ❌ | 允许暂停上限 |
| mintPauseCap | ✅ | ❌ | 铸造暂停上限 |
| revokePauseCap | ✅ | ❌ | 撤销暂停上限 |
| disableVersion | ✅ | ❌ | 禁用版本 |
| enableVersion | ✅ | ❌ | 启用版本 |
| **覆盖率** | **100%** | **0%** | **0/13** |

---

### 5. FlashLoanContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| borrowBase | ✅ | ❌ | 借入基础币 |
| borrowQuote | ✅ | ❌ | 借入报价币 |
| returnBaseAsset | ✅ | ❌ | 归还基础币 |
| returnQuoteAsset | ✅ | ❌ | 归还报价币 |
| borrowBaseAsset | ✅ | ❌ | 借入基础资产 |
| borrowQuoteAsset | ✅ | ❌ | 借入报价资产 |
| **覆盖率** | **100%** | **0%** | **0/6** |

---

### 6. GovernanceContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| submitProposal | ✅ | ❌ | 提交提案 |
| vote | ✅ | ❌ | 投票 |
| quorum | ✅ | ❌ | 法定人数 |
| **覆盖率** | **100%** | **0%** | **0/3** |

---

### 7. MarginAdminContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| createMarginPool | ✅ | ❌ | 创建保证金池 |
| newMarginPoolConfig | ✅ | ❌ | 新建保证金池配置 |
| newMarginPoolConfigWithRateLimit | ✅ | ❌ | 带限速的配置 |
| newInterestConfig | ✅ | ❌ | 新建利息配置 |
| updateMarginPoolConfig | ✅ | ❌ | 更新保证金池配置 |
| updateInterestParams | ✅ | ❌ | 更新利息参数 |
| updateRiskParams | ✅ | ❌ | 更新风险参数 |
| mintMaintainerCap | ✅ | ❌ | 铸造维护者上限 |
| revokeMaintainerCap | ✅ | ❌ | 撤销维护者上限 |
| authorizeMarginApp | ✅ | ❌ | 授权保证金应用 |
| deauthorizeMarginApp | ✅ | ❌ | 取消授权保证金应用 |
| mintSupplierCap | ✅ | ❌ | 铸造供应者上限 |
| newProtocolConfig | ✅ | ❌ | 新建协议配置 |
| newPythConfig | ✅ | ❌ | 新建Pyth配置 |
| allowedMaintainers | ✅ | ❌ | 允许的维护者 |
| **覆盖率** | **100%** | **0%** | **0/15** |

---

### 8. MarginManagerContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| depositBase | ✅ | ❌ | 存入基础币 |
| depositQuote | ✅ | ❌ | 存入报价币 |
| withdrawBase | ✅ | ❌ | 提取基础币 |
| withdrawQuote | ✅ | ❌ | 提取报价币 |
| borrowBase | ✅ | ❌ | 借入基础币 |
| borrowQuote | ✅ | ❌ | 借入报价币 |
| repayBase | ✅ | ❌ | 偿还基础币 |
| repayQuote | ✅ | ❌ | 偿还报价币 |
| liquidate | ✅ | ❌ | 清算 |
| liquidateBase | ✅ | ❌ | 清算基础币 |
| liquidateQuote | ✅ | ❌ | 清算报价币 |
| managerState | ✅ | ❌ | 获取管理器状态 |
| getMarginAccountOrderDetails | ✅ | ❌ | 获取保证金账户订单详情 |
| getAccountOrderDetails | ✅ | ❌ | 获取账户订单详情 |
| placeLimitOrder | ✅ | ❌ | 保证金限价单 |
| placeMarketOrder | ✅ | ❌ | 保证金市价单 |
| cancelOrder | ✅ | ❌ | 取消保证金订单 |
| cancelAllOrders | ✅ | ❌ | 取消所有保证金订单 |
| withdrawSettledAmounts | ✅ | ❌ | 提取已结算金额 |
| placeReduceOnlyLimitOrder | ✅ | ❌ | 仅减仓限价单 |
| placeReduceOnlyMarketOrder | ✅ | ❌ | 仅减仓市价单 |
| **覆盖率** | **100%** | **0%** | **0/21** |

---

### 9. MarginPoolContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| supplyToMarginPool | ✅ | ❌ | 向保证金池供应 |
| withdrawFromMarginPool | ✅ | ❌ | 从保证金池提取 |
| borrowShares | ✅ | ❌ | 借入份额 |
| borrowedShares | ✅ | ❌ | 已借入份额 |
| calculateAssets | ✅ | ❌ | 计算资产 |
| calculateDebts | ✅ | ❌ | 计算债务 |
| totalSupply | ✅ | ❌ | 总供应 |
| totalBorrow | ✅ | ❌ | 总借入 |
| interestRate | ✅ | ❌ | 利率 |
| supplyCap | ✅ | ❌ | 供应上限 |
| minBorrow | ✅ | ❌ | 最小借入 |
| maxUtilizationRate | ✅ | ❌ | 最大利用率 |
| liquidationRiskRatio | ✅ | ❌ | 清算风险比率 |
| targetLiquidationRiskRatio | ✅ | ❌ | 目标清算风险比率 |
| minBorrowRiskRatio | ✅ | ❌ | 最小借入风险比率 |
| minWithdrawRiskRatio | ✅ | ❌ | 最小提取风险比率 |
| **覆盖率** | **100%** | **0%** | **0/16** |

---

### 10. MarginLiquidationsContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| liquidate | ✅ | ❌ | 清算 |
| liquidateBase | ✅ | ❌ | 清算基础币 |
| liquidateQuote | ✅ | ❌ | 清算报价币 |
| createLiquidationVault | ✅ | ❌ | 创建清算金库 |
| userLiquidationReward | ✅ | ❌ | 用户清算奖励 |
| poolLiquidationReward | ✅ | ❌ | 池清算奖励 |
| **覆盖率** | **100%** | **0%** | **0/6** |

---

### 11. MarginMaintainerContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| updateRiskParams | ✅ | ❌ | 更新风险参数 |
| updateInterestParams | ✅ | ❌ | 更新利息参数 |
| allowedMaintainers | ✅ | ❌ | 允许的维护者 |
| **覆盖率** | **100%** | **0%** | **0/3** |

---

### 12. MarginRegistryContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| registerBalanceManager | ✅ | ❌ | 注册余额管理器 |
| getBalanceManagerIds | ✅ | ❌ | 获取余额管理器ID |
| getMarginManagerIds | ✅ | ❌ | 获取保证金管理器ID |
| getMarginPoolId | ✅ | ❌ | 获取保证金池ID |
| **覆盖率** | **100%** | **0%** | **0/4** |

---

### 13. MarginTPSLContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| addConditionalOrder | ✅ | ❌ | 添加条件订单 |
| cancelConditionalOrder | ✅ | ❌ | 取消条件订单 |
| cancelAllConditionalOrders | ✅ | ❌ | 取消所有条件订单 |
| executeConditionalOrders | ✅ | ❌ | 执行条件订单 |
| conditionalOrder | ✅ | ❌ | 获取条件订单 |
| conditionalOrderIds | ✅ | ❌ | 获取条件订单ID |
| highestTriggerBelowPrice | ✅ | ❌ | 最高触发下限价 |
| lowestTriggerAbovePrice | ✅ | ❌ | 最低触发上限价 |
| **覆盖率** | **100%** | **0%** | **0/8** |

---

### 14. PoolProxyContract

| 方法 | TypeScript | Kotlin | 说明 |
|------|-----------|--------|------|
| getPoolDeepPrice | ✅ | ❌ | 获取池DEEP价格 |
| getPoolReferralBalances | ✅ | ❌ | 获取池推荐余额 |
| withdrawReferralFees | ✅ | ❌ | 提取推荐费用 |
| **覆盖率** | **100%** | **0%** | **0/3** |

---

## 统计总结

### 按模块统计

| 模块 | TypeScript方法数 | Kotlin方法数 | Kotlin覆盖率 |
|------|----------------|-------------|--------------|
| BalanceManager | ~10 | 2 | 20% |
| DeepBook | ~65 | 4 | 6% |
| Swap | ~11 | 0 | 0% |
| DeepBookAdmin | ~13 | 0 | 0% |
| FlashLoan | ~6 | 0 | 0% |
| Governance | ~3 | 0 | 0% |
| MarginAdmin | ~15 | 0 | 0% |
| MarginManager | ~21 | 0 | 0% |
| MarginPool | ~16 | 0 | 0% |
| MarginLiquidations | ~6 | 0 | 0% |
| MarginMaintainer | ~3 | 0 | 0% |
| MarginRegistry | ~4 | 0 | 0% |
| MarginTPSL | ~8 | 0 | 0% |
| PoolProxy | ~3 | 0 | 0% |
| **总计** | **~188** | **6** | **~3%** |

---

## Kotlin SDK 需要实现的方法

### P0 - 核心交易功能（高优先级）

#### DeepBookContract (~60方法)
- [ ] placeReduceOnlyLimitOrder
- [ ] placeReduceOnlyMarketOrder
- [ ] modifyOrder
- [ ] cancelOrders (批量)
- [ ] cancelAllOrders
- [ ] canPlaceLimitOrder
- [ ] canPlaceMarketOrder
- [ ] accountOpenOrders
- [ ] getOrder
- [ ] getOrders
- [ ] account
- [ ] accountExists
- [ ] balance
- [ ] baseBalance
- [ ] quoteBalance
- [ ] deepBalance
- [ ] vaultBalances
- [ ] withdrawSettledAmounts
- [ ] withdrawSettledAmountsPermissionless
- [ ] withdrawMarginSettledAmounts
- [ ] getLevel2Range
- [ ] getLevel2TicksFromMid
- [ ] midPrice
- [ ] whitelisted
- [ ] getQuoteQuantityOut
- [ ] getBaseQuantityOut
- [ ] getQuantityOut
- [ ] burnDeep
- [ ] poolTradeParams
- [ ] poolBookParams
- [ ] claimRebates
- [ ] addDeepPricePoint
- [ ] mintReferral
- [ ] claimPoolReferralRewards
- [ ] updatePoolAllowedVersions
- [ ] getId
- [ ] getPoolIdByAssets

#### Swap方法 (~11方法)
- [ ] swapExactBaseForQuote
- [ ] swapExactQuoteForBase
- [ ] swapExactQuantity
- [ ] swapExactBaseForQuoteWithManager
- [ ] swapExactQuoteForBaseWithManager
- [ ] swapExactQuantityWithManager
- [ ] getQuoteQuantityIn
- [ ] getBaseQuantityIn
- [ ] getQuoteQuantityOutInputFee
- [ ] getBaseQuantityOutInputFee
- [ ] getQuantityOutInputFee

#### BalanceManagerContract (~8方法)
- [ ] createAndShareBalanceManager
- [ ] createBalanceManagerWithOwner
- [ ] shareBalanceManager
- [ ] withdrawAllFromManager
- [ ] checkManagerBalance
- [ ] generateProof
- [ ] generateProofAsOwner
- [ ] generateProofAsTrader

**P0总计**: ~83方法

---

### P1 - 管理功能（中优先级）

#### DeepBookAdminContract (~13方法)
- [ ] createPoolAdmin
- [ ] unregisterPoolAdmin
- [ ] createPermissionlessPool
- [ ] registerDeepbookPool
- [ ] enableDeepbookPool
- [ ] disableDeepbookPool
- [ ] enableDeepbookPoolForLoan
- [ ] disableDeepbookPoolForLoan
- [ ] allowedPauseCaps
- [ ] mintPauseCap
- [ ] revokePauseCap
- [ ] disableVersion
- [ ] enableVersion

#### FlashLoanContract (~6方法)
- [ ] borrowBase
- [ ] borrowQuote
- [ ] returnBaseAsset
- [ ] returnQuoteAsset
- [ ] borrowBaseAsset
- [ ] borrowQuoteAsset

#### GovernanceContract (~3方法)
- [ ] submitProposal
- [ ] vote
- [ ] quorum

**P1总计**: ~22方法

---

### P2 - 保证金功能（低优先级）

#### MarginAdminContract (~15方法)
- [ ] createMarginPool
- [ ] newMarginPoolConfig
- [ ] newMarginPoolConfigWithRateLimit
- [ ] newInterestConfig
- [ ] updateMarginPoolConfig
- [ ] updateInterestParams
- [ ] updateRiskParams
- [ ] mintMaintainerCap
- [ ] revokeMaintainerCap
- [ ] authorizeMarginApp
- [ ] deauthorizeMarginApp
- [ ] mintSupplierCap
- [ ] newProtocolConfig
- [ ] newPythConfig
- [ ] allowedMaintainers

#### MarginManagerContract (~21方法)
- [ ] depositBase
- [ ] depositQuote
- [ ] withdrawBase
- [ ] withdrawQuote
- [ ] borrowBase
- [ ] borrowQuote
- [ ] repayBase
- [ ] repayQuote
- [ ] liquidate
- [ ] liquidateBase
- [ ] liquidateQuote
- [ ] managerState
- [ ] getMarginAccountOrderDetails
- [ ] getAccountOrderDetails
- [ ] placeLimitOrder
- [ ] placeMarketOrder
- [ ] cancelOrder
- [ ] cancelAllOrders
- [ ] withdrawSettledAmounts
- [ ] placeReduceOnlyLimitOrder
- [ ] placeReduceOnlyMarketOrder

#### MarginPoolContract (~16方法)
- [ ] supplyToMarginPool
- [ ] withdrawFromMarginPool
- [ ] borrowShares
- [ ] borrowedShares
- [ ] calculateAssets
- [ ] calculateDebts
- [ ] totalSupply
- [ ] totalBorrow
- [ ] interestRate
- [ ] supplyCap
- [ ] minBorrow
- [ ] maxUtilizationRate
- [ ] liquidationRiskRatio
- [ ] targetLiquidationRiskRatio
- [ ] minBorrowRiskRatio
- [ ] minWithdrawRiskRatio

#### MarginLiquidationsContract (~6方法)
- [ ] liquidate
- [ ] liquidateBase
- [ ] liquidateQuote
- [ ] createLiquidationVault
- [ ] userLiquidationReward
- [ ] poolLiquidationReward

#### MarginMaintainerContract (~3方法)
- [ ] updateRiskParams
- [ ] updateInterestParams
- [ ] allowedMaintainers

#### MarginRegistryContract (~4方法)
- [ ] registerBalanceManager
- [ ] getBalanceManagerIds
- [ ] getMarginManagerIds
- [ ] getMarginPoolId

#### MarginTPSLContract (~8方法)
- [ ] addConditionalOrder
- [ ] cancelConditionalOrder
- [ ] cancelAllConditionalOrders
- [ ] executeConditionalOrders
- [ ] conditionalOrder
- [ ] conditionalOrderIds
- [ ] highestTriggerBelowPrice
- [ ] lowestTriggerAbovePrice

#### PoolProxyContract (~3方法)
- [ ] getPoolDeepPrice
- [ ] getPoolReferralBalances
- [ ] withdrawReferralFees

**P2总计**: ~76方法

---

## 总体统计

| 优先级 | 方法数 | 预估代码量 | 说明 |
|--------|--------|-----------|------|
| **P0** | ~83 | ~4,150行 | 核心交易功能 |
| **P1** | ~22 | ~1,100行 | 管理功能 |
| **P2** | ~76 | ~3,800行 | 保证金功能 |
| **总计** | **~181** | **~9,050行** | |

---

## 实现优先级建议

### 第一阶段（P0核心功能，~4,150行）
1. **DeepBookContract扩展** (~2,000行)
   - 订单修改、批量取消
   - 账户查询、余额检查
   - Level2数据获取
   - 价格计算
   - 返佣领取

2. **Swap方法** (~500行)
   - 精确交换
   - 使用管理器交换
   - 手续费计算

3. **BalanceManager扩展** (~350行)
   - 创建管理器
   - 证明生成

### 第二阶段（P1管理功能，~1,100行）
1. **DeepBookAdminContract** (~500行)
2. **FlashLoanContract** (~300行)
3. **GovernanceContract** (~300行)

### 第三阶段（P2保证金功能，~3,800行）
1. **MarginAdminContract** (~600行)
2. **MarginManagerContract** (~800行)
3. **MarginPoolContract** (~600行)
4. **其他Margin合约** (~1,800行)

---

## 当前Kotlin SDK实现情况

### 已实现方法（6个）
1. ✅ placeLimitOrder
2. ✅ placeMarketOrder
3. ✅ cancelOrder
4. ✅ depositIntoManager
5. ✅ withdrawFromManager
6. ✅ openMarginPosition (MarginTransactionBuilder)
7. ✅ closeMarginPosition (MarginTransactionBuilder)
8. ✅ addCollateral (MarginTransactionBuilder)

### 覆盖率计算

| 总方法数 | 已实现 | 未实现 | 覆盖率 |
|---------|--------|--------|--------|
| ~188 | 8 | ~180 | **~4%** |

---

**报告日期**: 2026-02-11  
**TypeScript SDK覆盖率**: **100%**  
**Kotlin SDK覆盖率**: **~4%**  
**差距**: **~96%** (~180方法, ~9,000行代码)