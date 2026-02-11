use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Argument {
    #[serde(rename = "GasCoin")]
    #[allow(non_snake_case)]
    GasCoin {
        GasCoin: bool,
    },
    Input(u32),
    Result(u32),
    NestedResult(u32, u32, u32),
    Pure(Vec<u8>),
}

impl Argument {
    pub fn gas_coin() -> Self {
        Argument::GasCoin { GasCoin: true }
    }

    pub fn input(index: u32) -> Self {
        Argument::Input(index)
    }

    pub fn result(index: u32) -> Self {
        Argument::Result(index)
    }

    pub fn nested_result(index1: u32, index2: u32, index3: u32) -> Self {
        Argument::NestedResult(index1, index2, index3)
    }

    pub fn pure(bytes: Vec<u8>) -> Self {
        Argument::Pure(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument_gas_coin() {
        let arg = Argument::gas_coin();
        match arg {
            Argument::GasCoin { .. } => (),
            _ => panic!("Expected GasCoin"),
        }
    }

    #[test]
    fn test_argument_input() {
        let arg = Argument::input(5);
        assert_eq!(arg, Argument::Input(5));
    }

    #[test]
    fn test_argument_result() {
        let arg = Argument::result(3);
        assert_eq!(arg, Argument::Result(3));
    }

    #[test]
    fn test_argument_nested_result() {
        let arg = Argument::nested_result(0, 1, 2);
        assert_eq!(arg, Argument::NestedResult(0, 1, 2));
    }

    #[test]
    fn test_argument_pure() {
        let arg = Argument::pure(vec![1, 2, 3, 4]);
        assert_eq!(arg, Argument::Pure(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_serialize_argument() {
        let arg = Argument::input(5);
        let serialized = serde_json::to_string(&arg).unwrap();
        assert!(serialized.contains("Input"));
    }

    #[test]
    fn test_deserialize_argument() {
        let json = r#"{"$kind":"Input","Input":5}"#;
        let arg: Argument = serde_json::from_str(json).unwrap();
        assert_eq!(arg, Argument::Input(5));
    }

    #[test]
    fn test_gas_coin_serialization() {
        let arg = Argument::gas_coin();
        let serialized = serde_json::to_string(&arg).unwrap();
        assert!(serialized.contains("GasCoin"));
        let deserialized: Argument = serde_json::from_str(&serialized).unwrap();
        match deserialized {
            Argument::GasCoin { .. } => (),
            _ => panic!("Expected GasCoin"),
        }
    }
}
