use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct Contract<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> Contract<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::storage_resource::{}", self.package_id, function)
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

    pub fn startEpoch(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "start_epoch", args, type_args)
    }

    pub fn endEpoch(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "end_epoch", args, type_args)
    }

    pub fn size(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "size", args, type_args)
    }

    pub fn splitByEpoch(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "split_by_epoch", args, type_args)
    }

    pub fn splitBySize(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "split_by_size", args, type_args)
    }

    pub fn fusePeriods(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "fuse_periods", args, type_args)
    }

    pub fn fuseAmount(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "fuse_amount", args, type_args)
    }

    pub fn fuse(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "fuse", args, type_args)
    }

    pub fn destroy(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "destroy", args, type_args)
    }
}
