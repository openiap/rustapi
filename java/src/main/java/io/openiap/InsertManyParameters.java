package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.ArrayList;

public class InsertManyParameters extends Structure {
    public String collectionname;
    public String items;
    public int w;
    public boolean j;
    public boolean skipresults;
    public int request_id;

    public InsertManyParameters() {
        this.collectionname = null;
        this.items = null;
        this.w = 0;
        this.j = false;
        this.skipresults = false;
        this.request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "items", "w", "j", "skipresults", "request_id"
        );
    }

    public static class Builder {
        private InsertManyParameters instance = new InsertManyParameters();
        private static final ObjectMapper objectMapper = new ObjectMapper();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder items(String items) {
            instance.items = items;
            return this;
        }

        public Builder itemsFromObjects(List<Object> objects) {
            try {
                instance.items = objectMapper.writeValueAsString(objects);
            } catch (Exception e) {
                throw new IllegalArgumentException("Failed to serialize objects to JSON", e);
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

        public Builder skipresults(boolean skipresults) {
            instance.skipresults = skipresults;
            return this;
        }

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public InsertManyParameters build() {
            instance.write();
            return instance;
        }
    }
}
