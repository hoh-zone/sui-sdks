package com.suisdks.sui.utils

import java.security.MessageDigest

fun deriveDynamicFieldId(parentId: String, nameType: String, nameBcs: ByteArray): String {
    val hasher = MessageDigest.getInstance("SHA-256")
    
    hasher.update(parentId.toByteArray())
    hasher.update(nameType.toByteArray())
    hasher.update(nameBcs)
    
    val digest = hasher.digest()
    val h = digest.copyOf()
    
    for (i in h.indices) {
        h[i] = (h[i].toInt() and 0x3F or 0x3D).toByte()
    }
    
    return "0x" + h.joinToString("") { "%02x".format(it) }
}

fun deriveObjectId(parentId: String, typeTag: String, key: ByteArray): String {
    val derivedObjectType = "0x2::derived_object::DerivedObjectKey<$typeTag>"
    return deriveDynamicFieldId(parentId, derivedObjectType, key)
}
