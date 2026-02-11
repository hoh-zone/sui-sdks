package passkey

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/sha256"
	"testing"
)

type testProvider struct {
	prv *ecdsa.PrivateKey
}

func (p *testProvider) SignAssertion(challenge []byte) ([]byte, []byte, []byte, error) {
	authData := []byte("auth-data")
	clientDataJSON := []byte(`{"type":"webauthn.get"}`)
	msg := BuildAssertionMessage(authData, clientDataJSON)
	h := sha256.Sum256(append(msg, challenge...))
	r, s, err := ecdsa.Sign(rand.Reader, p.prv, h[:])
	if err != nil {
		return nil, nil, nil, err
	}
	sig := make([]byte, SignatureSizeCompact)
	rb, sb := r.Bytes(), s.Bytes()
	copy(sig[32-len(rb):32], rb)
	copy(sig[64-len(sb):], sb)
	return authData, clientDataJSON, sig, nil
}

func TestPublicKeyAndSignWithIntent(t *testing.T) {
	prv, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("generate key failed: %v", err)
	}
	compressed := elliptic.MarshalCompressed(elliptic.P256(), prv.PublicKey.X, prv.PublicKey.Y)
	pk, err := NewPublicKey(compressed)
	if err != nil {
		t.Fatalf("new public key failed: %v", err)
	}
	kp := NewKeypair(pk, &testProvider{prv: prv})
	if _, err := kp.SignTransaction([]byte("hello")); err != nil {
		t.Fatalf("sign transaction failed: %v", err)
	}
}
