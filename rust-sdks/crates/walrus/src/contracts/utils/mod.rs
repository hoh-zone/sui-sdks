use sui::transactions::Transaction;

pub fn move_target(package_id: &str, module: &str, function: &str) -> String {
    format!("{}::{}::{}", package_id, module, function)
}

pub fn move_call(
    tx: &mut Transaction,
    package_id: &str,
    module: &str,
    function: &str,
    args: Vec<serde_json::Value>,
    type_args: Vec<String>,
) -> serde_json::Value {
    tx.move_call(&move_target(package_id, module, function), args, type_args)
}
