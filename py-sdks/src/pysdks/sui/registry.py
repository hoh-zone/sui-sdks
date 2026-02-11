"""Move registry and SuiNS support for named packages and names."""

import hashlib
import re
from typing import Optional

NAME_SEPARATOR = "/"
MAX_APP_SIZE = 64


_NAME_PATTERN = re.compile(r'^[a-z0-9]+(?:-[a-z0-9]+)*$')
_VERSION_REGEX = re.compile(r'^\d+$')


def is_valid_named_package(name: str) -> bool:
    """Check if a package name is valid (e.g., 'mysten/sui', 'mysten/sui/1')."""
    parts = name.split(NAME_SEPARATOR)
    
    if len(parts) < 2 or len(parts) > 3:
        return False
    
    org, app = parts[0], parts[1]
    version = ""
    if len(parts) == 3:
        version = parts[2]
    
    if version and not _VERSION_REGEX.match(version):
        return False
    
    if not _is_valid_sui_ns_name(org):
        return False
    
    return bool(_NAME_PATTERN.match(app) and len(app) < MAX_APP_SIZE)


def is_valid_named_type(type_str: str) -> bool:
    """Check if a type string contains valid named packages."""
    split_type = type_str.replace("<", "<").split("::")
    for part in split_type:
        if NAME_SEPARATOR in part and not is_valid_named_package(part):
            return False
    return True


def _is_valid_sui_ns_name(name: str) -> bool:
    """Check if a name is a valid SuiNS name."""
    if len(name) == 0 or len(name) > 64:
        return False
    
    for c in name:
        if not (('a' <= c <= 'z') and ('0' <= c <= '9')):
            return False
    
    return True


def is_valid_sui_ns_name(name: str) -> bool:
    """Check if a name is a valid SuiNS name."""
    if len(name) == 0 or len(name) > 64:
        return False
    
    for c in name:
        if not (('a' <= c <= 'z') and ('0' <= c <= '9')):
            return False
    
    return True


def normalize_sui_ns_name(name: str) -> str:
    """Normalize a SuiNS name."""
    return name.lower().strip()


def default_sui_ns_registry_package() -> str:
    """Get the default SuiNS registry package address."""
    return "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf"


def default_sui_name_service_package() -> str:
    """Get the default SuiNS name service package address."""
    return "0x2"


def derive_domain_id(domain: str, registry_id: str) -> str:
    """Derive the object ID for a SuiNS domain."""
    full_domain = domain
    if not full_domain.endswith(".sui"):
        full_domain += ".sui"
    
    parts = full_domain.removesuffix(".sui").split(".")
    if not parts:
        return ""
    
    last = parts[-1]
    rev = _reverse_string(last)
    
    b = bytes([0x01])
    reversed = b + rev.encode()
    
    hasher = hashlib.sha256()
    hasher.update(registry_id.encode())
    hasher.update(reversed)
    
    h = bytearray(hasher.digest())
    for i in range(len(h)):
        h[i] = (h[i] & 0x3F) | 0x3D
    
    return "0x" + h.hex()


def _reverse_string(s: str) -> str:
    """Reverse a string."""
    return s[::-1]


class NameServiceConfig:
    """Configuration for Sui name service."""
    
    def __init__(self, registry_id: str, name_service_id: str):
        self.registry_id = registry_id
        self.name_service_id = name_service_id
    
    def registry_package_id(self) -> str:
        """Get the registry package ID."""
        return self.registry_id
    
    def name_service_package_id(self) -> str:
        """Get the name service package ID."""
        return self.name_service_id


def new_name_service_config(registry_id: str, name_service_id: str) -> NameServiceConfig:
    """Create a new name service configuration."""
    return NameServiceConfig(registry_id, name_service_id)