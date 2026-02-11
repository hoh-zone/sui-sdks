package com.suisdks.sui.transactions

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import org.bouncycastle.crypto.digests.Blake2bDigest
import java.util.Base64
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executor
import java.util.concurrent.ForkJoinPool

private const val SYSTEM_STATE_OBJECT_ID = "0x5"
private const val STAKE_REQUEST_TARGET = "0x3::sui_system::request_add_stake"
private const val UNSTAKE_REQUEST_TARGET = "0x3::sui_system::request_withdraw_stake"
private val TX_DIGEST_PREFIX = "TransactionData::".toByteArray(Charsets.UTF_8)
private const val BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"

class Transaction(private val client: Any? = null) {
    private val gson = Gson()
    val data = TransactionData()
    val commands: List<Map<String, Any?>> get() = data.commands
    val inputs: List<Map<String, Any?>> get() = data.inputs

    fun setSender(sender: String) {
        data.sender = sender
    }

    fun setSenderIfNotSet(sender: String) {
        if (data.sender.isBlank()) {
            setSender(sender)
        }
    }

    fun setGasBudget(budget: Long) {
        data.gasData["budget"] = budget.toString()
    }

    fun setGasBudgetIfNotSet(budget: Long) {
        if (data.gasData["budget"] == null) {
            setGasBudget(budget)
        }
    }

    fun setExpiration(expiration: Any?) {
        data.expiration = expiration
    }

    fun setGasPrice(price: Long) {
        data.gasData["price"] = price.toString()
    }

    fun setGasOwner(owner: String) {
        data.gasData["owner"] = owner
    }

    fun setGasPayment(payments: List<Map<String, Any?>>) {
        data.gasData["payment"] = payments
    }

    fun gas(): Map<String, Any?> = mapOf("\$kind" to "GasCoin", "GasCoin" to true)

    fun addInput(arg: Map<String, Any?>): Map<String, Any?> {
        data.inputs.add(arg)
        return mapOf("\$kind" to "Input", "Input" to (data.inputs.size - 1))
    }

    fun obj(value: Any): Map<String, Any?> {
        if (value is String) {
            return addInput(mapOf("\$kind" to "UnresolvedObject", "UnresolvedObject" to mapOf("objectId" to value)))
        }
        @Suppress("UNCHECKED_CAST")
        val map = value as? Map<String, Any?> ?: throw IllegalArgumentException("object input must be String or Map")
        if (map["\$kind"] == "Input") {
            return map
        }
        return addInput(map)
    }

    fun `object`(value: Any): Map<String, Any?> = obj(value)

    fun pure(value: ByteArray): Map<String, Any?> = addInput(Inputs.pure(value))

    fun addCommand(cmd: Map<String, Any?>): Map<String, Any?> {
        data.commands.add(cmd)
        return mapOf("\$kind" to "Result", "Result" to (data.commands.size - 1))
    }

    fun moveCall(target: String, args: List<Map<String, Any?>>, typeArgs: List<String> = emptyList()): Map<String, Any?> =
        addCommand(TransactionCommands.moveCall(target, args, typeArgs))

    fun transferObjects(objects: List<Map<String, Any?>>, address: Map<String, Any?>): Map<String, Any?> =
        addCommand(TransactionCommands.transferObjects(objects, address))

    fun splitCoins(coin: Map<String, Any?>, amounts: List<Map<String, Any?>>): Map<String, Any?> =
        addCommand(TransactionCommands.splitCoins(coin, amounts))

    fun mergeCoins(destination: Map<String, Any?>, sources: List<Map<String, Any?>>): Map<String, Any?> =
        addCommand(TransactionCommands.mergeCoins(destination, sources))

    fun publish(modules: List<ByteArray>, dependencies: List<String>): Map<String, Any?> =
        addCommand(TransactionCommands.publish(modules, dependencies))

    fun upgrade(
        modules: List<ByteArray>,
        dependencies: List<String>,
        packageId: String,
        ticket: Map<String, Any?>,
    ): Map<String, Any?> = addCommand(TransactionCommands.upgrade(modules, dependencies, packageId, ticket))

    fun publishUpgrade(
        modules: List<ByteArray>,
        dependencies: List<String>,
        packageId: String,
        ticket: Map<String, Any?>,
    ): Map<String, Any?> = upgrade(modules, dependencies, packageId, ticket)

    fun customUpgrade(
        modules: List<ByteArray>,
        dependencies: List<String>,
        packageId: String,
        ticket: Map<String, Any?>,
    ): Map<String, Any?> = upgrade(modules, dependencies, packageId, ticket)

    fun makeMoveVec(typeTag: String?, elements: List<Map<String, Any?>>): Map<String, Any?> =
        addCommand(TransactionCommands.makeMoveVec(typeTag, elements))

    fun makeMoveVector(typeTag: String?, elements: List<Map<String, Any?>>): Map<String, Any?> =
        makeMoveVec(typeTag, elements)

    fun transferSui(recipient: String, amount: Long): Map<String, Any?> {
        val splitResult = splitCoins(gas(), listOf(pure(u64Bytes(amount))))
        return transferObjects(listOf(splitResult), pure(recipient.toByteArray(Charsets.UTF_8)))
    }

    fun splitCoinEqual(coin: Map<String, Any?>, splitCount: Int, amountPerSplit: Long): Map<String, Any?> {
        require(splitCount > 0) { "splitCount must be positive" }
        val amounts = (0 until splitCount).map { pure(u64Bytes(amountPerSplit)) }
        return splitCoins(coin, amounts)
    }

    fun splitCoinAndReturn(coin: Map<String, Any?>, amount: Long, recipient: String): Map<String, Any?> {
        val splitResult = splitCoins(coin, listOf(pure(u64Bytes(amount))))
        transferObjects(listOf(splitResult), pure(recipient.toByteArray(Charsets.UTF_8)))
        return splitResult
    }

    fun splitCoin(coin: Map<String, Any?>, amount: Long): Map<String, Any?> =
        splitCoins(coin, listOf(pure(u64Bytes(amount))))

    fun publicTransferObject(objectToSend: Any, recipient: String): Map<String, Any?> {
        val objArg = obj(objectToSend)
        return transferObjects(listOf(objArg), pure(recipient.toByteArray(Charsets.UTF_8)))
    }

    fun stakeCoin(
        coins: List<Any>,
        validatorAddress: String,
        amount: Long? = null,
        systemStateObjectId: String = SYSTEM_STATE_OBJECT_ID,
    ): Map<String, Any?> {
        val coinArgs = coins.map { obj(it) }
        val coinsVec = makeMoveVec(null, coinArgs)
        val amountArg = pure(optionU64Bytes(amount))
        return moveCall(
            STAKE_REQUEST_TARGET,
            listOf(
                obj(systemStateObjectId),
                coinsVec,
                amountArg,
                pure(validatorAddress.toByteArray(Charsets.UTF_8)),
            ),
        )
    }

    fun unstakeCoin(stakedCoin: Any, systemStateObjectId: String = SYSTEM_STATE_OBJECT_ID): Map<String, Any?> =
        moveCall(UNSTAKE_REQUEST_TARGET, listOf(obj(systemStateObjectId), obj(stakedCoin)))

    fun build(): ByteArray = gson.toJson(transactionPayload()).toByteArray(Charsets.UTF_8)

    fun buildBase64(): String = Base64.getEncoder().encodeToString(build())

    fun serialize(): String = build().toString(Charsets.UTF_8)

    fun getTransactionData(): Map<String, Any?> = transactionPayload()

    fun deferredExecution(): Map<String, Any?> = mapOf("sender" to data.sender, "tx_bytes" to buildBase64())

    fun execute(
        client: Any? = null,
        signatures: List<String>? = null,
        options: Map<String, Any?>? = null,
    ): Map<String, Any?> {
        val activeClient = resolveClient(client)
        val txBytes = buildBase64()
        val params = if (signatures == null && options == null) {
            listOf(txBytes)
        } else {
            listOf(txBytes, signatures ?: emptyList<String>(), options ?: emptyMap())
        }
        val result = activeClient.call("sui_executeTransactionBlock", params)
        @Suppress("UNCHECKED_CAST")
        return result as? Map<String, Any?> ?: mapOf("result" to result)
    }

    fun inspectAll(client: Any? = null, sender: String? = null): Map<String, Any?> {
        val activeClient = resolveClient(client)
        val activeSender = sender ?: data.sender
        val result = activeClient.call("sui_devInspectTransactionBlock", listOf(activeSender, buildBase64()))
        @Suppress("UNCHECKED_CAST")
        return result as? Map<String, Any?> ?: mapOf("result" to result)
    }

    fun inspectForCost(client: Any? = null, sender: String? = null): Map<String, Long> {
        val result = inspectAll(client = client, sender = sender)

        @Suppress("UNCHECKED_CAST")
        val resultMap = result["result"] as? Map<String, Any?> ?: emptyMap()
        @Suppress("UNCHECKED_CAST")
        val effects = resultMap["effects"] as? Map<String, Any?> ?: emptyMap()
        @Suppress("UNCHECKED_CAST")
        val gasUsed = effects["gasUsed"] as? Map<String, Any?> ?: emptyMap()

        val computation = gasUsed["computationCost"].toLongOrZero()
        val storage = gasUsed["storageCost"].toLongOrZero()
        val rebate = gasUsed["storageRebate"].toLongOrZero()

        return mapOf(
            "computation_cost" to computation,
            "storage_cost" to storage,
            "storage_rebate" to rebate,
            "total_cost" to (computation + storage - rebate),
        )
    }

    fun executeAsync(
        client: Any? = null,
        signatures: List<String>? = null,
        options: Map<String, Any?>? = null,
        executor: Executor = ForkJoinPool.commonPool(),
    ): CompletableFuture<Map<String, Any?>> = CompletableFuture.supplyAsync(
        { execute(client = client, signatures = signatures, options = options) },
        executor,
    )

    fun inspectAllAsync(
        client: Any? = null,
        sender: String? = null,
        executor: Executor = ForkJoinPool.commonPool(),
    ): CompletableFuture<Map<String, Any?>> = CompletableFuture.supplyAsync(
        { inspectAll(client = client, sender = sender) },
        executor,
    )

    fun inspectForCostAsync(
        client: Any? = null,
        sender: String? = null,
        executor: Executor = ForkJoinPool.commonPool(),
    ): CompletableFuture<Map<String, Long>> = CompletableFuture.supplyAsync(
        { inspectForCost(client = client, sender = sender) },
        executor,
    )

    companion object {
        fun digestFromBytes(transactionDataBytes: ByteArray): String {
            val digest = Blake2bDigest(256)
            val input = TX_DIGEST_PREFIX + transactionDataBytes
            digest.update(input, 0, input.size)
            val out = ByteArray(32)
            digest.doFinal(out, 0)
            return base58Encode(out)
        }

        fun digestFromB64Str(transactionDataBytesStr: String): String =
            digestFromBytes(Base64.getDecoder().decode(transactionDataBytesStr))

        fun fromSerialized(serialized: String): Transaction {
            val raw = if (serialized.trimStart().startsWith("{")) {
                serialized.toByteArray(Charsets.UTF_8)
            } else {
                Base64.getDecoder().decode(serialized)
            }

            val gson = Gson()
            val type = object : TypeToken<Map<String, Any?>>() {}.type
            val payload = gson.fromJson<Map<String, Any?>>(raw.toString(Charsets.UTF_8), type)

            val tx = Transaction()
            tx.data.sender = payload["Sender"]?.toString() ?: ""
            tx.data.expiration = payload["Expiration"]

            @Suppress("UNCHECKED_CAST")
            tx.data.gasData.putAll(payload["GasData"] as? Map<String, Any?> ?: emptyMap())
            @Suppress("UNCHECKED_CAST")
            tx.data.inputs.addAll(payload["Inputs"] as? List<Map<String, Any?>> ?: emptyList())
            @Suppress("UNCHECKED_CAST")
            tx.data.commands.addAll(payload["Commands"] as? List<Map<String, Any?>> ?: emptyList())
            return tx
        }

        private fun base58Encode(raw: ByteArray): String {
            if (raw.isEmpty()) {
                return ""
            }
            var number = java.math.BigInteger(1, raw)
            val base = java.math.BigInteger.valueOf(58)
            val encoded = StringBuilder()

            while (number > java.math.BigInteger.ZERO) {
                val divRem = number.divideAndRemainder(base)
                number = divRem[0]
                encoded.append(BASE58_ALPHABET[divRem[1].toInt()])
            }

            var leadingZeros = 0
            for (b in raw) {
                if (b.toInt() == 0) {
                    leadingZeros++
                } else {
                    break
                }
            }
            repeat(leadingZeros) { encoded.append('1') }
            return encoded.reverse().toString()
        }
    }

    private fun transactionPayload(): Map<String, Any?> = mapOf(
        "Sender" to data.sender,
        "Expiration" to data.expiration,
        "GasData" to data.gasData,
        "Inputs" to data.inputs,
        "Commands" to data.commands,
    )

    private fun resolveClient(clientOverride: Any? = null): DynamicClient {
        val active = clientOverride ?: client ?: throw IllegalArgumentException("client is required for execution and inspection")
        return when (active) {
            is DynamicClient -> active
            else -> ReflectionClient(active)
        }
    }

    private fun u64Bytes(value: Long): ByteArray {
        require(value >= 0) { "u64 value must be non-negative" }
        val out = ByteArray(8)
        var v = value
        repeat(8) { i ->
            out[i] = (v and 0xff).toByte()
            v = v ushr 8
        }
        return out
    }

    private fun optionU64Bytes(value: Long?): ByteArray =
        if (value == null) {
            byteArrayOf(0)
        } else {
            byteArrayOf(1) + u64Bytes(value)
        }
}

private fun Any?.toLongOrZero(): Long = when (this) {
    is Number -> this.toLong()
    is String -> this.toLongOrNull() ?: 0L
    else -> 0L
}

interface DynamicClient {
    fun call(method: String, params: List<Any?> = emptyList()): Any?
}

private class ReflectionClient(private val delegate: Any) : DynamicClient {
    override fun call(method: String, params: List<Any?>): Any? {
        val fn = delegate::class.java.methods.firstOrNull {
            it.name == "call" && it.parameterTypes.size == 2
        } ?: throw IllegalArgumentException("client must expose call(method, params)")
        return fn.invoke(delegate, method, params)
    }
}
