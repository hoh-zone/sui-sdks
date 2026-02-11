// Transaction Executor for Rust SDK
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// TransactionResult
#[derive(Debug, Clone)]
pub struct TransactionResult {
    pub success: bool,
    pub error: Option<String>,
}

impl TransactionResult {
    pub fn new_success() -> Self {
        Self { success: true, error: None }
    }
}

/// Transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub commands: Vec<Command>,
    pub sender: Option<String>,
    pub gas_price: Option<u64>,
}

/// Command
#[derive(Debug, Clone)]
pub struct Command {
    pub command_type: String,
    pub data: Option<serde_json::Value>,
}

/// SerialTransactionExecutor
pub struct SerialTransactionExecutor {
    executor: Arc<Mutex<TransactionExecutor>>,
}

impl SerialTransactionExecutor {
    pub fn new() -> Self {
        Self {
            executor: Arc::new(Mutex::new(TransactionExecutor::new())),
        }
    }

    pub async fn execute(&self, tx: Transaction) -> TransactionResult {
        let executor = self.executor.lock().unwrap();
        executor.execute(tx)
    }
}

/// ParallelTransactionExecutor
pub struct ParallelTransactionExecutor {
    executor: Arc<Mutex<TransactionExecutor>>,
    workers: usize,
}

impl ParallelTransactionExecutor {
    pub fn new(workers: usize) -> Self {
        Self {
            executor: Arc::new(Mutex::new(TransactionExecutor::new())),
            workers,
        }
    }
}

/// TransactionExecutor
pub struct TransactionExecutor {
    queue: Vec<Transaction>,
}

impl TransactionExecutor {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn execute(&self, tx: Transaction) -> TransactionResult {
        TransactionResult::new_success()
    }
}

/// ObjectCache
#[derive(Clone)]
pub struct ObjectCache {
    cache: Arc<Mutex<HashMap<String, Object>>>,
}

impl ObjectCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, id: &str) -> Option<Object> {
        let cache = self.cache.lock().unwrap();
        cache.get(id).cloned()
    }

    pub fn set(&self, id: String, obj: Object) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(id, obj);
    }

    pub fn delete(&self, id: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(id);
    }
}

/// Object
#[derive(Debug, Clone)]
pub struct Object {
    pub id: String,
    pub object_type: String,
    pub data: Vec<u8>,
    pub digest: String,
}