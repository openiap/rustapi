package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class InsertOneParameters extends Structure {
    public String collectionname;
    public String item;
    public int w;
    public boolean j;
    public int request_id;

    public InsertOneParameters() {
        this.collectionname = null;
        this.item = null;
        this.w = 0;
        this.j = false;
        this.request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "item", "w", "j", "request_id"
        );
    }

    public static class Builder {
        private InsertOneParameters instance = new InsertOneParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder item(String item) {
            instance.item = item;
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

        public InsertOneParameters build() {
            instance.write();
            return instance;
        }
    }
}
