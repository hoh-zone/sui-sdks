package utils

import (
	"crypto/sha256"
	"encoding/hex"
	"strings"
)

type SuiNSConfig struct {
	RegistryAddress    string
	NameServiceAddress string
}

type DomainRecord struct {
	Domain  string
	Target  string
	Version string
	Digest  string
}

type NameServiceConfig struct {
	RegistryID    string
	NameServiceID string
}

func NewNameServiceConfig(registryID, nameServiceID string) *NameServiceConfig {
	return &NameServiceConfig{
		RegistryID:    normalizeAddress(registryID),
		NameServiceID: normalizeAddress(nameServiceID),
	}
}

func normalizeAddress(addr string) string {
	return NormalizeSuiAddress(addr)
}

func (c *NameServiceConfig) RegistryPackageID() string {
	return c.RegistryID
}

func DefaultSuiNSRegistryPackage() string {
	return "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf"
}

func DefaultSuiNameServicePackage() string {
	return "0x2"
}

func DeriveDomainId(domain string, config *NameServiceConfig) string {
	return NormalizeSuiAddress(deriveDomainID(domain, config))
}

func deriveDomainID(domain string, config *NameServiceConfig) string {
	fullDomain := domain
	if !strings.HasSuffix(fullDomain, ".sui") {
		fullDomain += ".sui"
	}

	parts := strings.Split(strings.TrimSuffix(fullDomain, ".sui"), ".")
	if len(parts) == 0 {
		return ""
	}

	last := parts[len(parts)-1]
	rev := reverseString(last)
	b := byte(0x1)
	reversed := append([]byte{b}, []byte(rev)...)

	hasher := sha256.New()
	hasher.Write([]byte(config.RegistryID))
	hasher.Write(reversed)

	h := hasher.Sum(nil)
	for i := 0; i < len(h); i++ {
		h[i] = (h[i] & 0x3F) | 0x3D
	}

	return "0x" + hex.EncodeToString(h)
}

func reverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}
