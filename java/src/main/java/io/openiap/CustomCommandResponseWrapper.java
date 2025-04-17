package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class CustomCommandResponseWrapper extends Structure {
    public boolean success;
    public String result;
    public String error;
    public int request_id;

    public CustomCommandResponseWrapper() {}

    public CustomCommandResponseWrapper(Pointer ptr) {
        super(ptr);
        read(); // Read native memory into fields
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("success", "result", "error", "request_id");
    }

    public boolean getSuccess() {
        return success;
    }
}
