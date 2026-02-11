package transactions

import "testing"

func TestNamedPackagesPluginReplacesNames(t *testing.T) {
	tx := NewTransaction()
	tx.MoveCall("mysten/sui::coin::transfer", []Argument{}, []string{"mysten/sui::coin::Coin"})

	p := NewNamedPackagesPlugin(map[string]string{
		"mysten/sui": "0x2",
	})
	if err := p.Build(tx); err != nil {
		t.Fatalf("build failed: %v", err)
	}
	cmd := tx.GetData().Commands[0]
	move := cmd["MoveCall"].(map[string]any)
	if move["package"] != "0x2" {
		t.Fatalf("expected package replaced, got %v", move["package"])
	}
	typeArgs := move["typeArguments"].([]string)
	if typeArgs[0] != "0x2::coin::Coin" {
		t.Fatalf("expected type arg replaced, got %v", typeArgs[0])
	}
}
