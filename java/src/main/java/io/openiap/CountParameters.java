package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class CountParameters extends Structure {
    public String collectionname;
    public String query;
    public String queryas;
    public boolean explain;
    public int request_id;

    public CountParameters() {
        collectionname = "";
        query = "";
        queryas = "";
        explain = false;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "query", "queryas", "explain", "request_id"
        );
    }

    public static class Builder {
        private CountParameters instance = new CountParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder query(String query) {
            instance.query = query;
            return this;
        }

        public Builder queryas(String queryas) {
            instance.queryas = queryas;
            return this;
        }

        public Builder explain(boolean explain) {
            instance.explain = explain;
            return this;
        }

        public CountParameters build() {
            instance.write();
            return instance;
        }
    }
}
