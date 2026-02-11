package bcs

import (
	"fmt"
	"strings"
	"sync"
)

type TypeTag struct {
	Address string
	Module  string
	Name    string
}

type TypeTagOptions struct {
	address string
}

type tagCache struct {
	registry map[string]string
	mu       sync.RWMutex
}

var globalTagCache tagCache = tagCache{
	registry: make(map[string]string),
	mu:       sync.RWMutex{},
}

func (t *TypeTag) String() string {
	if t.Address != "" {
		return fmt.Sprintf("%s::%s::%s", t.Address, t.Module, t.Name)
	}
	return fmt.Sprintf("%s::%s", t.Module, t.Name)
}

func NormalizeTypeTag(tag string, opts ...TypeTagOptions) string {
	tag = strings.TrimSpace(tag)

	if len(opts) > 0 && opts[0].address != "" {
		tag = strings.ReplaceAll(tag, "0x1", "0x"+opts[0].address)
	}

	parts := strings.Split(tag, "::")
	if len(parts) >= 2 {
		module := strings.Split(parts[1], "<")[0]
		name := strings.Split(parts[len(parts)-1], "<")[0]
		return fmt.Sprintf("%s::%s::%s", parts[0], module, name)
	}

	return tag
}

func ParseTypeTag(tag string) *TypeTag {
	parts := strings.Split(tag, "::")
	if len(parts) < 3 {
		return nil
	}

	result := &TypeTag{
		Address: strings.TrimSpace(parts[0]),
		Module:  strings.Split(parts[1], "<")[0],
		Name:    strings.Split(parts[len(parts)-1], "<")[0],
	}

	return result
}

func (tt *TypeTag) Serialize() ([]byte, error) {
	ser := NewWriter()

	ser.WriteULEB(uint64(len(tt.Address)))
	ser.WriteBytes([]byte(tt.Address))

	ser.WriteULEB(uint64(len(tt.Module)))
	ser.WriteBytes([]byte(tt.Module))

	ser.WriteULEB(uint64(len(tt.Name)))
	ser.WriteBytes([]byte(tt.Name))

	return ser.ToBytes(), nil
}

func TypeTagFromBytes(data []byte) (*TypeTag, error) {
	reader := NewReader(data)

	addrLen, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}

	address, err := reader.ReadBytes(int(addrLen))
	if err != nil {
		return nil, err
	}

	moduleLen, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}

	module, err := reader.ReadBytes(int(moduleLen))
	if err != nil {
		return nil, err
	}

	nameLen, err := reader.ReadULEB()
	if err != nil {
		return nil, err
	}

	name, err := reader.ReadBytes(int(nameLen))
	if err != nil {
		return nil, err
	}

	return &TypeTag{
		Address: string(address),
		Module:  string(module),
		Name:    string(name),
	}, nil
}

func (tt *TypeTag) Validate() error {
	if tt.Address == "" {
		return fmt.Errorf("type tag address is required")
	}
	if tt.Module == "" {
		return fmt.Errorf("type tag module is required")
	}
	if tt.Name == "" {
		return fmt.Errorf("type tag name is required")
	}
	return nil
}
