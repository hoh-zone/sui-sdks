package utils

func DeriveObjectID(parentId string, typeTag interface{}, key []byte) string {
	typeTagStr := ""
	switch v := typeTag.(type) {
	case string:
		typeTagStr = v
	}

	derivedObjectTypeTag := "0x2::derived_object::DerivedObjectKey<" + typeTagStr + ">"

	return DeriveDynamicFieldID(parentId, derivedObjectTypeTag, key)
}
