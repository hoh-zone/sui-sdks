package com.suisdks.sui.bcs;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SuiBcs {
    private final BcsWriter writer;
    private final BcsReader reader;

    public SuiBcs() {
        this.writer = new BcsWriter();
        this.reader = new BcsReader(new byte[0]);
    }

    public SuiBcs(byte[] data) {
        this.writer = new BcsWriter();
        this.reader = new BcsReader(data);
    }

    public byte[] serialize(Object value) {
        if (value == null) {
            throw new IllegalArgumentException("Cannot serialize null");
        }

        if (value instanceof String) {
            writer.writeString((String) value);
        } else if (value instanceof Integer) {
            writer.writeUInt32((Integer) value);
        } else if (value instanceof Long) {
            writer.writeUInt64((Long) value);
        } else if (value instanceof Boolean) {
            writer.writeBool((Boolean) value);
        } else if (value instanceof Byte) {
            writer.writeUInt8((Byte) value);
        } else if (value instanceof byte[]) {
            writer.writeBytes((byte[]) value);
        } else if (value instanceof List) {
            serializeList((List<?>) value);
        } else if (value instanceof Map) {
            serializeMap((Map<?, ?>) value);
        } else {
            throw new IllegalArgumentException("Unsupported type: " + value.getClass());
        }

        return writer.toByteArray();
    }

    private void serializeList(List<?> list) {
        writer.writeULEB128(list.size());
        for (Object item : list) {
            serialize(item);
        }
    }

    private void serializeMap(Map<?, ?> map) {
        writer.writeULEB128(map.size());
        for (Map.Entry<?, ?> entry : map.entrySet()) {
            serialize(entry.getKey());
            serialize(entry.getValue());
        }
    }

    public <T> T deserialize(Class<T> type) {
        if (type == String.class) {
            return type.cast(reader.readString());
        } else if (type == Integer.class) {
            return type.cast(reader.readUInt32());
        } else if (type == Long.class) {
            return type.cast(reader.readUInt64());
        } else if (type == Boolean.class) {
            return type.cast(reader.readBool());
        } else if (type == Byte.class) {
            return type.cast(reader.readUInt8());
        } else if (type == List.class) {
            return type.cast(readList());
        } else if (type == Map.class) {
            return type.cast(readMap());
        } else {
            throw new IllegalArgumentException("Unsupported type: " + type);
        }
    }

    @SuppressWarnings("unchecked")
    private List<Object> readList() {
        long size = reader.readULEB128();
        List<Object> list = new ArrayList<>((int) size);
        for (long i = 0; i < size; i++) {
            list.add(deserialize(Object.class));
        }
        return list;
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> readMap() {
        long size = reader.readULEB128();
        Map<String, Object> map = new HashMap<>();
        for (long i = 0; i < size; i++) {
            String key = reader.readString();
            Object value = deserialize(Object.class);
            map.put(key, value);
        }
        return map;
    }
}