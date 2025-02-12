package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class ColCollationWrapper extends Structure {
    public String locale;
    public boolean case_level;
    public String case_first;
    public int strength;
    public boolean numeric_ordering;
    public String alternate;
    public String max_variable;
    public boolean backwards;

    public ColCollationWrapper() {
        locale = "";
        case_level = false;
        case_first = "";
        strength = 0;
        numeric_ordering = false;
        alternate = "";
        max_variable = "";
        backwards = false;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "locale", "case_level", "case_first", "strength",
            "numeric_ordering", "alternate", "max_variable", "backwards"
        );
    }
}
