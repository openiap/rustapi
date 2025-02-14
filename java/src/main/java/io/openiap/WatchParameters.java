package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class WatchParameters extends Structure {
    public String collectionname;
    public String paths;
    public int request_id;

    public WatchParameters() {
        paths = "";
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "paths", "request_id"
        );
    }

    public static class Builder {
        private WatchParameters instance = new WatchParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder paths(String paths) {
            instance.paths = paths;
            return this;
        }

        public WatchParameters build() {
            instance.write();
            return instance;
        }
    }
}
