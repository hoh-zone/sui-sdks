use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct Contract<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> Contract<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::committee::{}", self.package_id, function)
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

    pub fn shards(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "shards", args, type_args)
    }

    pub fn size(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "size", args, type_args)
    }

    pub fn inner(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "inner", args, type_args)
    }

    pub fn toInner(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "to_inner", args, type_args)
    }
}
