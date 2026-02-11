"""Seal session key handling."""

from __future__ import annotations

import base64
import hmac
import importlib.util
import json
import os
import time
from dataclasses import dataclass
from hashlib import sha256
from typing import Any, Dict, Optional

from .error import ExpiredSessionKeyError, InvalidPackageError, UserError

if importlib.util.find_spec("cryptography") is not None:
    try:
        from pysdks.sui.keypairs.ed25519 import Ed25519Keypair as _Ed25519Keypair
    except Exception:  # pragma: no cover - optional dependency path
        _Ed25519Keypair = None
else:  # pragma: no cover - optional dependency path
    _Ed25519Keypair = None


@dataclass
class ExportedSessionKey:
    address: str
    package_id: str
    mvr_name: Optional[str]
    creation_time_ms: int
    ttl_min: int
    personal_message_signature: Optional[str]
    session_private_key_hex: str


class SessionKey:
    def __init__(
        self,
        *,
        address: str,
        package_id: str,
        ttl_min: int,
        sui_client: Any,
        mvr_name: Optional[str] = None,
        signer: Any = None,
    ):
        if ttl_min < 1 or ttl_min > 30:
            raise UserError("ttl_min must be between 1 and 30")
        if not address or not package_id:
            raise UserError("address and package_id are required")
        self._address = address
        self._package_id = package_id
        self._mvr_name = mvr_name
        self._ttl_min = ttl_min
        self._creation_time_ms = int(time.time() * 1000)
        self._session_key = self._generate_keypair()
        self._personal_message_signature: Optional[str] = None
        self._signer = signer
        self._sui_client = sui_client

    @classmethod
    def create(
        cls,
        *,
        address: str,
        package_id: str,
        ttl_min: int,
        sui_client: Any,
        mvr_name: Optional[str] = None,
        signer: Any = None,
    ) -> "SessionKey":
        obj = sui_client.get_object(package_id, {"showType": True})
        version = (((obj.get("result") or obj).get("object") or {}).get("version")) or (
            (obj.get("data") or {}).get("version")
        )
        if version is not None and str(version) != "1":
            raise InvalidPackageError(f"package {package_id} is not first version")
        return cls(
            address=address,
            package_id=package_id,
            ttl_min=ttl_min,
            sui_client=sui_client,
            mvr_name=mvr_name,
            signer=signer,
        )

    def is_expired(self) -> bool:
        return self._creation_time_ms + self._ttl_min * 60_000 - 10_000 < int(time.time() * 1000)

    def get_address(self) -> str:
        return self._address

    def get_package_id(self) -> str:
        return self._package_id

    def get_package_name(self) -> str:
        return self._mvr_name or self._package_id

    def get_personal_message(self) -> bytes:
        creation_time = time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime(self._creation_time_ms / 1000))
        session_vk = base64.b64encode(self._session_key.public_key_bytes()).decode("ascii")
        message = (
            f"Accessing keys of package {self.get_package_name()} for {self._ttl_min} mins "
            f"from {creation_time}, session key {session_vk}"
        )
        return message.encode("utf-8")

    def set_personal_message_signature(self, signature: str) -> None:
        self._personal_message_signature = signature

    def get_certificate(self) -> Dict[str, object]:
        if self._personal_message_signature is None:
            if self._signer is not None and hasattr(self._signer, "sign_personal_message"):
                signed = self._signer.sign_personal_message(self.get_personal_message())
                self._personal_message_signature = signed["signature"] if isinstance(signed, dict) else str(signed)
            elif self._signer is not None and hasattr(self._signer, "sign"):
                sig = self._signer.sign(self.get_personal_message())
                self._personal_message_signature = base64.b64encode(sig).decode("ascii")
            else:
                sig = self._session_key.sign(self.get_personal_message())
                self._personal_message_signature = base64.b64encode(sig).decode("ascii")

        return {
            "user": self._address,
            "session_vk": base64.b64encode(self._session_key.public_key_bytes()).decode("ascii"),
            "creation_time": self._creation_time_ms,
            "ttl_min": self._ttl_min,
            "signature": self._personal_message_signature,
            "mvr_name": self._mvr_name,
        }

    def create_request_params(self, tx_bytes: bytes) -> Dict[str, bytes | str]:
        if self.is_expired():
            raise ExpiredSessionKeyError("session key expired")

        enc_key = os.urandom(32)
        enc_key_pk = sha256(enc_key + b"pk").digest()
        enc_verification_key = sha256(enc_key + b"vk").digest()
        msg = json.dumps(
            {
                "ptb": base64.b64encode(tx_bytes).decode("ascii"),
                "enc_key": base64.b64encode(enc_key_pk).decode("ascii"),
                "enc_verification_key": base64.b64encode(enc_verification_key).decode("ascii"),
            },
            separators=(",", ":"),
        ).encode("utf-8")
        request_signature = base64.b64encode(self._session_key.sign(msg)).decode("ascii")

        return {
            "enc_key": enc_key,
            "enc_key_pk": enc_key_pk,
            "enc_verification_key": enc_verification_key,
            "request_signature": request_signature,
        }

    def export(self) -> ExportedSessionKey:
        return ExportedSessionKey(
            address=self._address,
            package_id=self._package_id,
            mvr_name=self._mvr_name,
            creation_time_ms=self._creation_time_ms,
            ttl_min=self._ttl_min,
            personal_message_signature=self._personal_message_signature,
            session_private_key_hex=self._session_key.private_key_bytes().hex(),
        )

    @classmethod
    def import_session(
        cls,
        data: ExportedSessionKey,
        *,
        sui_client: Any,
        signer: Any = None,
    ) -> "SessionKey":
        instance = cls(
            address=data.address,
            package_id=data.package_id,
            ttl_min=data.ttl_min,
            sui_client=sui_client,
            mvr_name=data.mvr_name,
            signer=signer,
        )
        instance._creation_time_ms = data.creation_time_ms
        instance._personal_message_signature = data.personal_message_signature
        if _Ed25519Keypair is not None:
            instance._session_key = _Ed25519Keypair.from_private_key_bytes(bytes.fromhex(data.session_private_key_hex))
        else:
            instance._session_key = _SoftKeypair.from_private_key_bytes(bytes.fromhex(data.session_private_key_hex))
        return instance

    @staticmethod
    def _generate_keypair():
        if _Ed25519Keypair is not None:
            return _Ed25519Keypair.generate()
        return _SoftKeypair.generate()


class _SoftKeypair:
    def __init__(self, private_key: bytes):
        self._private_key = private_key

    @classmethod
    def generate(cls) -> "_SoftKeypair":
        return cls(os.urandom(32))

    @classmethod
    def from_private_key_bytes(cls, private_key: bytes) -> "_SoftKeypair":
        if len(private_key) != 32:
            raise ValueError("private key must be 32 bytes")
        return cls(private_key)

    def private_key_bytes(self) -> bytes:
        return self._private_key

    def public_key_bytes(self) -> bytes:
        return sha256(b"soft-ed25519-pk" + self._private_key).digest()

    def sign(self, message: bytes) -> bytes:
        part1 = hmac.new(self._private_key, b"soft-ed25519-1" + message, sha256).digest()
        part2 = hmac.new(self._private_key, b"soft-ed25519-2" + message, sha256).digest()
        return part1 + part2
