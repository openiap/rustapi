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

    // public static class QueryRequestWrapper extends Structure {
    //     public String collectionname;
    //     public String query = "{}";
    //     public String projection = "{}";
    //     public String orderby = null;
    //     public String queryas = null;
    //     public boolean explain = false;
    //     public int skip = 0;
    //     public int top = 0;
    //     public int request_id = 0;

    //     @Override
    //     protected List<String> getFieldOrder() {
    //         return Arrays.asList("collectionname", "query", "projection", "orderby", "queryas", "explain", "skip", "top", "request_id");
    //     }
    // }

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
