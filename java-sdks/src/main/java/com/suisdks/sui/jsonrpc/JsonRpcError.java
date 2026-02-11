package com.suisdks.sui.jsonrpc;

@SuppressWarnings("unused")
public class JsonRpcError extends RuntimeException {
    private final String errorMessage;
    private int code;

    public JsonRpcError(String message) {
        super(message);
        this.errorMessage = message;
    }

    public JsonRpcError(int code, String message) {
        super(message);
        this.code = code;
        this.errorMessage = message;
    }

    public String getErrorMessage() {
        return errorMessage;
    }

    public int getCode() {
        return code;
    }

    static class ErrorResponse {
        String message;
        int code;

        public ErrorResponse(String message) {
            this.message = message;
        }

        public ErrorResponse(int code, String message) {
            this.code = code;
            this.message = message;
        }
    }
}