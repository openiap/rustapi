package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class PushWorkitemResponseWrapper {
    public static class Response extends Structure {
        public byte success;
        public String error;
        public Pointer workitem;
        public int request_id;

        public Response(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "workitem", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }
}
