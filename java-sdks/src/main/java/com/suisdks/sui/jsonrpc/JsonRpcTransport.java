package com.suisdks.sui.jsonrpc;

import java.util.List;
import java.util.Map;

public interface JsonRpcTransport {
    Map<String, Object> request(String method, List<Object> params);
}
