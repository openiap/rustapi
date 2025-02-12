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
            read(); // Read the data from native memory
        }
        
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }
    }
}
