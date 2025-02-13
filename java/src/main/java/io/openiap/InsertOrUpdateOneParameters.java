package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;
import com.fasterxml.jackson.databind.ObjectMapper;

public class InsertOrUpdateOneParameters extends Structure {
    public String collectionname;
    public String uniqeness;
    public String item;
    public int w;
    public boolean j;
    public int request_id;

    public InsertOrUpdateOneParameters() {
        this.collectionname = null;
        this.uniqeness = null;
        this.item = null;
        this.w = 0;
        this.j = false;
        this.request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "uniqeness", "item", "w", "j", "request_id"
        );
    }

    public static class Builder {
        private InsertOrUpdateOneParameters instance = new InsertOrUpdateOneParameters();
        private static final ObjectMapper objectMapper = new ObjectMapper();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder uniqeness(String uniqeness) {
            instance.uniqeness = uniqeness;
            return this;
        }

        public Builder item(String item) {
            instance.item = item;
            return this;
        }

        public Builder itemFromObject(Object obj) {
            try {
                instance.item = objectMapper.writeValueAsString(obj);
            } catch (Exception e) {
                throw new IllegalArgumentException("Failed to serialize object to JSON", e);
            }
            return this;
        }

        public Builder w(int w) {
            instance.w = w;
            return this;
        }

        public Builder j(boolean j) {
            instance.j = j;
            return this;
        }

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public InsertOrUpdateOneParameters build() {
            instance.write();
            return instance;
        }
    }
}
