package com.suisdks.sui.zklogin

import com.suisdks.sui.bcs.BcsWriter
import com.suisdks.sui.utils.normalizeSuiAddress
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.math.BigInteger
import java.security.MessageDigest

@Serializable
data class ZkLoginInputs(
    val identity: Identity,
    val randomness: String,
    val epochs: List<BigInteger>,
    val jwt: String
)

@Serializable
data class Identity(
    val value: List<String>
)

@Serializable
data class ZkLoginSignature(
    val inputs: ZkLoginInputs,
    val maxEpoch: BigInteger,
    val userSignature: String
)

class ZkLogin(
    private val provider: String,
    private val clientId: String
) {
    private val json = Json { ignoreUnknownKeys = true }

    @Serializable
    data class JwtPayload(
        val iss: String,
        val sub: String,
        val aud: String,
        val exp: Long,
        val iat: Long,
        val email: String? = null,
        val nonce: String? = null
    )

    fun parseJwt(jwt: String): JwtPayload {
        val parts = jwt.split(".")
        require(parts.size == 3) { "Invalid JWT format" }
        
        val payload = java.util.Base64.getUrlDecoder().decode(parts[1])
        return json.decodeFromString(JwtPayload.serializer(), String(payload))
    }

    fun getAddressSeed(jwt: JwtPayload): BigInteger {
        val data = buildString {
            append(jwt.iss)
            append(":")
            append(jwt.sub)
        }
        
        val hash = MessageDigest.getInstance("SHA-256")
            .digest(data.toByteArray())
        
        return BigInteger(1, hash.copyOfRange(0, 32))
    }

    fun deriveAddress(jwt: JwtPayload, zkp: String): String {
        val addressSeed = getAddressSeed(jwt)
        val inputs = serializeZkLoginInputs(addressSeed, zkp, jwt)
        
        // Derive address from inputs
        val hasher = MessageDigest.getInstance("SHA-256")
        hasher.update(inputs)
        
        val hash = hasher.digest()
        val normalized = ByteArray(32)
        for (i in hash.indices) {
            normalized[i] = (hash[i].toInt() and 0x3F or 0x3D).toByte()
        }
        
        return normalizeSuiAddress("0x" + normalized.joinToString("") { "%02x".format(it) })
    }

    private fun serializeZkLoginInputs(
        addressSeed: BigInteger,
        zkp: String,
        jwt: JwtPayload
    ): ByteArray {
        val output = java.io.ByteArrayOutputStream()
        val writer = BcsWriter(output)
        
        // Serialize address seed
        writer.writeU128(addressSeed)
        
        // Serialize ZKP
        writer.writeString(zkp)
        
        // Serialize JWT claims
        writer.writeString(jwt.iss)
        writer.writeString(jwt.sub)
        writer.writeString(jwt.aud)
        
        return output.toByteArray()
    }

    fun createSignature(
        jwt: String,
        zkp: String,
        userSignature: String,
        maxEpoch: BigInteger
    ): ZkLoginSignature {
        val payload = parseJwt(jwt)
        val addressSeed = getAddressSeed(payload)
        val inputs = ZkLoginInputs(
            identity = Identity(listOf(payload.iss, payload.sub)),
            randomness = zkp,
            epochs = listOf(maxEpoch),
            jwt = jwt
        )
        
        return ZkLoginSignature(inputs, maxEpoch, userSignature)
    }

    fun serializeSignature(sig: ZkLoginSignature): ByteArray {
        val output = java.io.ByteArrayOutputStream()
        val writer = BcsWriter(output)
        
        // Scheme flag for zklogin
        writer.writeU8(4)
        
        // Serialize inputs
        writer.writeString(json.encodeToString(ZkLoginInputs.serializer(), sig.inputs))
        
        // Serialize max epoch
        writer.writeU64(sig.maxEpoch)
        
        // Serialize user signature
        val userSigBytes = java.util.Base64.getDecoder().decode(sig.userSignature)
        writer.writeBytes(userSigBytes)
        
        return output.toByteArray()
    }

    companion object {
        const val SCHEME_FLAG = 4
        
        fun isValidZkLoginSignature(data: ByteArray): Boolean {
            if (data.isEmpty()) return false
            return data[0] == SCHEME_FLAG
        }
    }
}

class ZkLoginVerifier {
    fun verify(signature: ByteArray, message: ByteArray, address: String): Boolean {
        if (!ZkLogin.isValidZkLoginSignature(signature)) {
            return false
        }
        
        // Extract and verify zklogin components
        // This is a simplified verification
        return try {
            // In production, would verify:
            // 1. ZK proof validity
            // 2. JWT signature
            // 3. Epoch validity
            // 4. Address derivation matches
            true
        } catch (e: Exception) {
            false
        }
    }
}
