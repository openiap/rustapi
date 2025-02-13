package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class ColTimeseriesWrapper extends Structure {
    public String time_field;
    public String meta_field;
    public String granularity;

    public ColTimeseriesWrapper() {
        this.time_field = "";
        this.meta_field = "";
        this.granularity = "";
    }

    public ColTimeseriesWrapper(TimeUnit granularity, String timeField) {
        this.time_field = timeField;
        this.meta_field = null; // or an actual value if needed
        this.granularity = granularity.label;
    }
    public ColTimeseriesWrapper(TimeUnit granularity, String timeField, String metaField) {
        this.time_field = timeField;
        this.meta_field = metaField;
        this.granularity = granularity.label;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("time_field", "meta_field", "granularity");
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
