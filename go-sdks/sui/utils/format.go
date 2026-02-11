package utils

import "strings"

func FormatAddress(address string) string {
	if len(address) <= 6 {
		return address
	}

	offset := 0
	if strings.HasPrefix(address, "0x") {
		offset = 2
	}

	if len(address) < offset+4 {
		return address
	}

	var prefix string
	var middle string
	var suffix string

	prefix = "0x" + address[offset:offset+4]
	ellipsis := "\u2026"
	middle = ellipsis
	suffix = address[len(address)-4:]

	if len(address) > 4 {
		return prefix + middle + suffix
	}

	return prefix + middle
}

func FormatDigest(digest string) string {
	if len(digest) < 10 {
		return digest
	}

	ellipsis := "\u2026"
	if len(digest) >= 10 {
		return digest[:10] + ellipsis
	}
	return digest + ellipsis
}

func IsValidTransactionDigest(digest string) bool {
	digest = strings.TrimSpace(strings.TrimPrefix(digest, "0x"))
	if len(digest) == 0 || len(digest) > 64 {
		return false
	}

	for _, c := range digest {
		if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
			return false
		}
	}

	return true
}

func NormalizeSuiObjectId(id string) string {
	return NormalizeSuiAddress(id)
}

func IsValidSuiObjectId(id string) bool {
	return IsValidSuiAddress(id)
}
