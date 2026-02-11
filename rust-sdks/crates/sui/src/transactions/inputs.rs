use serde::{Deserialize, Serialize};

use super::arguments::Argument;
use super::pure::PureValue;

fn normalize_sui_address(addr: &str) -> String {
    let mut raw = addr.trim().to_lowercase();
    if let Some(stripped) = raw.strip_prefix("0x") {
        raw = stripped.to_string();
    }
    let max_len = 32 * 2;
    if raw.len() > max_len {
        raw = raw[raw.len() - max_len..].to_string();
    }
    if raw.len() < max_len {
        raw = format!("{}{}", "0".repeat(max_len - raw.len()), raw);
    }
    format!("0x{raw}")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$kind")]
pub enum CallArg {
    Pure(Pure),
    Object(ObjectKind),
    FundsWithdrawal(FundsWithdrawal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pure {
    pub bytes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$kind")]
pub enum ObjectKind {
    ImmOrOwnedObject(ImmOrOwnedObject),
    SharedObject(SharedObject),
    Receiving(ReceivingRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmOrOwnedObject {
    pub digest: String,
    pub version: u64,
    #[serde(rename = "objectId")]
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedObject {
    pub mutable: bool,
    #[serde(rename = "initialSharedVersion")]
    pub initial_shared_version: u64,
    #[serde(rename = "objectId")]
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivingRef {
    pub digest: String,
    pub version: u64,
    #[serde(rename = "objectId")]
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRef {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub digest: String,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedObjectRef {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub mutable: bool,
    #[serde(rename = "initialSharedVersion")]
    pub initial_shared_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundsWithdrawal {
    pub reservation: Reservation,
    #[serde(rename = "typeArg")]
    pub type_arg: String,
    #[serde(rename = "withdrawFrom")]
    pub withdraw_from: WithdrawFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    #[serde(rename = "reservationNumber")]
    pub reservation_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$kind")]
pub enum WithdrawFrom {
    CallArg(Argument),
    #[serde(rename = "$Intent")]
    Intent(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$kind")]
pub enum TransactionInput {
    Call(CallArg),
    Pure(PureValue),
}

impl CallArg {
    pub fn pure(bytes: Vec<u8>) -> Self {
        use base64::Engine as _;
        CallArg::Pure(Pure {
            bytes: base64::engine::general_purpose::STANDARD.encode(bytes),
        })
    }

    pub fn object_ref(object_ref: ObjectRef) -> Self {
        CallArg::Object(ObjectKind::ImmOrOwnedObject(ImmOrOwnedObject {
            digest: object_ref.digest,
            version: object_ref.version,
            object_id: normalize_sui_address(&object_ref.object_id),
        }))
    }

    pub fn shared_object_ref(shared_ref: SharedObjectRef) -> Self {
        CallArg::Object(ObjectKind::SharedObject(SharedObject {
            mutable: shared_ref.mutable,
            initial_shared_version: shared_ref.initial_shared_version,
            object_id: normalize_sui_address(&shared_ref.object_id),
        }))
    }

    pub fn receiving_ref(receiving_ref: ReceivingRef) -> Self {
        CallArg::Object(ObjectKind::Receiving(receiving_ref))
    }

    pub fn funds_withdrawal(
        reservation: Reservation,
        type_arg: String,
        withdraw_from: WithdrawFrom,
    ) -> Self {
        CallArg::FundsWithdrawal(FundsWithdrawal {
            reservation,
            type_arg,
            withdraw_from,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_ref_creation() {
        let object_ref = ObjectRef {
            object_id: "0x1".to_string(),
            digest: "digest".to_string(),
            version: 1,
        };
        let call_arg = CallArg::object_ref(object_ref);
        match call_arg {
            CallArg::Object(ObjectKind::ImmOrOwnedObject(obj)) => {
                assert_eq!(
                    obj.object_id,
                    "0x0000000000000000000000000000000000000000000000000000000000000001"
                );
            }
            _ => panic!("Expected ImmOrOwnedObject"),
        }
    }

    #[test]
    fn test_shared_object_ref_creation() {
        let shared_ref = SharedObjectRef {
            object_id: "0x2".to_string(),
            mutable: true,
            initial_shared_version: 1,
        };
        let call_arg = CallArg::shared_object_ref(shared_ref);
        match call_arg {
            CallArg::Object(ObjectKind::SharedObject(obj)) => {
                assert!(obj.mutable);
                assert_eq!(obj.initial_shared_version, 1);
            }
            _ => panic!("Expected SharedObject"),
        }
    }

    #[test]
    fn test_pure_call_arg() {
        let call_arg = CallArg::pure(vec![1, 2, 3, 4]);
        match call_arg {
            CallArg::Pure(_) => (),
            _ => panic!("Expected Pure"),
        }
    }

    #[test]
    fn test_serialize_call_arg() {
        let call_arg = CallArg::pure(vec![1, 2, 3, 4]);
        let serialized = serde_json::to_string(&call_arg).unwrap();
        assert!(serialized.contains("Pure"));
    }

    #[test]
    fn test_deserialize_call_arg() {
        let json = r#"{"$kind":"Pure","bytes":"AQIDBA=="}"#;
        let call_arg: CallArg = serde_json::from_str(json).unwrap();
        match call_arg {
            CallArg::Pure(_) => (),
            _ => panic!("Expected Pure"),
        }
    }
}
