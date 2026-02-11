package passkey

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"math/big"

	"github.com/sui-sdks/go-sdks/sui/cryptography"
)

const (
	PublicKeySize             = 33
	UncompressedPublicKeySize = 65
	SignatureSizeCompact      = 64
)

var Secp256r1SPKIHeader = []byte{
	0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01,
	0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00,
}

type PublicKey struct{ data []byte }

func NewPublicKey(value []byte) (*PublicKey, error) {
	if len(value) != PublicKeySize {
		return nil, fmt.Errorf("invalid public key size: %d", len(value))
	}
	return &PublicKey{data: append([]byte(nil), value...)}, nil
}

func NewPublicKeyFromBase64(value string) (*PublicKey, error) {
	b, err := base64.StdEncoding.DecodeString(value)
	if err != nil {
		return nil, err
	}
	return NewPublicKey(b)
}

func ParseDerSPKI(der []byte) ([]byte, error) {
	expectedLen := len(Secp256r1SPKIHeader) + UncompressedPublicKeySize
	if len(der) != expectedLen {
		return nil, fmt.Errorf("invalid DER length: %d", len(der))
	}
	for i := range Secp256r1SPKIHeader {
		if der[i] != Secp256r1SPKIHeader[i] {
			return nil, fmt.Errorf("invalid SPKI header")
		}
	}
	if der[len(Secp256r1SPKIHeader)] != 0x04 {
		return nil, fmt.Errorf("invalid point marker")
	}
	return append([]byte(nil), der[len(Secp256r1SPKIHeader):]...), nil
}

func BuildAssertionMessage(authenticatorData, clientDataJSON []byte) []byte {
	hash := sha256.Sum256(clientDataJSON)
	out := make([]byte, 0, len(authenticatorData)+len(hash))
	out = append(out, authenticatorData...)
	out = append(out, hash[:]...)
	return out
}

func (p *PublicKey) ToRawBytes() []byte { return append([]byte(nil), p.data...) }
func (p *PublicKey) Flag() byte {
	return cryptography.SignatureSchemeToFlag[cryptography.SchemePasskey]
}

// Verify validates a compact r||s passkey/secp256r1 signature against data.
func (p *PublicKey) Verify(data, signature []byte) bool {
	if len(signature) != SignatureSizeCompact {
		return false
	}
	x, y := elliptic.UnmarshalCompressed(elliptic.P256(), p.data)
	if x == nil {
		return false
	}
	pub := ecdsa.PublicKey{Curve: elliptic.P256(), X: x, Y: y}
	return ecdsa.Verify(&pub, data, bytesToBig(signature[:32]), bytesToBig(signature[32:]))
}

func (p *PublicKey) ToSuiAddress() string { return cryptography.ToSuiAddress(p) }

func bytesToBig(b []byte) *big.Int {
	return new(big.Int).SetBytes(b)
}
