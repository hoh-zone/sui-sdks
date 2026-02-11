use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct Contract<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> Contract<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::storage_accounting::{}", self.package_id, function)
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

    pub fn maxEpochsAhead(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "max_epochs_ahead", args, type_args)
    }

    pub fn ringLookup(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "ring_lookup", args, type_args)
    }

    pub fn epoch(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "epoch", args, type_args)
    }

    pub fn usedCapacity(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "used_capacity", args, type_args)
    }

    pub fn rewards(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "rewards", args, type_args)
    }
}
