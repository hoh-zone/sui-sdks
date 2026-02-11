use sui::transactions::Transaction;

#[derive(Debug, Clone, Copy)]
pub struct Contract<'a> {
    pub package_id: &'a str,
}

#[allow(non_snake_case)]
impl<'a> Contract<'a> {
    pub fn target(&self, function: &str) -> String {
        format!("{}::upgrade::{}", self.package_id, function)
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

    pub fn voteForUpgrade(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "vote_for_upgrade", args, type_args)
    }

    pub fn authorizeUpgrade(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "authorize_upgrade", args, type_args)
    }

    pub fn authorizeEmergencyUpgrade(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "authorize_emergency_upgrade", args, type_args)
    }

    pub fn commitUpgrade(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "commit_upgrade", args, type_args)
    }

    pub fn cleanupUpgradeProposals(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "cleanup_upgrade_proposals", args, type_args)
    }

    pub fn burnEmergencyUpgradeCap(
        &self,
        tx: &mut Transaction,
        args: Vec<serde_json::Value>,
        type_args: Vec<String>,
    ) -> serde_json::Value {
        self.move_call(tx, "burn_emergency_upgrade_cap", args, type_args)
    }
}
