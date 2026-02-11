package utils

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"strings"
)

const SuiAddressLength = SUI_ADDRESS_LENGTH
const SUI_ADDRESS_LENGTH = 32

func NormalizeSuiAddress(addr string) string {
	a := strings.TrimPrefix(strings.ToLower(strings.TrimSpace(addr)), "0x")
	if len(a) > SuiAddressLength*2 {
		a = a[len(a)-SuiAddressLength*2:]
	}
	if len(a) < SuiAddressLength*2 {
		a = strings.Repeat("0", SuiAddressLength*2-len(a)) + a
	}
	return "0x" + a
}

func NormalizeSuiObjectID(id string) string {
	return NormalizeSuiAddress(id)
}

func IsValidSuiAddress(addr string) bool {
	a := strings.TrimPrefix(strings.ToLower(strings.TrimSpace(addr)), "0x")
	if len(a) == 0 || len(a) > SuiAddressLength*2 {
		return false
	}
	_, err := hex.DecodeString(a)
	return err == nil
}

func IsValidSuiObjectID(id string) bool {
	return IsValidSuiAddress(id)
}

func MustNormalizeSuiAddress(addr string) string {
	if !IsValidSuiAddress(addr) {
		panic(fmt.Sprintf("invalid Sui address: %s", addr))
	}
	return NormalizeSuiAddress(addr)
}

func NormalizeStructTag(tag string) string {
	tag = strings.TrimSpace(tag)

	addrParts := strings.Split(tag, "::")
	if len(addrParts) >= 2 {
		module := strings.Split(addrParts[1], "<")[0]
		name := strings.Split(addrParts[len(addrParts)-1], "<")[0]
		return fmt.Sprintf("%s::%s::%s", NormalizeSuiAddress(addrParts[0]), module, name)
	}

	return NormalizeSuiAddress(tag)
}

type StructTag struct {
	Address string
	Module  string
	Name    string
}

func ParseStructTag(tag string) *StructTag {
	parts := strings.Split(tag, "::")
	if len(parts) < 3 {
		return nil
	}

	result := &StructTag{
		Address: NormalizeSuiAddress(parts[0]),
		Module:  strings.Split(parts[1], "<")[0],
		Name:    strings.Split(parts[len(parts)-1], "<")[0],
	}

	return result
}

func (s *StructTag) String() string {
	return fmt.Sprintf("%s::%s::%s", s.Address, s.Module, s.Name)
}

func DeriveDynamicFieldID(parentID string, nameType string, nameBCS []byte) string {
	hasher := sha256.New()

	_, _ = hasher.Write([]byte(parentID))
	_, _ = hasher.Write([]byte(nameType))
	_, _ = hasher.Write(nameBCS)

	hash := hasher.Sum(nil)

	hashLen := len(hash)
	if hashLen == 32 {
		var id [32]byte
		copy(id[:], hash)

		id[31] = (id[31] & 0x3F) | 0x3D

		return fmt.Sprintf("0x%x", id)
	}

	return fmt.Sprintf("0x%x", hash)
}

func ParseU8(bytes []byte) uint8 {
	if len(bytes) < 1 {
		return 0
	}
	return uint8(bytes[0])
}

func ParseU16(bytes []byte) uint16 {
	if len(bytes) < 2 {
		return 0
	}
	return binary.LittleEndian.Uint16(bytes)
}

func ParseU32(bytes []byte) uint32 {
	if len(bytes) < 4 {
		return 0
	}
	return binary.LittleEndian.Uint32(bytes)
}

func ParseU64(bytes []byte) uint64 {
	if len(bytes) < 8 {
		return 0
	}
	return binary.LittleEndian.Uint64(bytes)
}

func ParseI8(bytes []byte) int8 {
	return int8(ParseU8(bytes))
}

func ParseI16(bytes []byte) int16 {
	return int16(ParseU16(bytes))
}

func ParseI32(bytes []byte) int32 {
	return int32(ParseU32(bytes))
}

func ParseI64(bytes []byte) int64 {
	return int64(ParseU64(bytes))
}

func ParseBool(bytes []byte) bool {
	if len(bytes) < 1 {
		return false
	}
	return bytes[0] == 1
}

func ParseString(bytes []byte) string {
	return string(bytes)
}
