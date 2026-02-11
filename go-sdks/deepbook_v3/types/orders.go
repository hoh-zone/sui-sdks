package types

type OrderKind uint8

const (
	OrderKindLimitLimit  OrderKind = 0
	OrderKindMarketOrder OrderKind = 1
)

type OrderSide uint8

const (
	OrderSideBid OrderSide = 0
	OrderSideAsk OrderSide = 1
)

type QueryOrderType uint8

const (
	QueryOrderTypeLimit  QueryOrderType = 0
	QueryOrderTypeMarket QueryOrderType = 1
	QueryOrderTypeBid    QueryOrderType = 2
)

type OrderStatus uint8

const (
	OrderStatusOpen      OrderStatus = 0
	OrderStatusFilled    OrderStatus = 1
	OrderStatusCancelled OrderStatus = 2
	OrderStatusRejected  OrderStatus = 3
)

type Order struct {
	OrderID       string
	ClientOrderID string
	Trader        string
	PoolKey       string
	BaseCoin      string
	QuoteCoin     string
	OrderType     QueryOrderType
	OrderSide     OrderSide
	BaseQuantity  uint64
	QuoteQuantity uint64
	Price         uint64
	BaseTimestamp uint64
	Expiration    uint64
	IsBid         bool
}

type LimitOrder struct {
	Order
}

type MarketOrder struct {
	Order
	SelfMatchingOption uint8
}

type BatchOrderCancellation struct {
	Orders []string
}

type OrderQueryOptions struct {
	Trader      string
	PoolKey     string
	OrderType   QueryOrderType
	OrderSide   OrderSide
	OrderStatus []OrderStatus
	StartTime   *uint64
	EndTime     *uint64
	Limit       *uint64
	Cursor      *string
}

type OrderHistory struct {
	Orders  []Order
	HasMore bool
	Cursor  string
}
