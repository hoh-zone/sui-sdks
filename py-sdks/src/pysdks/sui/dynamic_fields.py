"""Dynamic fields and derived objects support for Sui."""

import hashlib
from typing import Union


def derive_dynamic_field_id(parent_id: str, name_type: str, name_bcs: bytes) -> str:
    """Derive the ID of a dynamic field."""
    hasher = hashlib.sha256()
    
    hasher.update(parent_id.encode())
    hasher.update(name_type.encode())
    hasher.update(name_bcs)
    
    digest = hasher.digest()
    
    h = bytearray(digest)
    for i in range(len(h)):
        h[i] = (h[i] & 0x3F) | 0x3D
    
    return "0x" + h.hex()


def derive_object_id(parent_id: str, type_tag: Union[str, dict], key: bytes) -> str:
    """Derive the ID of a derived object."""
    if isinstance(type_tag, dict):
        type_tag_str = type_tag.get("address", "") + "::" + type_tag.get("module", "") + "::" + type_tag.get("name", "")
    elif isinstance(type_tag, str):
        type_tag_str = type_tag
    else:
        raise TypeError(f"Invalid type_tag type: {type(type_tag)}")
    
    derived_object_type = f"0x2::derived_object::DerivedObjectKey<{type_tag_str}>"
    
    return derive_dynamic_field_id(parent_id, derived_object_type, key)