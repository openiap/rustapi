package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class CreateCollection extends Structure {
    public String collectionname;
    public Pointer collation;
    public Pointer timeseries;
    public int expire_after_seconds;
    public boolean change_stream_pre_and_post_images;
    public boolean capped;
    public int max;
    public int size;
    public int request_id;

    public CreateCollection(String collectionname) {
        super();
        this.collectionname = collectionname;
        this.collation = null;
        this.timeseries = null;
        this.expire_after_seconds = 0;
        this.change_stream_pre_and_post_images = false;
        this.capped = false;
        this.max = 0;
        this.size = 0;
        this.request_id = 0;
    }

    public CreateCollection() {
        super();
        this.collectionname = null;
        this.collation = null;
        this.timeseries = null;
        this.expire_after_seconds = 0;
        this.change_stream_pre_and_post_images = false;
        this.capped = false;
        this.max = 0;
        this.size = 0;
        this.request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "collectionname", "collation", "timeseries", "expire_after_seconds",
            "change_stream_pre_and_post_images", "capped", "max", "size", "request_id"
        );
    }
    
    public static class Builder {
        private CreateCollection instance = new CreateCollection();

        public Builder(String collectionname) {
            instance.collectionname = collectionname;
        }

        public Builder collectionname(String collectionname) {
            instance.collectionname = collectionname;
            return this;
        }
        public Builder expire(int expire_after_seconds) {
            instance.expire_after_seconds = expire_after_seconds;
            return this;
        }

        public Builder collation(ColCollationWrapper collation) {
            instance.collation = collation != null ? collation.getPointer() : null;
            return this;
        }

        public Builder timeseries(ColTimeseriesWrapper timeseries) {
            if(timeseries != null) {
                timeseries.write();
                instance.timeseries = timeseries.getPointer();
            } else {
                instance.timeseries = null;
            }
            // instance.timeseries = timeseries != null ? timeseries.getPointer() : null;
            return this;
        }

        public CreateCollection build() {
            instance.write();
            return instance;
        }
    }
}
