package transactions

import (
	"math"

	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	stx "github.com/sui-sdks/go-sdks/sui/transactions"
)

type GovernanceContract struct {
	config         *utils.DeepBookConfig
	balanceManager *BalanceManagerContract
}

func NewGovernanceContract(config *utils.DeepBookConfig, balanceManager *BalanceManagerContract) *GovernanceContract {
	return &GovernanceContract{config: config, balanceManager: balanceManager}
}

func (c *GovernanceContract) Stake(tx *stx.Transaction, poolKey, balanceManagerKey string, stakeAmount float64) stx.Argument {
	pool := c.config.GetPool(poolKey)
	manager := c.config.GetBalanceManager(balanceManagerKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	stake := uint64(math.Round(stakeAmount * utils.DeepScalar))
	proof := c.balanceManager.GenerateProof(tx, balanceManagerKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)
	return tx.MoveCall(c.config.DeepbookPackageID+"::pool::stake", []stx.Argument{
		tx.Object(pool.Address),
		tx.Object(manager.Address),
		proof,
		pureU64(tx, stake),
	}, []string{base.Type, quote.Type})
}

func (c *GovernanceContract) Unstake(tx *stx.Transaction, poolKey, balanceManagerKey string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	manager := c.config.GetBalanceManager(balanceManagerKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	proof := c.balanceManager.GenerateProof(tx, balanceManagerKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)
	return tx.MoveCall(c.config.DeepbookPackageID+"::pool::unstake", []stx.Argument{
		tx.Object(pool.Address),
		tx.Object(manager.Address),
		proof,
	}, []string{base.Type, quote.Type})
}

func (c *GovernanceContract) SubmitProposal(tx *stx.Transaction, params types.ProposalParams) stx.Argument {
	pool := c.config.GetPool(params.PoolKey)
	manager := c.config.GetBalanceManager(params.BalanceManagerKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	proof := c.balanceManager.GenerateProof(tx, params.BalanceManagerKey)
	takerFee := uint64(math.Round(params.TakerFee * utils.FloatScalar))
	makerFee := uint64(math.Round(params.MakerFee * utils.FloatScalar))
	stakeRequired := uint64(math.Round(params.StakeRequired * utils.DeepScalar))
	tx.SetGasBudgetIfNotSet(utils.GasBudget)
	return tx.MoveCall(c.config.DeepbookPackageID+"::pool::submit_proposal", []stx.Argument{
		tx.Object(pool.Address),
		tx.Object(manager.Address),
		proof,
		pureU64(tx, takerFee),
		pureU64(tx, makerFee),
		pureU64(tx, stakeRequired),
	}, []string{base.Type, quote.Type})
}

func (c *GovernanceContract) Vote(tx *stx.Transaction, poolKey, balanceManagerKey, proposalID string) stx.Argument {
	pool := c.config.GetPool(poolKey)
	manager := c.config.GetBalanceManager(balanceManagerKey)
	base := c.config.GetCoin(pool.BaseCoin)
	quote := c.config.GetCoin(pool.QuoteCoin)
	proof := c.balanceManager.GenerateProof(tx, balanceManagerKey)
	tx.SetGasBudgetIfNotSet(utils.GasBudget)
	return tx.MoveCall(c.config.DeepbookPackageID+"::pool::vote", []stx.Argument{
		tx.Object(pool.Address),
		tx.Object(manager.Address),
		proof,
		pureAddress(tx, proposalID),
	}, []string{base.Type, quote.Type})
}
