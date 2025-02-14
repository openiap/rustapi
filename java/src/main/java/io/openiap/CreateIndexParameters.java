package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class CreateIndexParameters extends Structure {
    public String collectionname;
    public String index;
    public String options;
    public String name;
    public int request_id;

    public CreateIndexParameters() {
        collectionname = "";
        index = "";
        options = "";
        name = "";
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "index", "options", "name", "request_id"
        );
    }

    public static class Builder {
        private CreateIndexParameters instance = new CreateIndexParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder index(String index) {
            instance.index = index;
            return this;
        }

        public Builder options(String options) {
            instance.options = options;
            return this;
        }

        public Builder name(String name) {
            instance.name = name;
            return this;
        }

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public CreateIndexParameters build() {
            instance.write();
            return instance;
        }
    }
}
