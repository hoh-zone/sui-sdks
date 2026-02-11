# DeepBook TS-Go Conformance

## Status

- P0 compile blockers fixed.
- P1 API parity methods added for missing DeepBook v3 surfaces.
- P2 conformance tests expanded with method-target matrix for newly added parity methods.

## P0 Completed

- Removed broken duplicate flash-loan implementation file.
- Fixed package import path issues (`stx` -> `sui/transactions` aliases).
- Reworked `GovernanceContract` to a compilable implementation aligned with TS contract shape.
- Resolved type name collision in `types` (`OrderType` duplicate) by renaming order-query enum.
- Verified package builds and tests pass via:
  - `go test ./deepbook_v3/...`

## P1 Completed (TS parity additions)

- `DeepBookAdminContract.AdjustMinLotSize`
- `DeepBookContract.CreatePermissionlessPool`
- `MarginManagerContract.DepositDeep`
- `MarginRegistryContract.GetDeepbookPoolMarginPoolIDs`
- `PoolProxyContract.WithdrawMarginSettledAmounts`
- Governance parity surface:
  - `GovernanceContract.Stake`
  - `GovernanceContract.Unstake`
  - `GovernanceContract.SubmitProposal`
  - `GovernanceContract.Vote`
- Margin admin/maintainer expanded with TS-style config and registry methods (add/remove config, version toggles, pause-cap flows, margin-pool config builders, and with-cap variants).

## P2 Completed (tests and matrix)

- Added conformance tests:
  - `deepbook_v3/transactions/conformance_test.go`
- New tests assert Move target mapping for:
  - Governance parity methods
  - New parity methods listed above
- Existing encoding and contract target suites remain green.

## Remaining Optional Work

- Add fixture-based byte-level cross-SDK assertions generated from TS runtime (same target/type args/pure bytes) once TS workspace runtime dependencies are available.
