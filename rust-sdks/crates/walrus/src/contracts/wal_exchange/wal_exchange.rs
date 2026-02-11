use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct WalExchange<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> WalExchange<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::wal_exchange::{}", self.package_id, function)
    }

    pub fn move_call(
        &self,
        tx: &mut Transaction,
        function: &str,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        tx.move_call(&self.target(function), args, type_args)
    }

    pub fn newExchangeRate(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "new_exchange_rate", args, type_args)
    }

    pub fn _new(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "new", args, type_args)
    }

    pub fn newFunded(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "new_funded", args, type_args)
    }

    pub fn addWal(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_wal", args, type_args)
    }

    pub fn addSui(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_sui", args, type_args)
    }

    pub fn addAllWal(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_all_wal", args, type_args)
    }

    pub fn addAllSui(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_all_sui", args, type_args)
    }

    pub fn withdrawWal(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "withdraw_wal", args, type_args)
    }

    pub fn withdrawSui(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "withdraw_sui", args, type_args)
    }

    pub fn setExchangeRate(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "set_exchange_rate", args, type_args)
    }

    pub fn exchangeAllForWal(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "exchange_all_for_wal", args, type_args)
    }

    pub fn exchangeForWal(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "exchange_for_wal", args, type_args)
    }

    pub fn exchangeAllForSui(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "exchange_all_for_sui", args, type_args)
    }

    pub fn exchangeForSui(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "exchange_for_sui", args, type_args)
    }
}
