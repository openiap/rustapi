package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class DropIndexResponseWrapper {
    public static class Response extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public Response() {
            // Default constructor is required for JNA
        }

        public Response(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }

        public boolean getSuccess() {
            return success != 0;
        }
    }
    
}
