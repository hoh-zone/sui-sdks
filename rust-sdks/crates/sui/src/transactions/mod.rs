pub mod arguments;
pub mod builder;
pub mod commands;
pub mod hash;
pub mod inputs;
pub mod object;
pub mod object_cache;
pub mod plugins;
pub mod pure;
pub mod resolve;
pub mod serializer;
pub mod types;
pub mod utils;

pub use arguments::Argument;
pub use commands::{Command, CommandKind, TransactionCommands};
pub use inputs::{ObjectKind, CallArg, ObjectRef, SharedObjectRef, TransactionInput};
pub use object::SuiObject;
pub use pure::PureValue;
pub use types::{GasData, SignedTransaction, Transaction, TransactionData, TransactionError};

pub const SUI_ADDRESS_LENGTH: usize = 32;

pub fn normalize_sui_address(addr: &str) -> String {
    let mut raw = addr.trim().to_lowercase();
    if let Some(stripped) = raw.strip_prefix("0x") {
        raw = stripped.to_string();
    }

    let max_len = SUI_ADDRESS_LENGTH * 2;
    if raw.len() > max_len {
        raw = raw[raw.len() - max_len..].to_string();
    }
    if raw.len() < max_len {
        raw = format!("{}{}", "0".repeat(max_len - raw.len()), raw);
    }
    format!("0x{raw}")
}

pub fn validate_sui_address(addr: &str) -> bool {
    let addr = addr.trim();
    if addr.is_empty() || addr.len() > SUI_ADDRESS_LENGTH * 2 + 2 {
        return false;
    }
    let raw = addr.strip_prefix("0x").unwrap_or(addr);
    hex::decode(raw).is_ok()
}