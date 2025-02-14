package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class UploadParameters extends Structure {
    public String filepath;
    public String filename;
    public String mimetype;
    public String metadata;
    public String collectionname;
    public int request_id;

    public UploadParameters() {
        filename = "";
        mimetype = "";
        metadata = "";
        collectionname = "";
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "filepath", "filename", "mimetype", "metadata", "collectionname", "request_id"
        );
    }

    public static class Builder {
        private UploadParameters instance = new UploadParameters();

        public Builder filepath(String filepath) {
            instance.filepath = filepath;
            return this;
        }

        public Builder filename(String filename) {
            instance.filename = filename;
            return this;
        }

        public Builder mimetype(String mimetype) {
            instance.mimetype = mimetype;
            return this;
        }

        public Builder metadata(String metadata) {
            instance.metadata = metadata;
            return this;
        }

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }

        public UploadParameters build() {
            instance.write();
            return instance;
        }
    }
}
