package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class QueryParameters extends Structure {
    public String collectionname;
    public String query;
    public String projection;
    public String orderby;
    public String queryas;
    public boolean explain;
    public int skip;
    public int top;
    public int request_id;

    public QueryParameters() {
        query = "{}";
        projection = "{}";
        orderby = "";
        queryas = "";
        explain = false;
        skip = 0;
        top = 0;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "query", "projection", "orderby",
            "queryas", "explain", "skip", "top", "request_id"
        );
    }

    public static class Builder {
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

        public QueryParameters build() {
            instance.write();
            return instance;
        }
    }
}
