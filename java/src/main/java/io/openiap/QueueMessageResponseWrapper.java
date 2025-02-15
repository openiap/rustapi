package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class QueueMessageResponseWrapper {
    public static class Response extends Structure {
        public boolean success;
        public String error;

        public Response(Pointer p) {
            super(p);
            read();
        }

        public boolean getSuccess() {
            return success;
        }

        @Override
        protected java.util.List<String> getFieldOrder() {
            return java.util.Arrays.asList("success", "error");
        }
    }
}
