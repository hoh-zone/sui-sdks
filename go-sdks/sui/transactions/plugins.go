package transactions

import (
	"fmt"
	"strings"
)

type TransactionKind string

const (
	KindMoveCall     TransactionKind = "moveCall"
	KindProgrammable TransactionKind = "programmableTransaction"
)

type Plugin interface {
	Name() string
	BeforeTransaction(tx *Transaction, kind TransactionKind) error
	AfterTransaction(tx *Transaction, result any, err error) error
	Build(tx *Transaction) error
}

type NamedPackagesPlugin struct {
	packages map[string]string
}

func NewNamedPackagesPlugin(packages map[string]string) *NamedPackagesPlugin {
	return &NamedPackagesPlugin{
		packages: packages,
	}
}

func (p *NamedPackagesPlugin) Name() string {
	return "NamedPackagesPlugin"
}

func (p *NamedPackagesPlugin) BeforeTransaction(tx *Transaction, kind TransactionKind) error {
	_ = kind
	replaceNamedPackagesInAny(tx.data.Commands, p.packages)
	replaceNamedPackagesInAny(tx.data.Inputs, p.packages)
	return nil
}

func (p *NamedPackagesPlugin) AfterTransaction(tx *Transaction, result any, err error) error {
	_ = result
	_ = err
	if unresolved := findFirstNamedPackage(tx.data.Commands); unresolved != "" {
		return fmt.Errorf("unresolved named package: %s", unresolved)
	}
	return nil
}

func (p *NamedPackagesPlugin) Build(tx *Transaction) error {
	replaceNamedPackagesInAny(tx.data.Commands, p.packages)
	replaceNamedPackagesInAny(tx.data.Inputs, p.packages)
	return nil
}

func (p *NamedPackagesPlugin) Resolve(name string) (string, bool) {
	packageId, ok := p.packages[name]
	return packageId, ok
}

type PluginManager struct {
	plugins []Plugin
}

func NewPluginManager() *PluginManager {
	return &PluginManager{
		plugins: []Plugin{},
	}
}

func (pm *PluginManager) Register(plugin Plugin) {
	pm.plugins = append(pm.plugins, plugin)
}

func (pm *PluginManager) Unregister(plugin Plugin) {
	for i, p := range pm.plugins {
		if p.Name() == plugin.Name() {
			pm.plugins = append(pm.plugins[:i], pm.plugins[i+1:]...)
			break
		}
	}
}

func (pm *PluginManager) Get(name string) (Plugin, bool) {
	for _, plugin := range pm.plugins {
		if plugin.Name() == name {
			return plugin, true
		}
	}
	return nil, false
}

func (pm *PluginManager) BeforeTransaction(tx *Transaction, kind TransactionKind) error {
	for _, plugin := range pm.plugins {
		if err := plugin.BeforeTransaction(tx, kind); err != nil {
			return err
		}
	}
	return nil
}

func (pm *PluginManager) AfterTransaction(tx *Transaction, result any, err error) error {
	for _, plugin := range pm.plugins {
		if err := plugin.AfterTransaction(tx, result, err); err != nil {
			return err
		}
	}
	return nil
}

func (pm *PluginManager) Build(tx *Transaction) error {
	for _, plugin := range pm.plugins {
		if err := plugin.Build(tx); err != nil {
			return err
		}
	}
	return nil
}

type ValidatorPlugin struct {
	validator func(tx *Transaction) error
}

func NewValidatorPlugin(validator func(tx *Transaction) error) *ValidatorPlugin {
	return &ValidatorPlugin{
		validator: validator,
	}
}

func (p *ValidatorPlugin) Name() string {
	return "ValidatorPlugin"
}

func (p *ValidatorPlugin) BeforeTransaction(tx *Transaction, kind TransactionKind) error {
	if p.validator != nil {
		return p.validator(tx)
	}
	return nil
}

func (p *ValidatorPlugin) AfterTransaction(tx *Transaction, result any, err error) error {
	return nil
}

func (p *ValidatorPlugin) Build(tx *Transaction) error {
	return nil
}

func replaceNamedPackagesInAny(v any, packages map[string]string) {
	switch x := v.(type) {
	case map[string]any:
		for k, val := range x {
			if s, ok := val.(string); ok {
				x[k] = replaceNamedPackageString(s, packages)
			} else {
				replaceNamedPackagesInAny(val, packages)
			}
		}
	case []any:
		for i := range x {
			if s, ok := x[i].(string); ok {
				x[i] = replaceNamedPackageString(s, packages)
			} else {
				replaceNamedPackagesInAny(x[i], packages)
			}
		}
	case []Command:
		for i := range x {
			replaceNamedPackagesInAny(map[string]any(x[i]), packages)
		}
	case []CallArg:
		for i := range x {
			replaceNamedPackagesInAny(map[string]any(x[i]), packages)
		}
	case []Argument:
		for i := range x {
			replaceNamedPackagesInAny(map[string]any(x[i]), packages)
		}
	case []string:
		for i := range x {
			x[i] = replaceNamedPackageString(x[i], packages)
		}
	}
}

func replaceNamedPackageString(value string, packages map[string]string) string {
	out := value
	for name, address := range packages {
		if out == name {
			out = address
			continue
		}
		out = strings.ReplaceAll(out, name+"::", address+"::")
		out = strings.ReplaceAll(out, "<"+name+"::", "<"+address+"::")
		out = strings.ReplaceAll(out, ","+name+"::", ","+address+"::")
	}
	return out
}

func findFirstNamedPackage(v any) string {
	switch x := v.(type) {
	case map[string]any:
		for _, val := range x {
			if unresolved := findFirstNamedPackage(val); unresolved != "" {
				return unresolved
			}
		}
	case []any:
		for _, val := range x {
			if unresolved := findFirstNamedPackage(val); unresolved != "" {
				return unresolved
			}
		}
	case []Command:
		for _, val := range x {
			if unresolved := findFirstNamedPackage(map[string]any(val)); unresolved != "" {
				return unresolved
			}
		}
	case []CallArg:
		for _, val := range x {
			if unresolved := findFirstNamedPackage(map[string]any(val)); unresolved != "" {
				return unresolved
			}
		}
	case []Argument:
		for _, val := range x {
			if unresolved := findFirstNamedPackage(map[string]any(val)); unresolved != "" {
				return unresolved
			}
		}
	case string:
		if strings.Contains(x, "/") && strings.Contains(x, "::") {
			return strings.SplitN(x, "::", 2)[0]
		}
	}
	return ""
}
