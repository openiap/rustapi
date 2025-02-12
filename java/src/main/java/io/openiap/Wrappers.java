package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class Wrappers {
    
    public static class ConnectResponseWrapper extends Structure {
        public boolean success;
        public String error;
        public int request_id;
        
        public ConnectResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }
    }
    public static class QueryResponseWrapper extends Structure {
        public boolean success;
        public String results;
        public String error;
        public int request_id;

        public QueryResponseWrapper(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
    }
}
