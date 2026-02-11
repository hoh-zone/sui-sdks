package utils

import (
	"regexp"
	"strings"
)

const (
	NAME_SEPARATOR = "/"
	MAX_APP_SIZE   = 64
)

var namePattern = regexp.MustCompile(`^[a-z0-9]+(?:-[a-z0-9]+)*$`)
var versionRegex = regexp.MustCompile(`^\d+$`)

func IsValidNamedPackage(name string) bool {
	parts := strings.Split(name, NAME_SEPARATOR)

	if len(parts) < 2 || len(parts) > 3 {
		return false
	}

	org := parts[0]
	app := parts[1]
	version := ""
	if len(parts) == 3 {
		version = parts[2]
	}

	if version != "" && !versionRegex.MatchString(version) {
		return false
	}

	if !isValidSuiNSName(org) {
		return false
	}

	return namePattern.MatchString(app) && len(app) < MAX_APP_SIZE
}

func isValidSuiNSName(name string) bool {
	if len(name) == 0 || len(name) > 64 {
		return false
	}
	for _, c := range name {
		if !((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
			return false
		}
	}
	return true
}

func IsValidNamedType(typeStr string) bool {
	splitType := strings.FieldsFunc(typeStr, func(c rune) bool {
		return c == ':' || c == '<' || c == '>' || c == ','
	})

	for _, t := range splitType {
		if strings.Contains(t, NAME_SEPARATOR) && !IsValidNamedPackage(t) {
			return false
		}
	}

	return true
}

func NormalizeTypeTag(tag string) string {
	return NormalizeStructTag(tag)
}
