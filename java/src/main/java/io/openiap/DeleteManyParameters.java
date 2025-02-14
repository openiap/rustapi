package io.openiap;

import com.sun.jna.Memory;
import com.sun.jna.Pointer;
import com.sun.jna.Structure;

import java.util.Arrays;
import java.util.List;

public class DeleteManyParameters extends Structure {
    public String collectionname;
    public String query;
    public boolean recursive;
    public Pointer ids;
    public int request_id;

    public DeleteManyParameters() {
        recursive = false;
        request_id = 0;
        ids = null;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("collectionname", "query", "recursive", "ids", "request_id");
    }

    public static class Builder {
        private DeleteManyParameters instance = new DeleteManyParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder query(String query) {
            instance.query = query;
            return this;
        }

        public Builder recursive(boolean recursive) {
            instance.recursive = recursive;
            return this;
        }

        public DeleteManyParameters build() {
            instance.write();
            return instance;
        }

    }
}
