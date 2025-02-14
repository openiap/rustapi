package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class AggregateParameters extends Structure {
    public String collectionname;
    public String aggregates;
    public String queryas;
    public String hint;
    public boolean explain;
    public int request_id;

    public AggregateParameters() {
        aggregates = "[]";
        queryas = "";
        hint = "";
        explain = false;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "aggregates", "queryas", "hint",
            "explain", "request_id"
        );
    }

    public static class Builder {
        private AggregateParameters instance = new AggregateParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder aggregates(String aggregates) {
            instance.aggregates = aggregates;
            return this;
        }

        public Builder queryas(String queryas) {
            instance.queryas = queryas;
            return this;
        }

        public Builder hint(String hint) {
            instance.hint = hint;
            return this;
        }

        public Builder explain(boolean explain) {
            instance.explain = explain;
            return this;
        }

        public AggregateParameters build() {
            instance.write();
            return instance;
        }
    }
}
