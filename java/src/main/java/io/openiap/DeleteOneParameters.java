package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class DeleteOneParameters extends Structure {
    public String collectionname;
    public String id;
    public boolean recursive;
    public int request_id;

    public DeleteOneParameters() {
        recursive = false;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "id", "recursive", "request_id"
        );
    }

    public static class Builder {
        private DeleteOneParameters instance = new DeleteOneParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder id(String id) {
            instance.id = id;
            return this;
        }

        public Builder recursive(boolean recursive) {
            instance.recursive = recursive;
            return this;
        }

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public DeleteOneParameters build() {
            instance.write();
            return instance;
        }
    }
}
