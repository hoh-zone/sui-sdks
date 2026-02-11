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

type OrderType uint8

const (
	OrderTypeLimit  OrderType = 0
	OrderTypeMarket OrderType = 1
	OrderTypeBid    OrderType = 2
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
	OrderType     OrderType
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
	OrderType   OrderType
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
