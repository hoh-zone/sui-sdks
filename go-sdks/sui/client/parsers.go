package client

import (
	"encoding/binary"
	"fmt"

	"github.com/sui-sdks/go-sdks/bcs"
)

func ParseTransactionEffectsBcs(data []byte) (*TransactionEffects, error) {
	reader := bcs.NewReader(data)

	effects := &TransactionEffects{}

	if version, err := reader.Read64(); err != nil {
		return nil, fmt.Errorf("failed to read version: %w", err)
	} else {
		effects.Version = version
	}

	statusKind, err := reader.Read8()
	if err != nil {
		return nil, fmt.Errorf("failed to read status kind: %w", err)
	}

	effects.Status = Status{
		Success: statusKind == 0,
	}

	gasUsed, err := parseGasCostSummary(reader)
	if err != nil {
		return nil, fmt.Errorf("failed to read gas used: %w", err)
	}
	effects.GasUsed = *gasUsed

	if txDigest, err := readFixedString(reader, 32); err != nil {
		return nil, err
	} else {
		effects.TransactionDigest = txDigest
	}

	if eventsDigestLen, err := reader.Read32(); err == nil && eventsDigestLen > 0 {
		eventsDigest, err := readFixedBytes(reader, int(eventsDigestLen))
		if err != nil {
			return nil, fmt.Errorf("failed to read events digest: %w", err)
		}
		effects.EventsDigest = string(eventsDigest)
	}

	numDependencies, err := reader.ReadULEB()
	if err == nil {
		deps := make([]string, numDependencies)
		for i := uint64(0); i < numDependencies; i++ {
			if dep, err := readFixedString(reader, 32); err != nil {
				return nil, fmt.Errorf("failed to read dependency: %w", err)
			} else {
				deps[i] = dep
			}
		}
		effects.Dependencies = deps
	}

	numChanged, err := reader.ReadULEB()
	if err == nil && numChanged > 0 {
		items := make([]ChangedObject, numChanged)
		for i := uint64(0); i < numChanged; i++ {
			item, err := parseChangedObject(reader)
			if err != nil {
				return nil, fmt.Errorf("failed to read changed object %d: %w", i, err)
			}
			items[i] = *item
		}
		effects.ChangedObjects = items
	}

	return effects, nil
}

func ParseTransactionBcs(data []byte) (map[string]interface{}, error) {
	result := make(map[string]interface{})

	reader := bcs.NewReader(data)

	if txKind, err := reader.Read8(); err == nil {
		result["kind"] = txKind
	}

	if protocolVersion, err := reader.Read32(); err == nil {
		result["protocolVersion"] = protocolVersion
	}

	return result, nil
}

func ExtractStatusFromEffectsBcs(data []byte) (*ExecutionStatus, error) {
	reader := bcs.NewReader(data)

	statusKind, err := reader.Read8()
	if err != nil {
		return nil, fmt.Errorf("failed to read status kind: %w", err)
	}

	status := &ExecutionStatus{
		Success: statusKind == 0,
	}

	return status, nil
}

func FormatMoveAbortMessage(data []byte) string {
	if len(data) < 4 {
		return "unknown"
	}

	return fmt.Sprintf("0x%X", binary.LittleEndian.Uint32(data[0:4]))
}

func parseGasCostSummary(reader *bcs.Reader) (*GasCostSummary, error) {
	summary := &GasCostSummary{}

	if computationCost, err := reader.Read64(); err != nil {
		return nil, err
	} else {
		summary.ComputationCost = fmt.Sprintf("%d", computationCost)
	}

	if storageCost, err := reader.Read64(); err != nil {
		return nil, err
	} else {
		summary.StorageCost = fmt.Sprintf("%d", storageCost)
	}

	if storageRebate, err := reader.Read64(); err != nil {
		return nil, err
	} else {
		summary.StorageRebate = fmt.Sprintf("%d", storageRebate)
	}

	if nonRefundableStorageFee, err := reader.Read64(); err != nil {
		return nil, err
	} else {
		summary.NonRefundableStorageFee = fmt.Sprintf("%d", nonRefundableStorageFee)
	}

	return summary, nil
}

func parseChangedObject(reader *bcs.Reader) (*ChangedObject, error) {
	obj := &ChangedObject{}

	objectID, err := readFixedString(reader, 32)
	if err != nil {
		return nil, err
	}
	obj.ObjectID = objectID

	stateKind, err := reader.Read8()
	if err != nil {
		return nil, err
	}

	switch stateKind {
	case 0:
		obj.InputState = InputStateUnknown
	case 1:
		obj.InputState = InputStateDoesNotExist
	case 2:
		obj.InputState = InputStateExists
	}

	if obj.InputState == InputStateExists {
		if inputVersion, err := reader.ReadULEB(); err == nil {
			version := fmt.Sprintf("%d", inputVersion)
			obj.InputVersion = &version
		}
		if digest := readDigest(reader); digest != "" {
			obj.InputDigest = &digest
		}
		if owner := readObjectOwner(reader); owner != nil {
			obj.InputOwner = owner
		}
	}

	return obj, nil
}

func readFixedString(reader *bcs.Reader, length int) (string, error) {
	data := make([]byte, length)
	for i := 0; i < length; i++ {
		b, err := reader.Read8()
		if err != nil {
			return "", err
		}
		data[i] = b
	}
	return string(data), nil
}

func readFixedBytes(reader *bcs.Reader, length int) ([]byte, error) {
	data := make([]byte, length)
	for i := 0; i < length; i++ {
		b, err := reader.Read8()
		if err != nil {
			return nil, err
		}
		data[i] = b
	}
	return data, nil
}

func readDigest(reader *bcs.Reader) string {
	data, err := readFixedBytes(reader, 32)
	if err != nil {
		return ""
	}
	return string(data)
}

func readObjectOwner(reader *bcs.Reader) *ObjectOwner {
	kind, err := reader.Read8()
	if err != nil {
		return nil
	}

	owner := &ObjectOwner{Kind: fmt.Sprintf("%d", kind)}
	switch kind {
	case 0:
		if addr := readAddress(reader); addr != "" {
			owner.AddressOwner = &addr
		}
	case 1:
		if objID := readObjectID(reader); objID != "" {
			owner.ObjectOwner = &objID
		}
	case 2:
		owner.Shared = &SharedOwner{
			InitialSharedVersion: readULEB128String(reader),
		}
	case 3:
		immutable := true
		owner.Immutable = &immutable
	}

	return owner
}

func readAddress(reader *bcs.Reader) string {
	data, err := readFixedBytes(reader, 32)
	if err != nil {
		return ""
	}
	return string(data)
}

func readObjectID(reader *bcs.Reader) string {
	data, err := readFixedBytes(reader, 32)
	if err != nil {
		return ""
	}
	return string(data)
}

func readULEB128String(reader *bcs.Reader) string {
	val, err := reader.ReadULEB()
	if err != nil {
		return "0"
	}
	return fmt.Sprintf("%d", val)
}
