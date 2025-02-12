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

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "time_field", "meta_field", "granularity"
        );
    }
}
