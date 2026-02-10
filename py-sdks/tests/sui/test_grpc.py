import json
import unittest

from pysdks.sui.grpc import (
    GrpcNativeTransport,
    GrpcRequest,
    GrpcResponse,
    JsonRpcGrpcTransport,
    SuiGrpcClient,
)


class _MockJsonRpcClient:
    def __init__(self):
        self.calls = []

    def call(self, method, params=None):
        self.calls.append((method, params or []))
        return {"result": {"method": method, "params": params or []}}


class _ErrorJsonRpcClient:
    def call(self, method, params=None):
        return {"error": {"code": -1, "message": "boom", "method": method}}


class _MockTransport:
    def __init__(self):
        self.calls = []

    def unary(self, request: GrpcRequest) -> GrpcResponse:
        self.calls.append(request)
        return GrpcResponse(result={"ok": True, "method": request.method}, raw={"result": {"ok": True}})


class _FakeStruct:
    def __init__(self, payload=None):
        self.payload = payload or {}

    def SerializeToString(self):
        return json.dumps(self.payload).encode("utf-8")

    @classmethod
    def FromString(cls, data):
        return cls(payload=json.loads(data.decode("utf-8")))


def _fake_parse_dict(value, message):
    message.payload = value
    return message


def _fake_message_to_dict(message, preserving_proto_field_name=False):
    return message.payload


class _FakeChannel:
    def __init__(self):
        self.path = None
        self.last_payload = None
        self.last_metadata = None

    def unary_unary(self, path, request_serializer=None, response_deserializer=None):
        self.path = path

        def _call(payload, metadata=None, timeout=None):
            self.last_payload = payload
            self.last_metadata = metadata
            body = {"result": {"ok": True, "echo_method": payload.payload.get("method")}}
            if response_deserializer:
                return response_deserializer(json.dumps(body).encode("utf-8"))
            return _FakeStruct(body)

        return _call


class _FakeGrpcModule:
    def __init__(self, channel):
        self.channel = channel
        self.targets = []

    def insecure_channel(self, target):
        self.targets.append(target)
        return self.channel


class TestGrpcPlaceholder(unittest.TestCase):
    def test_core_methods(self):
        mock = _MockJsonRpcClient()
        transport = JsonRpcGrpcTransport(client=mock)
        c = SuiGrpcClient(endpoint="https://example.invalid", _transport=transport)

        c.get_latest_checkpoint_sequence_number()
        c.get_object("0x1", {"showContent": True})
        c.execute_transaction_block("AA==", ["sig1"], {"showEffects": True})

        self.assertEqual("sui_getLatestCheckpointSequenceNumber", mock.calls[0][0])
        self.assertEqual("sui_getObject", mock.calls[1][0])
        self.assertEqual("sui_executeTransactionBlock", mock.calls[2][0])

    def test_transport_unwraps_result(self):
        mock = _MockJsonRpcClient()
        transport = JsonRpcGrpcTransport(client=mock)
        response = transport.unary(GrpcRequest(method="m", params=[1]))
        self.assertEqual({"method": "m", "params": [1]}, response.result)
        self.assertIsNone(response.error)

    def test_transport_error_raises_via_client(self):
        transport = JsonRpcGrpcTransport(client=_ErrorJsonRpcClient())
        c = SuiGrpcClient(endpoint="https://example.invalid", _transport=transport)
        with self.assertRaises(RuntimeError):
            c.call("sui_getObject", ["0x1", {}])

    def test_batch(self):
        c = SuiGrpcClient(endpoint="https://example.invalid", _transport=_MockTransport())
        responses = c.get_objects(["0x1", "0x2"], {"showType": True})
        self.assertEqual(2, len(responses))
        self.assertTrue(all(r.result["ok"] for r in responses))

    def test_native_transport_requires_grpc_module(self):
        with self.assertRaises(RuntimeError):
            GrpcNativeTransport(target="127.0.0.1:9000", _grpc=None)

    def test_native_transport_requires_protobuf_helpers(self):
        fake_channel = _FakeChannel()
        fake_grpc = _FakeGrpcModule(fake_channel)
        transport = GrpcNativeTransport(target="127.0.0.1:9000", _grpc=fake_grpc)
        with self.assertRaises(RuntimeError):
            transport.unary(GrpcRequest(method="GetObject", params=["0x2"]))

    def test_native_transport_unary(self):
        fake_channel = _FakeChannel()
        fake_grpc = _FakeGrpcModule(fake_channel)
        transport = GrpcNativeTransport(
            target="127.0.0.1:9000",
            _grpc=fake_grpc,
            _struct_cls=_FakeStruct,
            _parse_dict=_fake_parse_dict,
            _message_to_dict=_fake_message_to_dict,
        )

        resp = transport.unary(GrpcRequest(method="GetObject", params=["0x2"], metadata={"x-a": "1"}))
        self.assertEqual({"ok": True, "echo_method": "GetObject"}, resp.result)
        self.assertEqual("/sui.rpc.v2.Service/GetObject", fake_channel.path)
        self.assertEqual("GetObject", fake_channel.last_payload.payload.get("method"))
        self.assertEqual([("x-a", "1")], fake_channel.last_metadata)

    def test_client_from_native_grpc(self):
        fake_channel = _FakeChannel()
        fake_grpc = _FakeGrpcModule(fake_channel)
        transport = GrpcNativeTransport(
            target="127.0.0.1:9000",
            _grpc=fake_grpc,
            _struct_cls=_FakeStruct,
            _parse_dict=_fake_parse_dict,
            _message_to_dict=_fake_message_to_dict,
        )
        c = SuiGrpcClient(endpoint="127.0.0.1:9000", _transport=transport)
        out = c.call("GetObject", ["0x3"])
        self.assertEqual({"result": {"ok": True, "echo_method": "GetObject"}}, out)


if __name__ == "__main__":
    unittest.main()
