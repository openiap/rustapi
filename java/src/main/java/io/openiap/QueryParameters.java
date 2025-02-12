package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class QueryParameters extends Structure {
    // Fields that map to the native struct.
    public String collectionname;
    public String query;
    public String projection;
    public String orderby;
    public String queryas;
    public boolean explain;
    public int skip;
    public int top;
    public int request_id;

    // Default constructor (required by JNA)
    public QueryParameters() {
        // Optionally, initialize default values here
    }

    // Ensure the field order is exactly as expected by the native struct.
    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "query", "projection", "orderby",
            "queryas", "explain", "skip", "top", "request_id"
        );
    }

    // A builder for easier construction.
    public static class Builder {
        // Create an instance to be built.
        private QueryParameters instance = new QueryParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder query(String query) {
            instance.query = query;
            return this;
        }

        public Builder projection(String projection) {
            instance.projection = projection;
            return this;
        }

        public Builder orderby(String orderby) {
            instance.orderby = orderby;
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

        public Builder skip(int skip) {
            instance.skip = skip;
            return this;
        }

        public Builder top(int top) {
            instance.top = top;
            return this;
        }

        public Builder request_id(int request_id) {
            instance.request_id = request_id;
            return this;
        }

        public QueryParameters build() {
            // Write the field values into the native memory if needed.
            instance.write();
            return instance;
        }
    }
}
