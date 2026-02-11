# DeepBook V3 Rust SDK

This is the Rust implementation of the DeepBook V3 SDK for Sui blockchain.

## Implemented Modules

### P0 - Core Features

1. **Governance** (`src/contracts/governance.rs`)
   - `stake` - Stake in the pool
   - `unstake` - Unstake from the pool
   - `submit_proposal` - Submit governance proposal
   - `vote` - Vote on proposals

2. **TPSL** (`src/contracts/tpsl.rs`)
   - `new_condition` - Create condition for conditional order
   - `new_pending_limit_order` - Create pending limit order
   - `new_pending_market_order` - Create pending market order
   - `add_conditional_order` - Add conditional order (simplified)
   - `cancel_conditional_order` - Cancel a conditional order
   - `cancel_all_conditional_orders` - Cancel all conditional orders
   - `execute_conditional_orders` - Execute triggered orders
   - Read-only queries for conditional order data

3. **FlashLoans** (`src/contracts/flash_loans.rs`)
   - `borrow_base_asset` - Borrow base asset
   - `return_base_asset` - Return base asset
   - `borrow_quote_asset` - Borrow quote asset
   - `return_quote_asset` - Return quote asset

### P1 - Additional Features

4. **MarginAdmin** (`src/contracts/margin_admin.rs`)
   - `mint_maintainer_cap` - Mint maintainer capability
   - `register_deepbook_pool` - Register a pool
   - `enable_deepbook_pool` - Enable margin trading for pool
   - `disable_deepbook_pool` - Disable margin trading for pool
   - `update_risk_params` - Update risk parameters

5. **DeepBookAdmin** (`src/contracts/deepbook_admin.rs`)
   - `create_pool_admin` - Create pool as admin
   - `unregister_pool_admin` - Unregister pool
   - `update_allowed_versions` - Update allowed versions
   - `enable_version` - Enable a version
   - `disable_version` - Disable a version

## API Design

All contract methods return a tuple `(target, args, type_args)` where:
- `target`: The Move function target (e.g., `0x22be...::pool::stake`)
- `args`: Vector of arguments for the Move call
- `type_args`: Vector of type arguments (coin types)

Example usage:
```rust
use deepbook_v3::governance::GovernanceContract;

let config = DeepBookConfig::default();
let governance = GovernanceContract { config: &config };

let (target, args, type_args) = governance.stake(tx, "DEEP_SUI", "manager1", 100.0)?;
```

## Running Tests

```bash
cargo test --lib
```

## Notes

- Simplified implementations for some complex functions avoid Rust borrowing rules
- All modules compile successfully with passing tests
- Full parity with TypeScript SDK pending additional work on Move contract integration