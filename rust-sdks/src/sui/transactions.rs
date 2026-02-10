use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::crypto;

use super::jsonrpc;
use super::keypairs::{ed25519, secp256k1, secp256r1};

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("serialize transaction failed: {0}")]
    Serialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GasData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Value>,
    #[serde(default, rename = "gasData")]
    pub gas_data: GasData,
    #[serde(default)]
    pub inputs: Vec<Value>,
    #[serde(default)]
    pub commands: Vec<Value>,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub data: TransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx_bytes_base64: String,
    pub signatures: Vec<String>,
}

impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_sender(&mut self, sender: impl Into<String>) {
        self.data.sender = Some(sender.into());
    }

    pub fn set_sender_if_not_set(&mut self, sender: impl Into<String>) {
        if self.data.sender.is_none() {
            self.set_sender(sender);
        }
    }

    pub fn set_gas_budget(&mut self, budget: u64) {
        self.data.gas_data.budget = Some(budget.to_string());
    }

    pub fn set_gas_price(&mut self, price: u64) {
        self.data.gas_data.price = Some(price.to_string());
    }

    pub fn set_gas_owner(&mut self, owner: impl Into<String>) {
        self.data.gas_data.owner = Some(owner.into());
    }

    pub fn set_gas_payment(&mut self, payment: Vec<Value>) {
        self.data.gas_data.payment = Some(payment);
    }

    pub fn set_expiration(&mut self, expiration: Value) {
        self.data.expiration = Some(expiration);
    }

    pub fn gas() -> Value {
        json!({"$kind":"GasCoin","GasCoin":true})
    }

    pub fn add_input(&mut self, input: Value) -> Value {
        self.data.inputs.push(input);
        json!({"$kind":"Input","Input": self.data.inputs.len() - 1})
    }

    pub fn object(&mut self, object_id: impl Into<String>) -> Value {
        self.add_input(json!({
            "$kind":"UnresolvedObject",
            "UnresolvedObject":{"objectId": object_id.into()}
        }))
    }

    pub fn pure_bytes(&mut self, bytes: &[u8]) -> Value {
        use base64::Engine as _;
        self.add_input(json!({
            "$kind":"Pure",
            "Pure":{"bytes": base64::engine::general_purpose::STANDARD.encode(bytes)}
        }))
    }

    pub fn add_command(&mut self, command: Value) -> Value {
        self.data.commands.push(command);
        json!({"$kind":"Result","Result": self.data.commands.len() - 1})
    }

    pub fn move_call(
        &mut self,
        target: &str,
        arguments: Vec<Value>,
        type_arguments: Vec<String>,
    ) -> Value {
        let mut parts = target.split("::");
        let package = parts.next().unwrap_or_default();
        let module = parts.next().unwrap_or_default();
        let function = parts.next().unwrap_or_default();
        self.add_command(json!({
            "$kind":"MoveCall",
            "MoveCall":{
                "package": package,
                "module": module,
                "function": function,
                "arguments": arguments,
                "typeArguments": type_arguments
            }
        }))
    }

    pub fn transfer_objects(&mut self, objects: Vec<Value>, address: Value) -> Value {
        self.add_command(json!({
            "$kind":"TransferObjects",
            "TransferObjects":{"objects": objects, "address": address}
        }))
    }

    pub fn split_coins(&mut self, coin: Value, amounts: Vec<Value>) -> Value {
        self.add_command(json!({
            "$kind":"SplitCoins",
            "SplitCoins":{"coin": coin, "amounts": amounts}
        }))
    }

    pub fn merge_coins(&mut self, destination: Value, sources: Vec<Value>) -> Value {
        self.add_command(json!({
            "$kind":"MergeCoins",
            "MergeCoins":{"destination": destination, "sources": sources}
        }))
    }

    pub fn build(&self) -> Result<Vec<u8>, TransactionError> {
        Ok(serde_json::to_vec(&self.data)?)
    }

    pub fn build_base64(&self) -> Result<String, TransactionError> {
        use base64::Engine as _;
        Ok(base64::engine::general_purpose::STANDARD.encode(self.build()?))
    }

    pub fn sign_with_ed25519(&self, keypair: &ed25519::Keypair) -> Result<SignedTransaction, TransactionError> {
        self.sign_with_signer(0x00, |msg| keypair.sign(msg), keypair.public_key_bytes())
    }

    pub fn sign_with_secp256k1(
        &self,
        keypair: &secp256k1::Keypair,
    ) -> Result<SignedTransaction, TransactionError> {
        self.sign_with_signer(0x01, |msg| keypair.sign(msg), keypair.public_key_bytes())
    }

    pub fn sign_with_secp256r1(
        &self,
        keypair: &secp256r1::Keypair,
    ) -> Result<SignedTransaction, TransactionError> {
        self.sign_with_signer(0x02, |msg| keypair.sign(msg), keypair.public_key_bytes())
    }

    fn sign_with_signer<F>(
        &self,
        scheme_flag: u8,
        signer: F,
        public_key: Vec<u8>,
    ) -> Result<SignedTransaction, TransactionError>
    where
        F: FnOnce(&[u8]) -> Vec<u8>,
    {
        let tx_bytes = self.build()?;
        let msg = crypto::message_with_intent([0, 0, 0], &tx_bytes);
        let signature = signer(&msg);
        let serialized_signature = serialize_signature(scheme_flag, &signature, &public_key);

        use base64::Engine as _;
        Ok(SignedTransaction {
            tx_bytes_base64: base64::engine::general_purpose::STANDARD.encode(tx_bytes),
            signatures: vec![serialized_signature],
        })
    }
}

impl SignedTransaction {
    pub async fn execute(
        &self,
        client: &jsonrpc::Client,
        options: Option<Value>,
        request_type: Option<&str>,
    ) -> Result<Value, jsonrpc::JsonRpcError> {
        client
            .execute_transaction_block(
                &self.tx_bytes_base64,
                self.signatures.clone(),
                options,
                request_type,
            )
            .await
    }
}

fn serialize_signature(scheme_flag: u8, signature: &[u8], public_key: &[u8]) -> String {
    let mut bytes = Vec::with_capacity(1 + signature.len() + public_key.len());
    bytes.push(scheme_flag);
    bytes.extend_from_slice(signature);
    bytes.extend_from_slice(public_key);
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}
