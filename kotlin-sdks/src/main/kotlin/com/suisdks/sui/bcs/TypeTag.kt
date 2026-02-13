package com.suisdks.sui.bcs

import java.io.ByteArrayOutputStream

data class TypeTag(
    val address: String = "",
    val module: String = "",
    val name: String = "",
    val typeParams: List<TypeTag> = emptyList()
) {
    fun serialize(): ByteArray {
        val output = ByteArrayOutputStream()
        val writer = BcsWriter(output)
        
        // TypeTag enum: 0 = Vector, 1 = StructTag, 2 = TypeParam
        if (typeParams.isNotEmpty()) {
            writer.writeU8(0) // Vector
            writer.writeUleb128(typeParams.size)
            typeParams.forEach { it.serialize() }
        } else if (address.isNotEmpty()) {
            writer.writeU8(1) // StructTag
            serializeStructTag(writer)
        } else {
            writer.writeU8(2) // TypeParam
        }
        
        return output.toByteArray()
    }

    private fun serializeStructTag(writer: BcsWriter) {
        // Address
        val addressBytes = hexToBytes(address.removePrefix("0x"))
        writer.writeBytes(addressBytes)
        
        // Module
        writer.writeString(module)
        
        // Name
        writer.writeString(name)
        
        // Type params
        writer.writeUleb128(typeParams.size)
        typeParams.forEach { param ->
            param.serialize()
        }
    }

    override fun toString(): String {
        val base = if (address.isNotEmpty()) {
            "$address::$module::$name"
        } else {
            name
        }
        
        return if (typeParams.isNotEmpty()) {
            val params = typeParams.joinToString(", ") { it.toString() }
            "$base<$params>"
        } else {
            base
        }
    }

    companion object {
        fun fromString(typeStr: String): TypeTag {
            val parts = typeStr.split("::")
            if (parts.size < 3) {
                return TypeTag(name = typeStr)
            }
            
            val address = parts[0]
            val module = parts[1]
            val nameAndParams = parts[2]
            
            // Handle type parameters
            val ltIndex = nameAndParams.indexOf('<')
            if (ltIndex == -1) {
                return TypeTag(address, module, nameAndParams)
            }
            
            val name = nameAndParams.substring(0, ltIndex)
            val paramsStr = nameAndParams.substring(ltIndex + 1, nameAndParams.length - 1)
            
            // Parse nested type parameters (simplified)
            val typeParams = parseTypeParams(paramsStr)
            
            return TypeTag(address, module, name, typeParams)
        }

        private fun parseTypeParams(paramsStr: String): List<TypeTag> {
            if (paramsStr.isBlank()) return emptyList()
            
            val params = mutableListOf<String>()
            var depth = 0
            var current = StringBuilder()
            
            for (char in paramsStr) {
                when (char) {
                    '<' -> {
                        depth++
                        current.append(char)
                    }
                    '>' -> {
                        depth--
                        current.append(char)
                    }
                    ',' -> {
                        if (depth == 0) {
                            params.add(current.toString().trim())
                            current = StringBuilder()
                        } else {
                            current.append(char)
                        }
                    }
                    else -> current.append(char)
                }
            }
            
            if (current.isNotEmpty()) {
                params.add(current.toString().trim())
            }
            
            return params.map { fromString(it) }
        }

        private fun hexToBytes(hex: String): ByteArray {
            val cleanHex = hex.removePrefix("0x").padStart(32, '0')
            return ByteArray(cleanHex.length / 2) {
                cleanHex.substring(it * 2, it * 2 + 2).toInt(16).toByte()
            }
        }
    }
}

fun normalizeTypeTag(tag: String): String {
    return TypeTag.fromString(tag).toString()
}

fun isValidTypeTag(tag: String): Boolean {
    return try {
        TypeTag.fromString(tag)
        true
    } catch (e: Exception) {
        false
    }
}
