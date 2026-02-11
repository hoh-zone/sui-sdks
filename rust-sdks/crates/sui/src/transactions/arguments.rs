use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "$kind")]
pub enum Argument {
    #[allow(non_snake_case)]
    GasCoin {
        GasCoin: bool,
    },
    #[allow(non_snake_case)]
    Input {
        Input: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<String>,
    },
    #[allow(non_snake_case)]
    Result {
        Result: u32,
    },
    #[allow(non_snake_case)]
    NestedResult {
        NestedResult: (u32, u32, u32),
    },
    #[allow(non_snake_case)]
    Pure {
        Pure: Vec<u8>,
    },
}

impl Argument {
    pub fn gas_coin() -> Self {
        Argument::GasCoin { GasCoin: true }
    }

    pub fn input(index: u32) -> Self {
        Argument::Input {
            Input: index,
            r#type: None,
        }
    }

    pub fn result(index: u32) -> Self {
        Argument::Result { Result: index }
    }

    pub fn nested_result(index1: u32, index2: u32, index3: u32) -> Self {
        Argument::NestedResult {
            NestedResult: (index1, index2, index3),
        }
    }

    pub fn pure(bytes: Vec<u8>) -> Self {
        Argument::Pure { Pure: bytes }
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
        assert_eq!(
            arg,
            Argument::Input {
                Input: 5,
                r#type: None
            }
        );
    }

    #[test]
    fn test_argument_result() {
        let arg = Argument::result(3);
        assert_eq!(arg, Argument::Result { Result: 3 });
    }

    #[test]
    fn test_argument_nested_result() {
        let arg = Argument::nested_result(0, 1, 2);
        assert_eq!(
            arg,
            Argument::NestedResult {
                NestedResult: (0, 1, 2)
            }
        );
    }

    #[test]
    fn test_argument_pure() {
        let arg = Argument::pure(vec![1, 2, 3, 4]);
        assert_eq!(
            arg,
            Argument::Pure {
                Pure: vec![1, 2, 3, 4]
            }
        );
    }

    #[test]
    fn test_serialize_argument() {
        let arg = Argument::input(5);
        let serialized = serde_json::to_string(&arg).unwrap();
        assert!(serialized.contains("Input"));
    }

    #[test]
    fn test_deserialize_argument() {
        let json = r#"{"$kind":"Input","Input":5,"type":"pure"}"#;
        let arg: Argument = serde_json::from_str(json).unwrap();
        assert_eq!(
            arg,
            Argument::Input {
                Input: 5,
                r#type: Some("pure".to_string())
            }
        );
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
