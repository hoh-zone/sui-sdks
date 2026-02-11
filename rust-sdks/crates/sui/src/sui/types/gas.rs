#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasCost {
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
    pub non_refundable_storage_fee: u64,
}

impl GasCost {
    pub fn new(
        computation_cost: u64,
        storage_cost: u64,
        storage_rebate: u64,
        non_refundable_storage_fee: u64,
    ) -> Self {
        Self {
            computation_cost,
            storage_cost,
            storage_rebate,
            non_refundable_storage_fee,
        }
    }

    pub fn total_gas(&self) -> u64 {
        self.computation_cost + self.storage_cost - self.storage_rebate
    }

    pub fn net_gas_price(&self) -> Option<u64> {
        if self.storage_rebate > self.storage_cost {
            Some(self.computation_cost)
        } else {
            Some(self.computation_cost + self.storage_cost)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasUsed {
    pub computation_cost: u64,
    pub storage_cost: u64,
    pub storage_rebate: u64,
    pub non_refundable_storage_fee: u64,
}

impl GasUsed {
    pub fn total(&self) -> u64 {
        self.computation_cost + self.storage_cost
            - self.storage_rebate
            - self.non_refundable_storage_fee
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasPrice {
    pub value: u64,
}

impl GasPrice {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }
}

impl From<u64> for GasPrice {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<GasPrice> for u64 {
    fn from(gas_price: GasPrice) -> Self {
        gas_price.value
    }
}

#[derive(Debug, Clone)]
pub struct GasBalance {
    pub total_balance: u64,
    pub gas_objects: Vec<GasObject>,
}

#[derive(Debug, Clone)]
pub struct GasObject {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
    pub balance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_cost() {
        let gas_cost = GasCost::new(100, 50, 20, 10);
        assert_eq!(gas_cost.total_gas(), 130);
    }

    #[test]
    fn test_gas_cost_zero_rebate() {
        let gas_cost = GasCost::new(100, 50, 0, 10);
        assert_eq!(gas_cost.total_gas(), 150);
    }

    #[test]
    fn test_gas_used_total() {
        let gas_used = GasUsed {
            computation_cost: 1000,
            storage_cost: 500,
            storage_rebate: 200,
            non_refundable_storage_fee: 50,
        };
        assert_eq!(gas_used.total(), 1250);
    }

    #[test]
    fn test_gas_price() {
        let gas_price = GasPrice::new(1000);
        assert_eq!(gas_price.as_u64(), 1000);
    }

    #[test]
    fn test_gas_price_from() {
        let gas_price: GasPrice = 2000.into();
        assert_eq!(gas_price.as_u64(), 2000);

        let u64_val: u64 = gas_price.into();
        assert_eq!(u64_val, 2000);
    }
}
