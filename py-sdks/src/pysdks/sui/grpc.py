"""gRPC compatibility client with pluggable transport.

Current baseline defaults to JSON-RPC transport while preserving a gRPC-oriented
client surface. Native gRPC transport uses official Google packages (`grpcio`
and `protobuf`) when available.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Protocol

from .jsonrpc import JsonRpcClient


@dataclass
class GrpcRequest:
    method: str
    params: List[Any] = field(default_factory=list)
    metadata: Optional[Dict[str, str]] = None


@dataclass
class GrpcResponse:
    result: Any = None
    error: Optional[Any] = None
    raw: Optional[Dict[str, Any]] = None


class GrpcTransport(Protocol):
    def unary(self, request: GrpcRequest) -> GrpcResponse:
        ...


@dataclass
class JsonRpcGrpcTransport:
    client: Any

    @classmethod
    def from_endpoint(cls, endpoint: str) -> "JsonRpcGrpcTransport":
        return cls(client=JsonRpcClient(endpoint))

    def unary(self, request: GrpcRequest) -> GrpcResponse:
        payload = self.client.call(request.method, request.params)

        if isinstance(payload, dict) and "error" in payload:
            return GrpcResponse(error=payload["error"], raw=payload)

        if isinstance(payload, dict) and "result" in payload:
            return GrpcResponse(result=payload["result"], raw=payload)

        return GrpcResponse(result=payload, raw=payload if isinstance(payload, dict) else None)


@dataclass
class GrpcNativeTransport:
    target: str
    service: str = "sui.rpc.v2.Service"
    timeout_sec: float = 30.0
    _grpc: Optional[Any] = None
    _channel: Optional[Any] = None
    _struct_cls: Optional[Any] = None
    _parse_dict: Optional[Any] = None
    _message_to_dict: Optional[Any] = None

    def __post_init__(self):
        if self._grpc is None:
            try:
                import grpc as grpc_module  # type: ignore
            except ImportError as e:  # pragma: no cover - exercised via unit test path
                raise RuntimeError("grpc native transport requires 'grpcio'") from e
            self._grpc = grpc_module

        if self._channel is None:
            self._channel = self._grpc.insecure_channel(self.target)

    def _ensure_protobuf(self) -> None:
        if self._struct_cls is not None and self._parse_dict is not None and self._message_to_dict is not None:
            return

        try:
            from google.protobuf.json_format import MessageToDict, ParseDict  # type: ignore
            from google.protobuf.struct_pb2 import Struct  # type: ignore
        except ImportError as e:  # pragma: no cover - exercised via unit test path
            raise RuntimeError("grpc native transport requires 'protobuf'") from e

        self._struct_cls = Struct
        self._parse_dict = ParseDict
        self._message_to_dict = MessageToDict

    def unary(self, request: GrpcRequest) -> GrpcResponse:
        self._ensure_protobuf()

        method_path = self._method_path(request.method)
        metadata = list((request.metadata or {}).items())

        request_msg = self._struct_cls()
        self._parse_dict({"method": request.method, "params": request.params}, request_msg)

        try:
            unary_callable = self._channel.unary_unary(
                method_path,
                request_serializer=lambda msg: msg.SerializeToString(),
                response_deserializer=lambda b: self._struct_cls.FromString(b),
            )
            response_msg = unary_callable(request_msg, metadata=metadata, timeout=self.timeout_sec)
        except Exception as e:
            return GrpcResponse(error={"message": str(e)})

        parsed = self._message_to_dict(response_msg, preserving_proto_field_name=True)

        if isinstance(parsed, dict) and "error" in parsed:
            return GrpcResponse(error=parsed["error"], raw=parsed)
        if isinstance(parsed, dict) and "result" in parsed:
            return GrpcResponse(result=parsed["result"], raw=parsed)
        return GrpcResponse(result=parsed, raw=parsed if isinstance(parsed, dict) else None)

    def _method_path(self, method: str) -> str:
        if method.startswith("/"):
            return method
        return f"/{self.service}/{method}"


@dataclass
class SuiGrpcClient:
    endpoint: str
    _transport: Optional[GrpcTransport] = None

    @classmethod
    def from_native_grpc(
        cls,
        target: str,
        service: str = "sui.rpc.v2.Service",
        timeout_sec: float = 30.0,
        grpc_module: Optional[Any] = None,
        channel: Optional[Any] = None,
    ) -> "SuiGrpcClient":
        transport = GrpcNativeTransport(
            target=target,
            service=service,
            timeout_sec=timeout_sec,
            _grpc=grpc_module,
            _channel=channel,
        )
        return cls(endpoint=target, _transport=transport)

    def __post_init__(self):
        if self._transport is None:
            self._transport = JsonRpcGrpcTransport.from_endpoint(self.endpoint)

    def unary(self, request: GrpcRequest) -> GrpcResponse:
        response = self._transport.unary(request)
        if response.error is not None:
            raise RuntimeError(f"grpc transport error: {response.error}")
        return response

    def call(self, method: str, params: Optional[List[Any]] = None) -> Dict[str, Any]:
        # Compatibility shim with existing placeholder API.
        response = self.unary(GrpcRequest(method=self._map_method(method), params=params or []))
        if response.raw is not None:
            return response.raw
        return {"result": response.result}

    def batch(self, requests: List[GrpcRequest]) -> List[GrpcResponse]:
        return [self.unary(req) for req in requests]

    def get_latest_checkpoint_sequence_number(self) -> Dict[str, Any]:
        return self.call("sui_getLatestCheckpointSequenceNumber")

    def get_object(self, object_id: str, options: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self.call("sui_getObject", [object_id, options or {}])

    def get_objects(self, object_ids: List[str], options: Optional[Dict[str, Any]] = None) -> List[GrpcResponse]:
        reqs = [GrpcRequest(method="sui_getObject", params=[obj_id, options or {}]) for obj_id in object_ids]
        return self.batch(reqs)

    def execute_transaction_block(
        self,
        tx_bytes_b64: str,
        signatures: List[str],
        options: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        return self.call("sui_executeTransactionBlock", [tx_bytes_b64, signatures, options or {}])

    def _map_method(self, method: str) -> str:
        # Placeholder mapping for future grpc-native method translation.
        return method


GrpcCoreClient = SuiGrpcClient
