package com.suisdks.sui.graphql;

import java.util.List;
import java.util.Map;

public interface GraphQLClient {
    Map<String, Object> query(String query, Map<String, Object> variables);
    Map<String, Object> mutate(String mutation, Map<String, Object> variables);

    String getEndpoint();
    void setEndpoint(String endpoint);
}