package client

type Status struct {
	Success bool   `json:"success"`
	Error   string `json:"error,omitempty"`
}

type Object struct {
	ObjectID string `json:"objectId"`
	Version  string `json:"version"`
	Digest   string `json:"digest"`
	Owner    string `json:"owner"`
	Type     string `json:"type"`
	Content  []byte `json:"content,omitempty"`
}

type TransactionEffects struct {
	Version                   uint64                     `json:"version"`
	Status                    Status                     `json:"status"`
	GasUsed                   GasCostSummary             `json:"gasUsed"`
	TransactionDigest         string                     `json:"transactionDigest"`
	GasObject                 *Object                    `json:"gasObject,omitempty"`
	EventsDigest              string                     `json:"eventsDigest,omitempty"`
	Dependencies              []string                   `json:"dependencies,omitempty"`
	LamportVersion            *string                    `json:"lamportVersion,omitempty"`
	ChangedObjects            []ChangedObject            `json:"changedObjects,omitempty"`
	UnchangedConsensusObjects []UnchangedConsensusObject `json:"unchangedConsensusObjects,omitempty"`
	AuxiliaryDataDigest       *string                    `json:"auxiliaryDataDigest,omitempty"`
}

type Transaction struct {
	Digest         string              `json:"digest"`
	Signatures     []string            `json:"signatures"`
	Epoch          *string             `json:"epoch,omitempty"`
	Status         ExecutionStatus     `json:"status"`
	BalanceChanges []BalanceChange     `json:"balanceChanges,omitempty"`
	Effects        *TransactionEffects `json:"effects,omitempty"`
	Events         []Event             `json:"events,omitempty"`
	ObjectTypes    map[string]string   `json:"objectTypes,omitempty"`
	Transaction    any                 `json:"transaction,omitempty"`
	BCS            []byte              `json:"bcs,omitempty"`
}

type GasCostSummary struct {
	ComputationCost         string `json:"computationCost"`
	StorageCost             string `json:"storageCost"`
	StorageRebate           string `json:"storageRebate"`
	NonRefundableStorageFee string `json:"nonRefundableStorageFee"`
}

type ChangedObject struct {
	ObjectID      string       `json:"objectId"`
	InputState    InputState   `json:"inputState"`
	InputVersion  *string      `json:"inputVersion,omitempty"`
	InputDigest   *string      `json:"inputDigest,omitempty"`
	InputOwner    *ObjectOwner `json:"inputOwner,omitempty"`
	OutputState   OutputState  `json:"outputState"`
	OutputVersion *string      `json:"outputVersion,omitempty"`
	OutputDigest  *string      `json:"outputDigest,omitempty"`
	OutputOwner   *ObjectOwner `json:"outputOwner,omitempty"`
	IDOperation   string       `json:"idOperation"`
}

type InputState string

const (
	InputStateUnknown      InputState = "Unknown"
	InputStateDoesNotExist InputState = "DoesNotExist"
	InputStateExists       InputState = "Exists"
)

type OutputState string

const (
	OutputStateUnknown            OutputState = "Unknown"
	OutputStateDoesNotExist       OutputState = "DoesNotExist"
	OutputStateObjectWrite        OutputState = "ObjectWrite"
	OutputStatePackageWrite       OutputState = "PackageWrite"
	OutputStateAccumulatorWriteV1 OutputState = "AccumulatorWriteV1"
)

type UnchangedConsensusObject struct {
	Kind     string  `json:"kind"`
	ObjectID string  `json:"objectId"`
	Version  *string `json:"version,omitempty"`
	Digest   *string `json:"digest,omitempty"`
}

type ObjectOwner struct {
	Kind string `json:"$kind"`

	AddressOwner          *string                `json:"AddressOwner,omitempty"`
	ObjectOwner           *string                `json:"ObjectOwner,omitempty"`
	Shared                *SharedOwner           `json:"Shared,omitempty"`
	Immutable             *bool                  `json:"Immutable,omitempty"`
	ConsensusAddressOwner *ConsensusAddressOwner `json:"ConsensusAddressOwner,omitempty"`
}

type SharedOwner struct {
	InitialSharedVersion string `json:"initialSharedVersion"`
}

type ConsensusAddressOwner struct {
	StartVersion string `json:"startVersion"`
	Owner        string `json:"owner"`
}

type Event struct {
	PackageID string `json:"packageId"`
	Module    string `json:"module"`
	Sender    string `json:"sender"`
	EventType string `json:"eventType"`
	BCS       []byte `json:"bcs"`
}

type BalanceChange struct {
	CoinType string `json:"coinType"`
	Address  string `json:"address"`
	Amount   string `json:"amount"`
}

type ExecutionStatus struct {
	Success bool            `json:"success"`
	Error   *ExecutionError `json:"error,omitempty"`
}

type ExecutionError struct {
	Message string `json:"message"`
	Command *int   `json:"command,omitempty"`
}

type SimError struct {
	Message string `json:"message"`
	Error   string `json:"error,omitempty"`
}
