package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class DownloadParameters extends Structure {
    public String collectionname;
    public String id;
    public String folder;
    public String filename;
    public int request_id;

    public DownloadParameters() {
        folder = "";
        filename = "";
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "id", "folder", "filename", "request_id"
        );
    }

    public static class Builder {
        private DownloadParameters instance = new DownloadParameters();

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public Builder id(String id) {
            instance.id = id;
            return this;
        }

        public Builder folder(String folder) {
            instance.folder = folder;
            return this;
        }

        public Builder filename(String filename) {
            instance.filename = filename;
            return this;
        }

        public DownloadParameters build() {
            instance.write();
            return instance;
        }
    }
}
