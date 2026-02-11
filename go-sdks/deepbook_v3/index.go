package deepbookv3

import (
	"github.com/sui-sdks/go-sdks/deepbook_v3/transactions"
	"github.com/sui-sdks/go-sdks/deepbook_v3/types"
	"github.com/sui-sdks/go-sdks/deepbook_v3/utils"
	"github.com/sui-sdks/go-sdks/stx"
)

type (
	DeepBookOptions          = Options
	DeepBookClientOptions    = ClientOptions
	DeepBookCompatibleClient = CompatibleClient
	DeepBookConfig           = utils.DeepBookConfig
)

var (
	MainnetCoins       = utils.MainnetCoins
	TestnetCoins       = utils.TestnetCoins
	MainnetPools       = utils.MainnetPools
	TestnetPools       = utils.TestnetPools
	MainnetMarginPools = utils.MainnetMarginPools
	TestnetMarginPools = utils.TestnetMarginPools
	MainnetPackageIDs  = utils.MainnetPackageIDs
	TestnetPackageIDs  = utils.TestnetPackageIDs
	MainnetPythConfigs = utils.MainnetPythConfigs
	TestnetPythConfigs = utils.TestnetPythConfigs
	DeepScalar         = utils.DeepScalar
	FloatScalar        = utils.FloatScalar
	GasBudget          = utils.GasBudget
	MaxTimestamp       = utils.MaxTimestamp

	NewBalanceManagerContract     = transactions.NewBalanceManagerContract
	NewDeepBookContract           = transactions.NewDeepBookContract
	NewDeepBookAdminContract      = transactions.NewDeepBookAdminContract
	NewFlashLoanContract          = transactions.NewFlashLoanContract
	NewGovernanceContract         = transactions.NewGovernanceContract
	NewMarginAdminContract        = transactions.NewMarginAdminContract
	NewMarginMaintainerContract   = transactions.NewMarginMaintainerContract
	NewMarginManagerContract      = transactions.NewMarginManagerContract
	NewMarginPoolContract         = transactions.NewMarginPoolContract
	NewMarginRegistryContract     = transactions.NewMarginRegistryContract
	NewMarginLiquidationsContract = transactions.NewMarginLiquidationsContract
	NewMarginTPSLContract         = transactions.NewMarginTPSLContract
	NewPoolProxyContract          = transactions.NewPoolProxyContract
	NewSuiPythClient              = pyth.NewSuiPythClient
)

func Deepbook(opts DeepBookClientOptions) *Client {
	return NewClient(opts)
}
