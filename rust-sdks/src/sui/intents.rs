// Intent system for Rust SDK
pub mod intents;

pub const COIN_WITH_BALANCE: &str = "CoinWithBalance";
pub const SUI_TYPE: &str = "0x2::sui::SUI";

/// CoinWithBalance intent
#[derive(Debug, Clone)]
pub struct CoinWithBalance {
    pub name: String,
    pub coin_type: String,
    pub balance: u64,
    pub use_gas_coin: bool,
}

/// CoinWithBalanceBuilder for building CoinWithBalance intent
pub struct CoinWithBalanceBuilder {
    balance: u64,
    coin_type: String,
    use_gas_coin: bool,
}

impl CoinWithBalanceBuilder {
    pub fn new(balance: u64) -> Self {
        Self {
            balance,
            coin_type: SUI_TYPE.to_string(),
            use_gas_coin: true,
        }
    }

    pub fn coin_type(mut self, coin_type: String) -> Self {
        self.coin_type = coin_type;
        self
    }

    pub fn use_gas_coin(mut self, use: bool) -> Self {
        self.use_gas_coin = use;
        self
    }

    pub fn build(self) -> CoinWithBalance {
        CoinWithBalance {
            name: COIN_WITH_BALANCE.to_string(),
            coin_type: self.coin_type,
            balance: self.balance,
            use_gas_coin: self.use_gas_coin,
        }
    }
}

/// Builder function for CoinWithBalance
pub fn coin_with_balance(balance: u64) -> CoinWithBalanceBuilder {
    CoinWithBalanceBuilder::new(balance)
}

/// IntentResolver trait
pub trait IntentResolver {
    fn resolve(&self, intent: &CoinWithBalance) -> Result<(), Box<dyn std::error::Error>>;
}

/// CoinWithBalanceResolver
pub struct CoinWithBalanceResolver {
    sender: String,
}

impl CoinWithBalanceResolver {
    pub fn new(sender: String) -> Self {
        Self { sender }
    }
}

impl IntentResolver for CoinWithBalanceResolver {
    fn resolve(&self, intent: &CoinWithBalance) -> Result<(), Box<dyn std::error::Error>> {
        // Resolution logic here
        Ok(())
    }
}

/// IntentScope enum
#[repr(u8)]
pub enum IntentScope {
    TransactionData = 0,
    PersonalMessage = 1,
    Transaction = 3,
}