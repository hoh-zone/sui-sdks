"""Sui utilities module - formatting and validation functions."""

from typing import Optional
import re


ELLIPSIS = "\u2026"
SUI_ADDRESS_LENGTH = 32
SUI_DECIMALS = 9
MIST_PER_SUI = 1_000_000_000
MOVE_STDLIB_ADDRESS = "0x2"
SUI_FRAMEWORK_ADDRESS = "0x3"
SUI_SYSTEM_ADDRESS = "0x5"
SUI_CLOCK_OBJECT_ID = "0x6"
SUI_SYSTEM_MODULE_NAME = "sui_system"
SUI_TYPE_ARG = "0x2::tx_context::TxContext"
SUI_SYSTEM_STATE_OBJECT_ID = "0x5"
SUI_RANDOM_OBJECT_ID = "0x8"
SUI_DENY_LIST_OBJECT_ID = "0xb"
NAME_SEPARATOR = "/"
MAX_APP_SIZE = 64


def format_address(address: str) -> str:
    """Format an address for display (add ellipsis in the middle)."""
    if len(address) <= 6:
        return address
    
    offset = 0
    if address.startswith("0x"):
        offset = 2
    
    if len(address) < offset + 4:
        return address
    
    prefix = "0x" + address[offset:offset + 4] + ELLIPSIS
    if len(address) > 4:
        return prefix + address[-4:]
    return prefix


def format_digest(digest: str) -> str:
    """Format a digest for display (add ellipsis)."""
    if len(digest) < 10:
        return digest
    
    return digest[:10] + ELLIPSIS


def is_valid_sui_address(address: str) -> bool:
    """Check if an address is a valid Sui address."""
    addr = address.strip().lower()
    addr = addr.removeprefix("0x")
    
    if len(addr) == 0 or len(addr) > 64:
        return False
    
    try:
        int(addr, 16)
        return True
    except ValueError:
        return False


def is_valid_sui_object_id(object_id: str) -> bool:
    """Check if an object ID is valid (same as address)."""
    return is_valid_sui_address(object_id)


def is_valid_transaction_digest(digest: str) -> bool:
    """Check if a transaction digest is valid hex string."""
    digest = digest.strip().removeprefix("0x")
    
    if len(digest) == 0 or len(digest) > 64:
        return False
    
    for c in digest:
        if not (('0' <= c <= '9') or ('a' <= c <= 'f')):
            return False
    
    return True


def normalize_sui_address(address: str) -> str:
    """Normalize a Sui address to the canonical format."""
    addr = address.strip().lower()
    addr = addr.removeprefix("0x")
    
    if len(addr) > 64:
        addr = addr[-64:]
    elif len(addr) < 64:
        addr = addr.zfill(64, '0')
    
    return "0x" + addr


def normalize_sui_object_id(object_id: str) -> str:
    """Normalize an object ID to the canonical format."""
    return normalize_sui_address(object_id)


def normalize_struct_tag(tag: str) -> str:
    """Normalize a struct tag to the canonical format."""
    tag = tag.strip()
    
    parts = tag.split("::")
    if len(parts) >= 2:
        module = parts[1].split("<")[0]
        name = parts[-1].split("<")[0]
        return f"{parts[0]}::{module}::{name}"
    
    return normalize_sui_address(tag)


def parse_struct_tag(tag: str) -> Optional[dict]:
    """Parse a struct tag into its components."""
    parts = tag.split("::")
    if len(parts) < 3:
        return None
    
    return {
        "address": normalize_sui_address(parts[0]),
        "module": parts[1].split("<")[0],
        "name": parts[-1].split("<")[0],
    }