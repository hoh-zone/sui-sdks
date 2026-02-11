package com.suisdks.sui.jsonrpc;

import java.util.Map;

public interface SuiRpcMethods {
    String HTTP_VERSION = "2.3.1";

    // Get object by ID
    Map<String, Object> getObject(String objectId, Map<String, Object> options);

    // Get multiple objects
    Map<String, Object> getObjects(String[] objectIds, Map<String, Object> options);

    // Transaction execution
    Map<String, Object> executeTransactionBlock(String txBytes, String[] signatures, Map<String, Object> options);

    // Transaction simulation
    Map<String, Object> devInspectTransactionBlock(String senderAddress, String txBytes, Map<String, Object> options);

    // Dry run transaction
    Map<String, Object> dryRunTransactionBlock(String txBytes, Map<String, Object> options);

    // Transaction queries
    Map<String, Object> getTransaction(String digest, Map<String, Object> options);
    Map<String, Object> getTotalTransactionBlocks(String query, Map<String, Object> cursor, long limit);

    // Coin operations
    Map<String, Object> getBalance(String owner, String coinType);
    Map<String, Object> getAllBalances(String owner);
    Map<String, Object> getAllCoins(String owner, String cursor, long limit);
    Map<String, Object> getCoinMetadata(String coinType);

    // System state
    Map<String, Object> getCurrentSystemState();
    Map<String, Object> getChainIdentifier();
    Map<String, Object> getReferenceGasPrice();

    // Staking
    Map<String, Object> getStakes(String owner);
    Map<String, Object> getStakesByIds(String[] stakeIds);
    Map<String, Object> getCurrentEpoch();
    Map<String, Object> getValidatorsApy();
    Map<String, Object> getLatestSystemState();
    Map<String, Object> getCommitteeInfo(long epoch);

    // Events
    Map<String, Object> getEvents(String query, Map<String, Object> cursor, long limit);
    Map<String, Object> queryEvents(String query, Map<String, Object> cursor, long limit);

    // Dynamic fields
    Map<String, Object> getDynamicFields(String parentId, Map<String, Object> cursor, long limit);

    // Objects and owner
    Map<String, Object> getOwnedObjects(String owner, Map<String, Object> options, Map<String, Object> cursor, long limit);
    Map<String, Object> listOwnedObjects(String owner, Map<String, Object> cursor, long limit);

    // Move module/functions
    Map<String, Object> getNormalizedMoveModules(String packageId);
    Map<String, Object> getMoveFunction(String packageId, String moduleName, String functionName);
    Map<String, Object> getRawObject(String objectId);
    Map<String, Object> tryGetPastObject(String objectId, long version);

    // Protocol
    Map<String, Object> getProtocolConfig();

    // Name service
    Map<String, Object> resolveNameServiceAddress(String name);
    Map<String, Object> resolveNameServiceNames(String address);

    // Checkpoints
    Map<String, Object> getCheckpoint(long digestID);
    Map<String, Object> getCheckpoints(Map<String, Object> cursor, long limit, boolean descending);
    Map<String, Object> getLatestCheckpointSequenceNumber();

    // Small packages
    Map<String, Object> getPackage(String packageId);
}