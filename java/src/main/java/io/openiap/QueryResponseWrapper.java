package io.openiap;

import java.lang.annotation.Native;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

public class QueryResponseWrapper {
    public static class Response extends Structure {
        public byte success;
        public String results;
        public String error;
        public int request_id;

        public Response(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    
}
