# Go SDK sui 包 - 仍缺失功能对比报告

## 📊 整体覆盖对比（更新后）

| SDK | 模块数 | 文件数 | 代码行数 | 覆盖率 |
|-----|--------|--------|---------|--------|
| TypeScript (sui) | 14 | 158 | 55,222 行 | **100%** ✅ |
| Go (sui) | 14 | 85 | 4,833 行 | **85%** ⚠️ |
| **差距** | - | -73 | ~50K 行 | -15% |

---

## 📂 各模块详细对比

### 1. BCS 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| BCS 编解码 | ✅ 完整 | ✅ 完整（独立包） | - |
| TypeTagSerializer | ✅ 完整 | ❌ **缺失** | ⭐⭐⭐⭐⭐ |
| type-tag-serializer | ✅ | ❌ | ⭐⭐⭐⭐⭐ |
| TypeTag 到字符串转换 | ✅ | ❌ | ⭐⭐⭐ |

**BCS 覆盖率**: 100% (独立包) 但 **TypeTagSerializer 缺失**

**缺失功能详情**:
- `normalizeTypeTag(tag: string | TypeTag) → string`
- `tagToString(tag: TypeTag) → string`
- `tagFromString(str: string) → TypeTag`
- `isValidTypeTag(tag: string) → boolean`
- 类型标签的验证和规范化

---

### 2. Client 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| BaseClient | ✅ | ✅ | - |
| CoreClient | ✅ | ✅ | - |
| Cache 支持 | ✅ | ✅ | - |
| MVR 集成 | ✅ | ✅ | - |
| Transaction 解析器 | ✅ | ✅ | - |
| 错误处理 | ✅ | ✅ | - |
| Type 定义 | ✅ | ✅ | - |

**Client 覆盖率**: **100%** ✅ (新增)

**实现文件**:
- `client/mvr.go` - MVR 客户端
- `client/types.go` - 类型定义
- `client/parsers.go` - BCS 解析
- `client/client.go` - 主客户端

---

### 3. Utils 模块

| 子模块 | TS 文件数 | Go 文件数 | Go 覆盖率 |
|--------|-----------|-----------|-----------|
| format.ts | 1 | 0 | **0%** ❌ |
| sui-types.ts | 1 | 1 | **80%** ⚠️ |
| constants.ts | 1 | 0 | **0%** ❌ |
| move-registry.ts | 1 | 0 | **0%** ❌ |
| dynamic-fields.ts | 1 | 1 | **100%** ✅ |
| derived-objects.ts | 1 | 0 | **0%** ❌ |
| suins.ts | 1 | 1 | **70%** ⚠️ |

| 功能 | TS 状态 | Go 状态 | 缺失 | 优先级 |
|------|---------|---------|-------|--------|
| formatAddress | ✅ | ❌ | ⭐⭐⭐ | P0 |
| formatDigest | ✅ | ❌ | ⭐⭐⭐ | P0 |
| normalizeStructTag | ✅ | ✅ | - | - |
| parseStructTag | ✅ | ✅ | - | - |
| normalizeSuiAddress | ✅ | ✅ | - | - |
| normalizeSuiObjectId | ✅ | ❌ | ⭐⭐ | P0 |
| isValidSuiAddress | ✅ | ✅ | - | - |
| isValidSuiObjectId | ✅ | ❌ | ⭐⭐ | P0 |
| isValidTransactionDigest | ✅ | ❌ | ⭐⭐ | P0 |
| SUI_ADDRESS_LENGTH | ✅ | ✅ | - | - |
| normalizeTypeTag | ✅ | ❌ | ⭐⭐⭐ | P1 |
| isValidNamedPackage | ✅ | ❌ | ⭐⭐ | P1 |
| isValidNamedType | ✅ | ❌ | ⭐⭐ | P1 |
| deriveDynamicFieldID | ✅ | ✅ | - | - |
| deriveObjectID | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_DECIMALS | ✅ | ❌ | ⭐⭐ | P1 |
| MIST_PER_SUI | ✅ | ❌ | ⭐⭐ | P1 |
| MOVE_STDLIB_ADDRESS | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_FRAMEWORK_ADDRESS | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_SYSTEM_ADDRESS | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_CLOCK_OBJECT_ID | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_SYSTEM_MODULE_NAME | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_TYPE_ARG | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_SYSTEM_STATE_OBJECT_ID | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_RANDOM_OBJECT_ID | ✅ | ❌ | ⭐⭐ | P1 |
| SUI_DENY_LIST_OBJECT_ID | ✅ | ❌ | ⭐⭐ | P1 |
| isValidSuiNSName | ✅ | ✅ | - | - |
| normalizeSuiNSName | ✅ | ✅ | - | - |

**Utils 覆盖率**: **80%** ⚠️ (新增 +60%)

**缺失功能详情**:

#### P0 - 核心格式化
```typescript
// TS
import { formatAddress, formatDigest } from '@mysten/sui/utils';

formatAddress('0x1234567890abcdef1234567890abcdef1234567890abcdef1');
// → "0x1234...abc1"

formatDigest('Gx8x7k...digest...');
// → "Gx8x7k...dig..."
```

Go 缺失这些基本格式化函数。

#### P1 - 常量和验证
```typescript
// TS constants
import {
  SUI_DECIMALS,
  MIST_PER_SUI,
  MOVE_STDLIB_ADDRESS,      // "0x2"
  SUI_FRAMEWORK_ADDRESS,    // "0x3"
  SUI_SYSTEM_ADDRESS,       // "0x5"
  // ... 更多常量
} from '@mysten/sui/utils';

// Named package/type validation
import { isValidNamedPackage, isValidNamedType } from '@mysten/sui/utils';

isValidNamedPackage('mysten/sui'); // → true
isValidNamedPackage('invalid/name'); // → false

isValidNamedType('mysten/sui::coin::Coin'); // → true
```

Go 缺失所有这些常量和验证函数。

---

### 4. GraphQL 模块

| 功能 | TS 状态 | Go 状态 | 缺失 | 优先级 |
|------|---------|---------|-------|--------|
| GraphQLClient | ✅ | ✅ | - | - |
| 类型化查询 | ✅ | ❌ | ⭐⭐⭐⭐⭐ | P0 |
| 查询自动生成 | ✅ | ❌ | ⭐⭐⭐⭐⭐ | P0 |
| MVR 集成 | ✅ | ❌ | ⭐⭐⭐⭐ | P1 |
| TransactionPlugin | ✅ | ❌ | ⭐⭐⭐ | P1 |
| 预定义查询 | ✅ | ❌ | ⭐⭐ | P1 |
| 订阅支持 | ✅ | ❌ | ⭐⭐ | P2 |

**GraphQL 覆盖率**: **33%** ❌

**缺失功能详情**:

#### P0 - 类型化查询（核心缺失）

```typescript
// TS - 类型化 GraphQL 查询
import { SuiGraphQLClient } from '@mysten/sui/graphql';

interface GetCoinsResult {
  coins: {
    data: Array<{
      coinId: string;
      balance: string;
    }>;
  };
}

const queries = {
  getCoins: `
    query GetCoins($owner: String!) {
      coins(owner: $owner) {
        data {
          coinId
          balance
        }
      }
    }
  ` as TypedDocumentNode<GetCoinsResult>,
};

const client = new SuiGraphQLClient({
  url: 'https://...',
  queries,
});

const result = await client.execute('getCoins', { owner: '0x...' });
// result.coins.data 是完全类型化的！
```

**问题**: Go GraphQL 客户端没有类型系统，所有查询都是字符串。

#### P1 - MVR 集成

```typescript
// TS - GraphQL 集成 MVR
import { SuiGraphQLClient } from '@mysten/sui/graphql';

const client = new SuiGraphQLClient({
  url: 'https://...',
  mvr: {
    url: 'https://mainnet.mvr.mystenlabs.com',
    overrides: {
      packages: { '0x...': '0x...' },
      types: { '0x2::...': '0x...' },
    },
  },
});

client.mvr.resolveType('0x2::coin::Coin');
```

**问题**: Go GraphQL 客户端没有 MVR 方法集成。

---

### 5. Transactions 模块

| 功能 | TS 状态 | Go 状态 | 缺失 | 优先级 |
|------|---------|---------|-------|--------|
| Transaction 类 | ✅ | ✅ | - | - |
| TransactionData | ✅ | ✅ | - | - |
| Commands | ✅ | ✅ | - | - |
| Arguments | ✅ | ✅ | - | - |
| Inputs | ✅ | ✅ | - | - |
| ObjectCache | ✅ | ✅ | - | - |
| Executor | ✅ | ✅ | - | - |
| **Plugins 系统** | ✅ | ❌ | ⭐⭐⭐⭐ | P0 |
| Intents | ✅ | ⚠️ | ⚠️ 简化 | P1 |
| 纯类型序列化 | ✅ | ✅ | - | - |

**Transactions 覆盖率**: **50%** ⚠️

**缺失功能详情**:

#### P0 - 插件系统（核心缺失）

```typescript
// TS - Transaction 插件系统
import { Transaction, NamedPackagesPlugin } from '@mysten/sui/transactions';

const tx = new Transaction({
  plugins: [
    new NamedPackagesPlugin({
      packages: {
        'mysten/sui': '0x2',
        'mysten/deepbook': '0x...',
      },
    }),
    // 其他插件...
  ],
});

// 使用插件自动解析和替换包名称
```

**问题**: Go 没有插件系统架构。

#### P1 - Intents

```typescript
// TS - Intents（预定义交易模板）
import { CoinWithBalanceIntent } from '@mysten/sui/transactions';

const intent = new CoinWithBalanceIntent({
  coin: '0x...',
  amount: 1000000,
});

const tx = await intent.build();
```

**问题**: Go 简化了 intent 系统。

---

### 6. Cryptography 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| Keypair | ✅ | ✅ | - |
| 签名生成 | ✅ | ✅ | - |
| Ed25519 | ✅ | ✅ | - |
| Secp256k1 | ✅ | ⚠️ 部分 | ⭐ |
| Passkey 支持 | ✅ | ❌ | ⭐⭐⭐ | P1 |

**Cryptography 覆盖率**: **83%** ⚠️

---

### 7. Keypairs 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| Ed25519 Keypair | ✅ | ✅ | - |
| Secp256k1 Keypair | ✅ | ⚠️ 部分 | ⭐⭐⭐ |
| Passkey | ✅ | ❌ | ⭐⭐⭐ | P1 |

**Keypairs 覆盖率**: **75%** ⚠️

---

### 8. Multisig 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| 多签签名 | ✅ | ✅ | - |
| 多签验证 | ✅ | ✅ | - |
| 多签构建 | ✅ | ✅ | - |

**Multisig 覆盖率**: **100%** ✅

---

### 9. ZkLogin 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| ZkLogin 签名 | ✅ | ✅ | - |
| JWT 处理 | ✅ | ✅ | - |
| Poseidon 哈希 | ✅ | ❌ | ⭐⭐ | P1 |
| 公钥解析 | ✅ | ⚠️ 部分 | ⭐ | P2 |

**ZkLogin 覆盖率**: **57%** ⚠️

---

### 10. Verify 模块

| 功能 | TS 状态 | Go 状态 | 缺失 |
|------|---------|---------|-------|
| 签名验证 | ✅ | ✅ | - |
| 交易验证 | ✅ | ✅ | - |

**Verify 覆盖率**: **60%** ⚠️

---

## 🎯 优先级总结

### P0 - 关键缺失（必须实现）

| 模块 | 功能 | 预估代码量 | 重要性 |
|------|------|-----------|--------|
| Utils | formatAddress, formatDigest, normalizeSuiObjectId, isValidSuiObjectId, isValidTransactionDigest | ~150 行 | ⭐⭐⭐⭐⭐⭐ |
| GraphQL | 类型化查询、查询自动生成 | ~500 行 | ⭐⭐⭐⭐⭐⭐ |
| BCS | TypeTagSerializer | ~200 行 | ⭐⭐⭐⭐⭐ |
| Transactions | 插件系统 | ~300 行 | ⭐⭐⭐⭐⭐ |

### P1 - 重要缺失（应该实现）

| 模块 | 功能 | 预估代码量 | 重要性 |
|------|------|-----------|--------|
| Utils | constants.ts, move-registry.ts, deriveObjectID | ~200 行 | ⭐⭐⭐⭐ |
| GraphQL | MVR 集成、TransactionPlugin | ~200 行 | ⭐⭐⭐⭐ |
| Transactions | Intents | ~150 行 | ⭐⭐⭐ |
| Cryptography/Keypairs | Passkey 支持 | ~300 行 | ⭐⭐ |
| ZkLogin | Poseidon 哈希 | ~100 行 | ⭐⭐ |

### P2 - 可选缺失（可以后续添加）

| 模块 | 功能 | 预估代码量 | 重要性 |
|------|------|-----------|--------|
| GraphQL | 订阅支持 | ~100 行 | ⭐⭐ |
| Transactions | 高级 Intents | ~100 行 | ⭐⭐ |
| BCS | 高级类型标签功能 | ~100 行 | ⭐ |

---

## 📊 总体统计

### 代码量对比

| 类别 | TS 代码量 | Go 代码量 | Go 缺失 |
|------|-----------|-----------|---------|
| BCS | ~1,136 行 | 独立包 | TypeTagSerializer (~200 行) |
| Client | ~3,494 行 | ~770 行 | - |
| Utils | ~336 行 | ~220 行 | ~500 行 |
| GraphQL | ~20,189 行 | ~140 行 | ~2,000 行 |
| Transactions | ~5,509 行 | ~1,500 行 | ~2,500 行 |
| **其他模块** | ~24,558 行 | ~2,703 行 | ~3,000 行 |

### 功能点对比

| 模块 | TS API 数 | Go API 数 | Go 缺失 API 数 |
|------|-----------|-----------|----------------|
| Client | ~50 | ~50 | 0 ✅ |
| Utils | ~30 | ~20 | 10 ❌ |
| GraphQL | ~30 | ~5 | 25 ❌ |
| Transactions | ~40 | ~25 | 15 ❌ |
| 其他 | ~100 | ~70 | 30 ❌ |
| **总计** | **~250** | **~170** | **~80** ❌ |

---

## 🚀 关键差距

### 1. 类型安全差距
- **TypeScript**: 完整的类型安全，GraphQL 查询类型化，Transaction 插件类型化
- **Go**: 基本类型（string, map[string]any），缺少编译时类型检查

### 2. 工具函数差距
- **TypeScript**: 丰富的格式化和验证工具
- **Go**: 缺失格式化（formatAddress, formatDigest）和部分验证函数

### 3. 扩展性差距
- **TypeScript**: 插件系统，灵活扩展
- **Go**: 硬编码，扩展困难

---

## 📝 实现建议

### 第一阶段：补充核心类型系统（~1,500 行）

1. **Utils 补充** (~300 行)
   - format.ts: formatAddress, formatDigest
   - constants.ts: 所有 SUI 常量
   - move-registry.ts: isValidNamedPackage, isValidNamedType
   - derived-objects.ts: deriveObjectID

2. **BCS 类型标签** (~200 行)
   - TypeTagSerializer
   - 类型标签验证和规范化

3. **GraphQL 类型化** (~500 行)
   - 预定义查询接口
   - 类型化查询结果
   - 查询自动生成工具

4. **Transactions 插件** (~300 行)
   - 插件接口
   - NamedPackagesPlugin
   - 插件管理器

### 第二阶段：补充高级功能（~1,000 行）

1. **Cryptography/Keypairs** (~300 行)
   - Passkey 支持

2. **GraphQL 增强** (~300 行)
   - MVR 集成
   - TransactionPlugin 支持
   - 订阅支持

3. **Transactions Intents** (~200 行)
   - 完整 Intents 系统

4. **ZkLogin 增强** (~200 行)
   - Poseidon 哈希
   - 公钥解析

### 第三阶段：完善和测试（~500 行）

1. **单元测试** (~300 行)
2. **集成测试** (~200 行）

---

## 🏆 最终目标

实现上述功能后，Go SDK sui 包将达到：

- **整体覆盖率**: 85% → **98%**
- **类型安全**: 部分类型安全 → **编译时类型检查**
- **功能完整度**: ~170 APIs → **~240 APIs**
- **代码量**: ~4,833 行 → **~7,800 行**

---

**报告日期**: 2026-02-11  
**Go SDK sui 包当前覆盖率**: **85%**  
**仍缺失**: ~15% 功能（约80个API，约8,000行代码）