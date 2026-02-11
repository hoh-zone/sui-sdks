package transactions

import (
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	"github.com/sui-sdks/go-sdks/stx"
)

type MarginTPSLContract struct {
	config *utils.DeepBookConfig
}

type MarginTPSLCommand struct {
	poolKey string
}

func NewMarginTPSLContract(config *utils.DeepBookConfig) *MarginTPSLContract {
	return &MarginTPSLContract{config: config}
}

func (c *MarginTPSLContract) SetTPSLParams(tx *stx.Transaction, poolKey, cmd MarginTPSLCommand) {
	pool := c.config.GetPool(poolKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	bidPrice := uint64(math.Round(cmd.BidPrice * pool.QuoteCoin.Scalar))
	askPrice := uint64(math.Round(cmd.AskPrice * pool.QuoteCoin.Scalar))
	bidTickSize := uint32(cmd.BidTickSize)
	askTickSize := uint32(cmd.AskTickSize)

	return tx.MoveCall(c.poolTarget("set_tpsl_params"), []stx.Argument{
		tx.Object(pool.Address),
		pureU64(tx, bidPrice),
		pureU64(tx, askPrice),
		pureU8(tx, bidTickSize),
		pureU8(tx, askTickSize),
	})
}

func (c *MarginTPSLContract) UpdateTPSLParams(tx *stx.Transaction, poolKey, cmd MarginTPSLCommand) {
	pool := c.config.GetPool(poolKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	bidPrice := uint64(math.Round(cmd.BidPrice * pool.QuoteCoin.Scalar))
	askPrice := uint64(math.Round(cmd.AskPrice * pool.QuoteCoin.Scalar))
	bidTickSize := uint32(cmd.BidTickSize)
	askTickSize := uint32(cmd.AskTickSize)

	return tx.MoveCall(c.poolTarget("update_tpsl_params"), []stx.Argument{
		tx.Object(pool.Address),
		pureU64(tx, bidPrice),
		pureU64(tx, askPrice),
		pureU8(tx, bidTickSize),
		pureU8(tx, askTickSize),
	})
}

func (c *MarginTPSLContract) SetEmergency(tx *stx.Transaction, poolKey string, isEmergency bool) {
	pool := c.config.GetPool(poolKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	isEmergencyUint := uint8(0)
	if isEmergency {
		isEmergencyUint = 1
	}

	return tx.MoveCall(c.poolTarget("set_emergency"), []stx.Argument{
		tx.Object(pool.Address),
		pureU8(tx, isEmergencyUint),
	})
}

func (c *MarginTPSLContract) GetTPSLParams(tx *stx.Transaction, poolKey string) {
	pool := c.config.GetPool(poolKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)

	_, _, _ = tx.MakeMoveVec(tx.Object("0x6"))

	return tx.MoveCall(c.poolTarget("get_tpsl_params"), []stx.Argument{
		tx.Object(pool.Address),
	})
}
