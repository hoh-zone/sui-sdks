# Rust SDK sui 包补全 TODO List

## P0 - 核心功能 (必须实现)

### 1.1 Transactions 模块拆分与增强

#### 1.1.1 创建 transactions 子模块结构

**任务**: 将 `transactions.rs` (233 行) 拆分为多个子模块

**文件结构**:
```
rust-sdks/crates/sui/src/transactions/
├── mod.rs
├── commands.rs       * 新建
├── arguments.rs      * 新建
├── inputs.rs         * 新建
├── serializer.rs     * 新建
├── resolve.rs        * 新建
├── object_cache.rs   * 新建
├── object.rs         * 新建
├── pure.rs           * 新建
├── hash.rs           * 新建 (来自 TS hash.ts)
├── utils.rs          * 新建
└── plugins/          * 新建子目录
    └── mod.rs
```

**实现细节**:

##### commands.rs (参考 Commands.ts)
```rust
// Move 指令构建
pub struct Command {
    pub kind: CommandKind,
}

pub enum CommandKind {
    // 参考 TS SDK Commands.ts:
    // - MoveCall
    // - TransferObjects
    // - SplitCoin
    // - MergeCoins
    // - PublishPackage
    // - MakeMoveVec
    // - etc.
}
```

**测试方法**:
- 测试每种命令类型的创建
- 测试命令序列化/反序列化
- 测试 BCS 编解码
- 测试指令构建的正确性

##### arguments.rs (参考 Arguments.ts)
```rust
// 命令参数处理
pub struct Argument(pub ArgumentKind);

pub enum ArgumentKind {
    Gas Coin,
    Input(u32),
    Result(u32, u32),
    NestedResult(u32, u32, u32),
}
```

**测试方法**:
- 测试参数类型创建
- 测试参数序列化
- 测试参数引用解析
- 测试嵌套参数处理

##### inputs.rs (参考 Inputs.ts)
```rust
// 交易输入
pub enum TransactionInput {
    Call(Vec<SuiValue>),
    Pure(SuiValue),
    Object(ObjectReference),
}
```

**测试方法**:
- 测试输入类型创建
- 测试 BCS 编解码
- 测试不同输入类型的序列化

##### serializer.rs (参考 serializer.ts + utils.ts)
```rust
// 交易序列化器
pub struct TransactionSerializer;

impl TransactionSerializer {
    // 参考 TS SDK:
    // - serializeTransactionData
    // - serializeTransactionDataV1
    // - serializeTransaction
}
```

**测试方法**:
- 测试交易数据序列化 (v1, v2)
- 测试意图消息处理
- 测试签名生成

##### resolve.rs (参考 resolve.ts)
```rust
// 地址/资源解析
pub struct Resolver;

impl Resolver {
    // 参考 TS SDK:
    // - resolveAddress
    // - resolveObjectRef
    // - resolveTypeParameter
}
```

**测试方法**:
- 测试地址解析
- 测试对象引用解析
- 测试类型参数解析

##### object_cache.rs (参考 ObjectCache.ts)
```rust
// 对象缓存机制 (优化交易构建)
pub struct ObjectCache {
    cache: HashMap<String, ObjectValue>,
}

impl ObjectCache {
    pub fn get(&self, id: &str) -> Option<&ObjectValue>;
    pub fn set(&mut self, id: String, value: ObjectValue);
    pub fn clear(&mut self);
}
```

**测试方法**:
- 测试缓存设置/获取
- 测试缓存命中
- 测试缓存过期 (如有)
- 测试并发访问

##### object.rs (参考 object.ts)
```rust
// 对象处理
pub enum Object {
    ImmOrOwned(ObjectRef),
    Shared(SharedObjectRef),
}
```

**测试方法**:
- 测试对象类型判别
- 测试对象引用创建
- 测试对象所有者验证

##### pure.rs (参考 pure.ts)
```rust
// 纯值处理 (用于 BCS)
pub fn serialize_pure<T: Serialize>(value: &T) -> Result<Vec<u8>>;
pub fn deserialize_pure<T: DeserializeOwned>(bytes: &[u8]) -> Result<T>;
```

**测试方法**:
- 测试纯值序列化
- 测试各种类型 (u8, u16, u32, u64, u128, bool, String, Vector)
- 测试嵌套值

##### hash.rs (参考 hash.ts)
```rust
// 交易哈希
pub fn transaction_digest(tx_data: &TransactionData) -> [u8; 32];
pub fn transaction_message(tx_data: &TransactionData) -> Vec<u8>;
```

**测试方法**:
- 测试交易摘要生成
- 测试交易消息生成
- 测试哈希一致性

##### utils.rs (参考 utils.ts)
```rust
// 交易工具函数
pub fn normalize_sui_address(addr: &str) -> String;
pub fn validate_sui_address(addr: &str) -> bool;
```

**测试方法**:
- 测试地址标准化
- 测试地址验证
- 测试边界情况

##### plugins/ (参考 plugins/)
```rust
// 事务插件系统
pub trait TransactionPlugin: Send + Sync {
    fn before_build(&mut self, tx: &mut Transaction) -> Result<()>;
    fn after_build(&mut self, tx: &Transaction) -> Result<()>;
}

pub struct TransactionBuilder {
    plugins: Vec<Box<dyn TransactionPlugin>>,
}
```

**测试方法**:
- 测试插件注册
- 测试插件生命周期钩子
- 测试插件顺序
- 测试插件错误处理

---

### 1.2 BCS 功能扩展

#### 1.2.1 effects.rs (参考 effects.ts)
**功能**: 处理交易效果

**实现细节**:
```rust
pub mod effects {
    pub use sui_types::effects::*;

    pub struct Effects {
        pub tnx_digest: [u8; 32],
        pub effects: TransactionEffects,
    }

    pub fn parse_effects(bytes: &[u8]) -> Result<Effects> {
        let effects = bcs::from_bytes::<TransactionEffects>(bytes)?;
        Ok(Effects { tnx_digest: tx_digest, effects })
    }
}
```

**测试方法**:
- 测试 BCS 解析交易效果
- 测试效果序列化/反序列化
- 测试不同效果类型
- 测试状态变更

#### 1.2.2 pure.rs (已在 transactions/ 中)
**功能**: 纯值序列化 (复用 transactions/pure.rs)

#### 1.2.3 type_tag_serializer.rs (参考 type-tag-serializer.ts)
**功能**: 类型标签序列化

**实现细节**:
```rust
pub mod type_tag_serializer {
    use serde::Serialize;

    pub fn serialize_type_tag(tag: &TypeTag) -> Result<Vec<u8>> {
        bcs::to_bytes(tag).map_err(Into::into)
    }

    pub fn deserialize_type_tag(bytes: &[u8]) -> Result<TypeTag> {
        bcs::from_bytes(bytes).map_err(Into::into)
    }
}
```

**TypeTag 类型**:
```rust
pub enum TypeTag {
    Bool,
    U8, U16, U32, U64, U128,
    U256,
    Address,
    Signer,
    Vector(Box<TypeTag>),
    Struct(StructTag),
    U8Vector,
    String,
}
```

测试方法:
- 测试类型标签序列化
- 测试类型标签反序列化
- 测试嵌套类型
- 测试特殊类型

---

### 1.3 Client 扩展

#### 1.3.1 subscription.rs (新建)
**功能**: 订阅/流式处理

**实现细节**:
```rust
pub mod subscription {
    use tokio::sync::mpsc;

    pub struct Subscription<T> {
        receiver: mpsc::UnboundedReceiver<T>,
    }

    impl<T> Subscription<T> {
        pub async fn next(&mut self) -> Option<T> {
            self.receiver.recv().await
        }

        pub async fn close(&mut self) {
            self.receiver.close();
        }
    }

    pub struct SubscriptionManager {
        subscriptions: HashMap<String, Box<dyn Any + Send + Sync>>,
    }
}
```

**测试方法**:
- 测试订阅创建
- 测试订阅消息接收
- 测试订阅关闭
- 测试多个订阅
- 测试订阅管理

#### 1.3.2 batch.rs (新建)
**功能**: 批量请求

**实现细节**:
```rust
pub mod batch {
    use serde_json::Value;

    pub struct BatchRequest {
        requests: Vec<Request>,
    }

    impl BatchRequest {
        pub fn new() -> Self {
            BatchRequest { requests: Vec::new() }
        }

        pub fn add(&mut self, request: Request) {
            self.requests.push(request);
        }

        pub async fn send(self, client: &SuiClient) -> Result<Vec<Value>> {
            // 实现批量发送
        }
    }
}
```

**测试方法**:
- 测试批量请求构建
- 测试批量发送
- 测试批量响应解析
- 测试错误处理

#### 1.3.3 events.rs (新建)
**功能**: 事件监听

**实现细节**:
```rust
pub mod events {
    pub struct EventSubscriber<T> {
        callback: Box<dyn Fn(T) + Send + Sync>,
    }

    impl<T> EventSubscriber<T> {
        pub fn new<F>(callback: F) -> Self
        where
            F: Fn(T) + Send + Sync + 'static,
        {
            EventSubscriber { callback: Box::new(callback) }
        }

        pub fn on_event(&self, event: T) {
            (self.callback)(event);
        }
    }
}
```

**测试方法**:
- 测试事件订阅
- 测试事件触发
- 测试多个订阅
- 测试取消订阅

---

### 1.4 Utils 工具库拆分

#### 1.4.1 创建 utils 子模块结构

**文件结构**:
```
rust-sdks/crates/sui/src/sui/utils/
├── mod.rs
├── address.rs      * 新建
├── resource.rs     * 新建
├── validators.rs   * 新建
├── wallet.rs       * 新建
└── object.rs       * 新建
```

**实现细节**:

##### address.rs
```rust
pub mod address {
    pub const SUI_ADDRESS_LENGTH: usize = 32;

    pub fn normalize_sui_address(addr: &str) -> String {
        let addr = addr.trim_start_matches("0x");
        let addr = format!("{:0>64}", addr);
        format!("0x{}", &addr[addr.len() - 64..])
    }

    pub fn validate_sui_address(addr: &str) -> bool {
        // 地址验证逻辑
    }

    pub fn is_invalid_sui_address(addr: &str) -> bool {
        !validate_sui_address(addr)
    }
}
```

测试方法:
- 测试地址标准化
- 测试地址验证
- 测试地址长度
- 测试地址格式
- 测试主网/测试网地址

##### resource.rs
```rust
pub mod resource {
    use serde_json::Value;

    pub fn parse_struct_tag(uri: &str) -> Result<StructTag>;

    pub fn get_resource_value(fields: Value) -> Result<Value>;

    pub fn decode_resource_object(obj: &Object) -> Result<ObjectValue>;
}
```

测试方法:
- 测试结构标签解析
- 测试资源值获取
- 测试资源对象解码

##### validators.rs
```rust
pub mod validators {
    pub fn validate_transaction_digest(digest: &[u8]) -> bool {
        digest.len() == 32
    }

    pub fn validate_signature(signature: &[u8]) -> bool {
        // 签名验证逻辑
    }

    pub fn validate_public_key(pubkey: &[u8]) -> bool {
        // 公钥验证逻辑
    }
}
```

测试方法:
- 测试交易摘要验证
- 测试签名验证
- 测试公钥验证

##### wallet.rs
```rust
pub mod wallet {
    pub fn derive_wallet_address(keypair: &Keypair) -> String;
    pub fn is_valid_wallet_address(addr: &str) -> bool;
}
```

测试方法:
- 测试钱包地址派生
- 测试钱包地址验证

##### object.rs
```rust
pub mod object {
    pub fn get_object_digest(obj: &Object) -> [u8; 32];
    pub fn get_object_version(obj: &Object) -> u64;
}
```

测试方法:
- 测试对象摘要获取
- 测试对象版本获取

---

### 1.5 cryptography 加密组织

#### 1.5.1 创建 cryptography 子目录
**功能**: 整合分散的加密功能

**文件结构**:
```
rust-sdks/crates/sui/src/sui/cryptography/
├── mod.rs
├── ed25519.rs      * 拆自 keypairs/ed25519.rs
├── secp256k1.rs    * 拆自 keypairs/secp256k1.rs
├── secp256r1.rs    * 拆自 keypairs/secp256r1.rs
├── keypair.rs      * 新建
└── signature.rs    * 新建
```

**实现细节**:

##### keypair.rs
```rust
pub mod keypair {
    use super::ed25519::Ed25519Keypair;
    use super::secp256k1::Secp256k1Keypair;
    use super::secp256r1::Secp256r1Keypair;

    pub trait Keypair: Send + Sync {
        type PublicKey;
        type Signature;

        fn public_key(&self) -> Self::PublicKey;
        fn sign(&self, message: &[u8]) -> Self::Signature;
    }
}
```

##### signature.rs
```rust
pub mod signature {
    pub enum Signature {
        Ed25519(ed25519::Signature),
        Secp256k1(secp256k1::Signature),
        Secp256r1(secp256r1::Signature),
    }
}
```

---

## P1 - 重要功能 (建议实现)

### 2.1 重连机制

#### 2.1.1 reconnect.rs (新建)
**实现细节**:
```rust
pub mod reconnect {
    pub struct ReconnectStrategy {
        max_retries: usize,
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    }

    pub async fn with_retry<T, F>(
        mut f: F,
        strategy: &ReconnectStrategy,
    ) -> Result<T>
    where
        F: FnMut() -> Pin<Box<dyn Future<Output = Result<T>> + Send>>,
    {
        let mut attempt = 0;
        let mut delay = strategy.initial_delay;

        loop {
            attempt += 1;
            match f().await {
                Ok(value) => return Ok(value),
                Err(e) if attempt < strategy.max_retries => {
                    async_std::task::sleep(delay).await;
                    delay = (delay.as_millis() as f64 * strategy.multiplier) as u64;
                    delay = delay.min(strategy.max_delay.as_millis() as u64);
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

测试方法:
- 测试重试策略
- 测试指数退避
- 测试最大延迟
- 测试成功案例
- 测试失败案例

---

### 2.2 类型系统增强

#### 2.2.1 types/ 子目录
**实现细节**:
```rust
rust-sdks/crates/sui/src/types/
├── mod.rs
├── transaction.rs  (交易类型)
├── object.rs       (对象类型)
├── coin.rs         (Coin 类型)
├── gas.rs          (Gas 类型)
└── dynamic.rs      (动态类型)
```

---

## P2 - 辅助功能 (可选)

### 3.1 Kiosk 包 (参考 @mysten/kiosk)

**文件**:
```
rust-sdks/crates/kiosk/
├── src/
│   ├── mod.rs
│   ├── client.rs
│   ├── types.rs
│   └── transactions.rs
└── Cargo.toml
```

**功能**: Kiosk 市场/NFT 交易接口

---

### 3.2 WALLET 工具

#### 3.2.1 wallet/ 子目录
```
rust-sdks/crates/sui/src/sui/wallet/
├── mod.rs
├── mnemonic.rs  (助记词处理)
├── derivation.rs (密钥派生)
└── recovery.rs  (恢复功能)
```

---

## 测试策略

### 单元测试
每个新模块需包含完整的单元测试:
- 正常情况测试
- 边界情况测试
- 错误情况测试
- 并发安全测试 (如有需要)

### 集成测试
使用 `httpmock` 模拟 RPC 响应:
```rust
#[tokio::test]
async fn test_transaction_build() {
    let server = MockServer::start();
    let client = SuiClient::new(server.url()).await;

    // 测试交易构建
    let tx = client.build_transaction(...).await.unwrap();

    // 验证交易
    assert!(tx.is_valid());
}
```

### 性能测试
对关键路径进行性能测试:
```rust
#[bench]
fn serialize_transaction_benchmark(b: &mut Bencher) {
    b.iter(|| {
        // 性能测试代码
    });
}
```

---

## 实施顺序

1. **Week 1-2**: P0 - Transactions 模块拆分
   - Week 1: commands, arguments, inputs
   - Week 2: serializer, resolve, object_cache, plugins

2. **Week 3**: P0 - BCS 功能扩展
   - effects, type_tag_serializer

3. **Week 4**: P0 - Utils 工具库
   - address, resource, validators, wallet, object

4. **Week 5**: P0 - Client 扩展
   - subscription, batch, events

5. **Week 6**: P1 - cryptography 重组
   - ed25519, secp256k1, secp256r1 整合

6. **Week 7-8**: P1 - 功能增强
   - 重连机制、类型系统

7. **Week 9-10**: P2 - 可选功能
   - Kiosk 包、Wallet 工具

---

## 完成标准

每个模块的完成标准:
- [ ] 所有 API 实现完成
- [ ] 所有测试用例通过
- [ ] 代码覆盖率 > 90%
- [ ] 文档完整
- [ ] 编译通过 (no warnings)
- [ ] 性能测试通过

---