package com.suisdks.sui.graphql;

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class SuiGraphQLClient implements GraphQLClient {
    private final HttpClient httpClient;
    private String endpoint;

    public SuiGraphQLClient(String endpoint) {
        this.endpoint = endpoint;
        this.httpClient = HttpClient.newHttpClient();
    }

    public static SuiGraphQLClient fromNetwork(String network) {
        String endpoint = switch (network.toLowerCase()) {
            case "mainnet" -> "https://sui-mainnet.mystenlabs.com/graphql";
            case "testnet" -> "https://sui-testnet.mystenlabs.com/graphql";
            case "devnet" -> "https://sui-devnet.mystenlabs.com/graphql";
            default -> throw new IllegalArgumentException("Unsupported network: " + network);
        };
        return new SuiGraphQLClient(endpoint);
    }

    @Override
    public Map<String, Object> query(String query, Map<String, Object> variables) {
        return executeRequest(query, variables, "query");
    }

    @Override
    public Map<String, Object> mutate(String mutation, Map<String, Object> variables) {
        return executeRequest(mutation, variables, "mutation");
    }

    @Override
    public String getEndpoint() {
        return endpoint;
    }

    @Override
    public void setEndpoint(String endpoint) {
        this.endpoint = endpoint;
    }

    private Map<String, Object> executeRequest(String operation, Map<String, Object> variables, String operationType) {
        Map<String, Object> body = new HashMap<>();
        body.put("query", operation);
        if (variables != null) body.put("variables", variables);

        try {
            HttpRequest request = HttpRequest.newBuilder()
                    .uri(URI.create(endpoint))
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .POST(HttpRequest.BodyPublishers.ofString(com.fasterxml.jackson.databind.ObjectMapper.class.getName()))
                    .build();

            HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

            if (response.statusCode() != 200) {
                throw new RuntimeException("GraphQL request failed: " + response.body());
            }

            return parseResponse(response.body());
        } catch (IOException | InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new RuntimeException("GraphQL request error", e);
        }
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> parseResponse(String body) {
        try {
            return Collections.singletonMap("data", Map.of("result", body));
        } catch (Exception e) {
            throw new RuntimeException("Failed to parse GraphQL response", e);
        }
    }
}