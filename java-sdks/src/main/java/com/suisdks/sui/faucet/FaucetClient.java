package com.suisdks.sui.faucet;

import com.suisdks.sui.jsonrpc.JsonRpcClient;
import com.suisdks.sui.jsonrpc.JsonRpcError;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.URL;
import java.util.List;
import java.util.Map;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.JsonNode;

public class FaucetClient {
    private final HttpClient httpClient;
    private final String faucetHost;

    public FaucetClient(String faucetHost) {
        this.faucetHost = faucetHost;
        this.httpClient = HttpClient.newHttpClient();
    }

    public static FaucetClient forTestnet() {
        return new FaucetClient("https://faucet.testnet.sui.io");
    }

    public static FaucetClient forDevnet() {
        return new FaucetClient("https://faucet.devnet.sui.io");
    }

    public static FaucetClient forLocalnet() {
        return new FaucetClient("http://127.0.0.1:9123");
    }

    public static String getFaucetHost(String network) {
        return switch (network.toLowerCase()) {
            case "testnet" -> "https://faucet.testnet.sui.io";
            case "devnet" -> "https://faucet.devnet.sui.io";
            case "localnet" -> "http://127.0.0.1:9123";
            default -> throw new IllegalArgumentException("Unsupported network: " + network);
        };
    }

    public Map<String, Object> requestSuiFromFaucet(String address) throws IOException, InterruptedException {
        return requestSuiFromFaucet(address, false);
    }

    public Map<String, Object> requestSuiFromFaucet(String address, boolean silent) throws IOException, InterruptedException {
        String url = faucetHost + "/gas";

        try {
            HttpRequest request = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .header("Content-Type", "application/json")
                    .POST(HttpRequest.BodyPublishers.ofString("{\"FixedAmountRequest\":{\"recipient\":\"" + address + "\"}}"))
                    .build();

            HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

            if (response.statusCode() == 429) {
                FaucetRateLimitError error = new FaucetRateLimitError();
                if (!silent) throw error;
                return Map.of("error", "Rate limited");
            }

            if (response.statusCode() != 200) {
                if (!silent) throw new IOException("Faucet request failed: " + response.body());
                return Map.of("error", response.body());
            }

            ObjectMapper mapper = new ObjectMapper();
            JsonNode json = mapper.readTree(response.body());

            String transferredSui = json.has("transferredSui") ? json.get("transferredSui").asText() : "0";
            String taskId = json.has("taskId") ? json.get("taskId").asText() : "";

            return Map.of(
                    "transferredSui", transferredSui,
                    "taskId", taskId
            );
        } catch (IOException | InterruptedException e) {
            if (!silent) throw e;
            return Map.of("error", e.getMessage());
        }
    }

    public Map<String, Object> requestSuiFromFaucetV2(String address) throws IOException, InterruptedException {
        return requestSuiFromFaucetV2(address, false);
    }

    public Map<String, Object> requestSuiFromFaucetV2(String address, boolean silent) throws IOException, InterruptedException {
        String url = faucetHost + "/v1/gas";

        try {
            HttpRequest request = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .header("Content-Type", "application/json")
                    .POST(HttpRequest.BodyPublishers.ofString("{\"recipient\":\"" + address + "\"}"))
                    .build();

            HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

            if (response.statusCode() == 429) {
                FaucetRateLimitError error = new FaucetRateLimitError();
                if (!silent) throw error;
                return Map.of("error", "Rate limited");
            }

            if (response.statusCode() != 200) {
                if (!silent) throw new IOException("Faucet request failed: " + response.body());
                return Map.of("error", response.body());
            }

            ObjectMapper mapper = new ObjectMapper();
            JsonNode json = mapper.readTree(response.body());

            String taskId = json.has("task") ? json.get("task").get("id").asText() : "";

            return Map.of("taskId", taskId);
        } catch (IOException | InterruptedException e) {
            if (!silent) throw e;
            return Map.of("error", e.getMessage());
        }
    }

    public Map<String, Object> getFaucetStatus(String taskId) throws IOException, InterruptedException {
        String url = faucetHost + "/v1/status/" + taskId;

        try {
            HttpRequest request = HttpRequest.newBuilder()
                    .uri(URI.create(url))
                    .header("Content-Type", "application/json")
                    .GET()
                    .build();

            HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

            if (response.statusCode() != 200) {
                throw new IOException("Faucet status check failed: " + response.body());
            }

            ObjectMapper mapper = new ObjectMapper();
            JsonNode json = mapper.readTree(response.body());

            return Map.of(
                    "status", json.get("status").asText(),
                    "task", json.toPrettyString()
            );
        } catch (IOException | InterruptedException e) {
            throw e;
        }
    }

    public static class FaucetRateLimitError extends RuntimeException {
        private static final String MESSAGE = "Rate limited. Try again later.";

        public FaucetRateLimitError() {
            super(MESSAGE);
        }

        public FaucetRateLimitError(String message) {
            super(message);
        }

        public String getMessage() {
            return MESSAGE;
        }
    }
}