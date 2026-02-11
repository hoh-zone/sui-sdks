use serde::{Deserialize, Serialize};

use super::arguments::Argument;
use super::normalize_sui_address;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$kind")]
pub enum CommandKind {
    MoveCall(MoveCall),
    TransferObjects(TransferObjects),
    SplitCoins(SplitCoins),
    MergeCoins(MergeCoins),
    Publish(Publish),
    Upgrade(Upgrade),
    MakeMoveVec(MakeMoveVec),
    Intent(Intent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCall {
    #[serde(rename = "package")]
    pub package: String,
    pub module: String,
    pub function: String,
    #[serde(rename = "typeArguments")]
    pub type_arguments: Vec<String>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferObjects {
    pub objects: Vec<Argument>,
    pub address: Argument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitCoins {
    pub coin: Argument,
    pub amounts: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCoins {
    pub destination: Argument,
    pub sources: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publish {
    pub modules: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub modules: Vec<String>,
    pub dependencies: Vec<String>,
    #[serde(rename = "package")]
    pub package: String,
    pub ticket: Argument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeMoveVec {
    #[serde(rename = "type")]
    pub type_arg: Option<String>,
    pub elements: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub name: String,
    pub inputs: serde_json::Value,
    pub data: serde_json::Value,
}

#[repr(u8)]
pub enum UpgradePolicy {
    Compatible = 0,
    Additive = 128,
    DepOnly = 192,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    #[serde(flatten)]
    pub kind: CommandKind,
}

pub struct TransactionCommands;

impl TransactionCommands {
    pub fn move_call(
        target: &str,
        arguments: Vec<Argument>,
        type_arguments: Vec<String>,
    ) -> Command {
        let parts: Vec<&str> = target.split("::").collect();
        let (package, module, function) = match parts.as_slice() {
            [p, m, f] => (*p, *m, *f),
            [p, m] => (*p, *m, ""),
            [p] => (*p, "", ""),
            _ => ("", "", ""),
        };
        Command {
            kind: CommandKind::MoveCall(MoveCall {
                package: package.to_string(),
                module: module.to_string(),
                function: function.to_string(),
                type_arguments,
                arguments,
            }),
        }
    }

    pub fn transfer_objects(objects: Vec<Argument>, address: Argument) -> Command {
        Command {
            kind: CommandKind::TransferObjects(TransferObjects { objects, address }),
        }
    }

    pub fn split_coins(coin: Argument, amounts: Vec<Argument>) -> Command {
        Command {
            kind: CommandKind::SplitCoins(SplitCoins { coin, amounts }),
        }
    }

    pub fn merge_coins(destination: Argument, sources: Vec<Argument>) -> Command {
        Command {
            kind: CommandKind::MergeCoins(MergeCoins {
                destination,
                sources,
            }),
        }
    }

    pub fn publish(modules: Vec<Vec<u8>>, dependencies: Vec<String>) -> Command {
        use base64::Engine as _;
        let encoded_modules = modules
            .iter()
            .map(|m| base64::engine::general_purpose::STANDARD.encode(m))
            .collect();
        Command {
            kind: CommandKind::Publish(Publish {
                modules: encoded_modules,
                dependencies: dependencies
                    .into_iter()
                    .map(|s| normalize_sui_address(&s))
                    .collect(),
            }),
        }
    }

    pub fn upgrade(
        modules: Vec<Vec<u8>>,
        dependencies: Vec<String>,
        package_id: String,
        ticket: Argument,
    ) -> Command {
        use base64::Engine as _;
        let encoded_modules = modules
            .iter()
            .map(|m| base64::engine::general_purpose::STANDARD.encode(m))
            .collect();
        Command {
            kind: CommandKind::Upgrade(Upgrade {
                modules: encoded_modules,
                dependencies: dependencies
                    .into_iter()
                    .map(|s| normalize_sui_address(&s))
                    .collect(),
                package: normalize_sui_address(&package_id),
                ticket,
            }),
        }
    }

    pub fn make_move_vec(type_arg: Option<String>, elements: Vec<Argument>) -> Command {
        Command {
            kind: CommandKind::MakeMoveVec(MakeMoveVec { type_arg, elements }),
        }
    }

    pub fn intent(name: String, inputs: serde_json::Value, data: serde_json::Value) -> Command {
        Command {
            kind: CommandKind::Intent(Intent { name, inputs, data }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_call_command() {
        let cmd = TransactionCommands::move_call(
            "0x2::coin::transfer",
            vec![],
            vec!["0x2::coin::Coin".to_string()],
        );
        match cmd.kind {
            CommandKind::MoveCall(move_call) => {
                assert_eq!(move_call.package, "0x2");
                assert_eq!(move_call.module, "coin");
                assert_eq!(move_call.function, "transfer");
            }
            _ => panic!("Expected MoveCall"),
        }
    }

    #[test]
    fn test_transfer_objects_command() {
        let address = Argument::input(0);
        let cmd = TransactionCommands::transfer_objects(vec![], address.clone());
        match cmd.kind {
            CommandKind::TransferObjects(_) => (),
            _ => panic!("Expected TransferObjects"),
        }
    }

    #[test]
    fn test_split_coins_command() {
        let coin = Argument::input(0);
        let amounts = vec![Argument::pure(vec![1, 2, 3, 4, 5, 6, 7, 8])];
        let cmd = TransactionCommands::split_coins(coin.clone(), amounts);
        match cmd.kind {
            CommandKind::SplitCoins(_) => (),
            _ => panic!("Expected SplitCoins"),
        }
    }

    #[test]
    fn test_serialize_command() {
        let cmd = TransactionCommands::move_call("0x2::coin::transfer", vec![], vec![]);
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains("MoveCall"));
    }
}
