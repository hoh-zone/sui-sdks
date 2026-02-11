package transactions

import (
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	"github.com/sui-sdks/go-sdks/stx"
)

type GovernanceContract struct {
	config *utils.DeepBookConfig
}

type GovernanceCommand struct {
	proposalId string
	approval   uint8
}

func NewGovernanceContract(config *utils.DeepBookConfig) *GovernanceContract {
	return &GovernanceContract{config: config}
}

func (c *GovernanceContract) CreateProposal(tx *stx.Transaction, cmd GovernanceCommand) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("create_proposal"), []stx.Argument{
		tx.Object("0x6"),
		tx.PureBytes([]byte(cmd.ProposalId)),
	})
}

func (c *GovernanceContract) ApproveProposal(tx *stx.Transaction, cmd GovernanceCommand) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("approve_proposal"), []stx.Argument{
		tx.Object("0x6"),
		tx.PureBytes([]byte(cmd.ProposalId), tx.PureU8(cmd.Approval)),
	})
}

func (c *GovernanceContract) ExecuteProposal(tx *stx.Transaction, cmd GovernanceCommand) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("execute_proposal"), []stx.Argument{
		tx.Object("0x6"),
		tx.PureBytes([]byte(cmd.ProposalId)),
	})
}

func (c *GovernanceContract) VoteProposal(tx *stx.Transaction, cmd GovernanceCommand) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("vote_proposal"), []stx.Argument{
		tx.Object("0x6"),
		tx.PureBytes([]byte(cmd.ProposalId)),
	})
}

func (c *GovernanceContract) CancelProposal(tx *stx.Transaction, cmd GovernanceCommand) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("cancel_proposal"), []stx.Argument{
		tx.Object("0x6"),
		tx.PureBytes([]byte(cmd.ProposalId)),
	})
}

func (c *GovernanceContract) SetAdminCap(tx *stx.Transaction, adminCap string) {
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	return tx.MoveCall(c.poolTarget("set_admin_cap"), []stx.Argument{
		tx.Object("0x6"),
		tx.Object("0x2::coin::Coin<0x2::coin::Coin>", adminCap),
	})
}
