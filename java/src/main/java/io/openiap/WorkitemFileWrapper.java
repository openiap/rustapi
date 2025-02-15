package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import java.util.Arrays;
import java.util.List;

public class WorkitemFileWrapper extends Structure {
    public String filename;
    public String id;
    public byte compressed;  // Using byte for boolean

    public WorkitemFileWrapper(Pointer p) {
        super(p);
        read();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("filename", "id", "compressed");
    }
}
