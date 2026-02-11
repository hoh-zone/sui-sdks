package transactions

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
)

const (
	GasSafeOverhead = int64(1000)
	MaxGas          = int64(50_000_000_000)
)

func CoreClientResolveTransaction(transactionData *TransactionData, options BuildTransactionOptions) error {
	if options.Client == nil {
		return errors.New("missing core client")
	}
	if err := normalizeInputs(transactionData); err != nil {
		return err
	}
	if err := resolveObjectReferences(transactionData, options.Client); err != nil {
		return err
	}
	if !options.OnlyTransactionKind {
		if err := setGasData(transactionData, options.Client); err != nil {
			return err
		}
	}
	return nil
}

func normalizeInputs(transactionData *TransactionData) error {
	for i, input := range transactionData.Inputs {
		if input["$kind"] == "UnresolvedPure" {
			transactionData.Inputs[i] = Inputs.Pure(encodeUnresolvedPure(input["UnresolvedPure"]))
		}
	}
	return nil
}

func resolveObjectReferences(transactionData *TransactionData, client CoreClient) error {
	for i, input := range transactionData.Inputs {
		if input["$kind"] != "UnresolvedObject" {
			continue
		}
		payload, _ := input["UnresolvedObject"].(map[string]any)
		objectID, _ := payload["objectId"].(string)
		if objectID == "" {
			return fmt.Errorf("invalid unresolved object at input %d", i)
		}
		resolved, err := resolveObjectInput(context.Background(), client, objectID, payload)
		if err != nil {
			return err
		}
		transactionData.Inputs[i] = resolved
	}
	return nil
}

func setGasData(transactionData *TransactionData, client CoreClient) error {
	if transactionData.GasData.Price == "" {
		var price string
		if err := client.Call(context.Background(), "suix_getReferenceGasPrice", []any{}, &price); err != nil {
			transactionData.GasData.Price = "1"
		} else {
			transactionData.GasData.Price = price
		}
	}
	if transactionData.GasData.Budget == "" {
		transactionData.GasData.Budget = itoa64(MaxGas)
	}
	if transactionData.GasData.Payment == nil {
		transactionData.GasData.Payment = []ObjectRef{}
	}
	if transactionData.Expiration == nil {
		transactionData.Expiration = map[string]any{
			"$kind": "ValidDuring",
			"ValidDuring": map[string]any{
				"minEpoch": "0",
				"maxEpoch": "1",
				"chain":    "unknown",
				"nonce":    0,
			},
		}
	}
	return nil
}

func encodeUnresolvedPure(v any) []byte {
	switch t := v.(type) {
	case []byte:
		return append([]byte(nil), t...)
	case string:
		return []byte(t)
	case map[string]any:
		if inner, ok := t["value"]; ok {
			return encodeUnresolvedPure(inner)
		}
	}
	if b, err := json.Marshal(v); err == nil {
		return b
	}
	return []byte(toString(v))
}

func resolveObjectInput(ctx context.Context, client CoreClient, objectID string, fallback map[string]any) (CallArg, error) {
	version := fallback["version"]
	digest := fallback["digest"]
	if version != nil && digest != nil {
		return makeImmOrOwnedInput(objectID, version, digest), nil
	}

	var obj map[string]any
	if err := client.Call(ctx, "sui_getObject", []any{objectID, map[string]any{"showOwner": true}}, &obj); err != nil {
		// fallback to unresolved payload-derived default if network fetch fails
		return makeImmOrOwnedInput(objectID, nonNil(version, "0"), nonNil(digest, "")), nil
	}

	data := pickMap(obj, "data")
	if data == nil {
		data = obj
	}
	version = nonNil(data["version"], nonNil(version, "0"))
	digest = nonNil(data["digest"], nonNil(digest, ""))
	owner := pickMap(data, "owner")
	if owner != nil {
		if shared := pickMap(owner, "Shared"); shared != nil {
			initial := nonNil(shared["initial_shared_version"], shared["initialSharedVersion"])
			if initial != nil {
				return CallArg{
					"$kind": "Object",
					"Object": map[string]any{
						"$kind": "SharedObject",
						"SharedObject": map[string]any{
							"objectId":             objectID,
							"mutable":              true,
							"initialSharedVersion": initial,
						},
					},
				}, nil
			}
		}
	}
	return makeImmOrOwnedInput(objectID, version, digest), nil
}

func makeImmOrOwnedInput(objectID string, version any, digest any) CallArg {
	return CallArg{
		"$kind": "Object",
		"Object": map[string]any{
			"$kind": "ImmOrOwnedObject",
			"ImmOrOwnedObject": map[string]any{
				"objectId": objectID,
				"version":  nonNil(version, "0"),
				"digest":   nonNil(digest, ""),
			},
		},
	}
}

func nonNil(v any, defaultValue any) any {
	if v == nil {
		return defaultValue
	}
	return v
}

func pickMap(m map[string]any, key string) map[string]any {
	if m == nil {
		return nil
	}
	if val, ok := m[key].(map[string]any); ok {
		return val
	}
	return nil
}
