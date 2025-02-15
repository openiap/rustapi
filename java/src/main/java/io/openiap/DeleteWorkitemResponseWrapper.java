package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import java.util.Arrays;
import java.util.List;

public class DeleteWorkitemResponseWrapper extends Structure {
    public static class Response extends Structure {
        public boolean success;
        public String error;
        public int request_id;

        public Response(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }

        public boolean getSuccess() {
            return success;
        }
    }
}
