package transactions

import (
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	"github.com/sui-sdks/go-sdks/stx"
)

type FlashLoanCommand struct {
	balanceManagerKey string
}

func NewFlashLoanContract(config *utils.DeepBookConfig) *FlashLoanContract {
	return &FlashLoanContract{config: config}
}

func (c *FlashLoanContract) CreateFlashLoan(tx *stx.Transaction, cmd FlashLoanCommand) {
	return tx.MoveCall(c.poolTarget("create_flash_loan"), []stx.Argument{
		tx.Object(c.config.DeepbookPackageID),
		tx.Object(balanceManager.Address),
		tx.Object("0x6"),
	})
}

func (c *FlashLoanContract) RepayFlashLoan(tx *stx.Transaction, cmd FlashLoanCommand) {
	return tx.MoveCall(c.poolTarget("repay_flash_loan"), []stx.Argument{
		tx.Object(c.config.DeepbookPackageID),
		tx.Object(balanceManager.Address),
		tx.Object("0x6"),
	})
}

func (c *FlashLoanContract) ClaimCollateral(tx *stx.Transaction, cmd FlashLoanCommand) {
	return tx.MoveCall(c.poolTarget("claim_collateral"), []stx.Argument{
		tx.Object(c.config.DeepbookPackageID),
		tx.Object(balanceManager.Address),
		tx.Object("0x6"),
	})
}

func (c *FlashLoanContract) CancelFlashLoan(tx *stx.Transaction, cmd FlashLoanCommand) {
	return tx.MoveCall(c.poolTarget("cancel_flash_loan"), []stx.Argument{
		tx.Object(c.config.DeepbookPackageID),
		tx.Object(balanceManager.Address),
		tx.Object("0x6"),
	})
}
