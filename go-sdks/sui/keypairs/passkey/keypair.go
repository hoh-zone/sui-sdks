package passkey

import (
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"fmt"

	"github.com/sui-sdks/go-sdks/sui/cryptography"
)

type Provider interface {
	SignAssertion(challenge []byte) (authenticatorData []byte, clientDataJSON []byte, compactSignature []byte, err error)
}

type Keypair struct {
	publicKey *PublicKey
	provider  Provider
}

type EncodedPasskeySignature struct {
	AuthenticatorData string `json:"authenticatorData"`
	ClientDataJSON    string `json:"clientDataJSON"`
	UserSignature     string `json:"userSignature"` // scheme-flagged r||s||pubkey
}

func NewKeypair(publicKey *PublicKey, provider Provider) *Keypair {
	return &Keypair{publicKey: publicKey, provider: provider}
}

func (k *Keypair) GetKeyScheme() cryptography.SignatureScheme { return cryptography.SchemePasskey }
func (k *Keypair) GetPublicKey() cryptography.PublicKey       { return k.publicKey }
func (k *Keypair) ToSuiAddress() string                       { return k.publicKey.ToSuiAddress() }
func (k *Keypair) GetSecretKey() string                       { return "" }

func (k *Keypair) Sign(bytes []byte) ([]byte, error) {
	if k.provider == nil {
		return nil, fmt.Errorf("passkey provider is required")
	}
	authData, clientDataJSON, sig, err := k.provider.SignAssertion(bytes)
	if err != nil {
		return nil, err
	}
	if len(sig) != SignatureSizeCompact {
		return nil, fmt.Errorf("invalid passkey signature size: %d", len(sig))
	}
	// userSignature = secp256r1-flag || compact-signature || passkey-public-key
	userSig := make([]byte, 1+SignatureSizeCompact+PublicKeySize)
	userSig[0] = cryptography.SignatureSchemeToFlag[cryptography.SchemeSecp256r1]
	copy(userSig[1:], sig)
	copy(userSig[1+SignatureSizeCompact:], k.publicKey.ToRawBytes())

	encoded := EncodedPasskeySignature{
		AuthenticatorData: base64.StdEncoding.EncodeToString(authData),
		ClientDataJSON:    base64.StdEncoding.EncodeToString(clientDataJSON),
		UserSignature:     base64.StdEncoding.EncodeToString(userSig),
	}
	return json.Marshal(encoded)
}

func (k *Keypair) SignWithIntent(bytes []byte, intent cryptography.IntentScope) (cryptography.SignatureWithBytes, error) {
	intentMsg := cryptography.MessageWithIntent(intent, bytes)
	digest := sha256.Sum256(intentMsg)
	sig, err := k.Sign(digest[:])
	if err != nil {
		return cryptography.SignatureWithBytes{}, err
	}
	// passkey signature carries its own envelope, prepend passkey scheme flag at serialization layer.
	out := make([]byte, 1+len(sig))
	out[0] = cryptography.SignatureSchemeToFlag[cryptography.SchemePasskey]
	copy(out[1:], sig)
	return cryptography.SignatureWithBytes{
		Bytes:     base64.StdEncoding.EncodeToString(bytes),
		Signature: base64.StdEncoding.EncodeToString(out),
	}, nil
}

func (k *Keypair) SignTransaction(bytes []byte) (cryptography.SignatureWithBytes, error) {
	return k.SignWithIntent(bytes, cryptography.IntentTransactionData)
}

func (k *Keypair) SignPersonalMessage(bytes []byte) (cryptography.SignatureWithBytes, error) {
	return k.SignWithIntent(bytes, cryptography.IntentPersonalMessage)
}

func ParseSerializedPasskeySignature(serialized string) (*EncodedPasskeySignature, error) {
	b, err := base64.StdEncoding.DecodeString(serialized)
	if err != nil {
		return nil, err
	}
	if len(b) == 0 || b[0] != cryptography.SignatureSchemeToFlag[cryptography.SchemePasskey] {
		return nil, fmt.Errorf("invalid passkey signature scheme")
	}
	var out EncodedPasskeySignature
	if err := json.Unmarshal(b[1:], &out); err != nil {
		return nil, err
	}
	return &out, nil
}
