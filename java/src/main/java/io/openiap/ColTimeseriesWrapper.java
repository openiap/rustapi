package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class ColTimeseriesWrapper extends Structure {
    public String time_field;
    public String meta_field;
    public String granularity;

    public ColTimeseriesWrapper() {
        time_field = "";
        meta_field = "";
        granularity = "";
    }
    public ColTimeseriesWrapper(TimeUnit granularity) {
        this.granularity = granularity.label;
        time_field = "";
        meta_field = "";
    }
    public ColTimeseriesWrapper(TimeUnit granularity, String time_field) {
        this.granularity = granularity.label;
        this.time_field = time_field;
        meta_field = "";
    }
    public ColTimeseriesWrapper(TimeUnit granularity, String time_field, String meta_field) {
        this.granularity = granularity.label;
        this.time_field = time_field;
        this.meta_field = meta_field;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "time_field", "meta_field", "granularity"
        );
    }


    public enum TimeUnit {
        SECONDS("seconds"),
        MINUTES("minutes"),
        HOURS("hours");
    
        private final String label;
    
        TimeUnit(String label) {
            this.label = label;
        }
    
        public String getLabel() {
            return label;
        }
    
        @Override
        public String toString() {
            return label;
        }
    }
}
