package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class DistinctParameters extends Structure {
    public String collectionname;
    public String field;
    public String query;
    public String queryas;
    public boolean explain;
    public int request_id;

    public DistinctParameters() {
        collectionname = "";
        field = "";
        query = "";
        queryas = "";
        explain = false;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "field", "query", "queryas", "explain", "request_id"
        );
    }

    public static class Builder {
        private DistinctParameters instance = new DistinctParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder field(String field) {
            instance.field = field;
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

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public DistinctParameters build() {
            instance.write();
            return instance;
        }
    }
}
