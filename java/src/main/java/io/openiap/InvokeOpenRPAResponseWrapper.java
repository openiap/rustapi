package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class InvokeOpenRPAResponseWrapper extends Structure {
    public static class ByReference extends InvokeOpenRPAResponseWrapper implements Structure.ByReference {}

    public boolean success;
    public String result;
    public String error;
    public int request_id;

    public InvokeOpenRPAResponseWrapper() {}

    public InvokeOpenRPAResponseWrapper(Pointer p) {
        super(p);
        read();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("success", "result", "error", "request_id");
    }

    public boolean getSuccess() {
        return success;
    }

    public static class Response extends InvokeOpenRPAResponseWrapper {
        public Response(Pointer p) {
            super(p);
        }
    }
}