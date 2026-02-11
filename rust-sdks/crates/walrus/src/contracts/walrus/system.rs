use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct Contract<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> Contract<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::system::{}", self.package_id, function)
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

    pub fn invalidateBlobId(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "invalidate_blob_id", args, type_args)
    }

    pub fn certifyEventBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "certify_event_blob", args, type_args)
    }

    pub fn reserveSpace(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "reserve_space", args, type_args)
    }

    pub fn reserveSpaceForEpochs(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "reserve_space_for_epochs", args, type_args)
    }

    pub fn registerBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "register_blob", args, type_args)
    }

    pub fn certifyBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "certify_blob", args, type_args)
    }

    pub fn deleteBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "delete_blob", args, type_args)
    }

    pub fn extendBlobWithResource(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "extend_blob_with_resource", args, type_args)
    }

    pub fn extendBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "extend_blob", args, type_args)
    }

    pub fn addSubsidy(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_subsidy", args, type_args)
    }

    pub fn addPerEpochSubsidies(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "add_per_epoch_subsidies", args, type_args)
    }

    pub fn updateProtocolVersion(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "update_protocol_version", args, type_args)
    }

    pub fn registerDenyListUpdate(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "register_deny_list_update", args, type_args)
    }

    pub fn updateDenyList(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "update_deny_list", args, type_args)
    }

    pub fn deleteDenyListedBlob(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "delete_deny_listed_blob", args, type_args)
    }

    pub fn epoch(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "epoch", args, type_args)
    }

    pub fn totalCapacitySize(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "total_capacity_size", args, type_args)
    }

    pub fn usedCapacitySize(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "used_capacity_size", args, type_args)
    }

    pub fn nShards(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "n_shards", args, type_args)
    }

    pub fn futureAccounting(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "future_accounting", args, type_args)
    }

    pub fn version(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "version", args, type_args)
    }
}
