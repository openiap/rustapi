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

    public static class ListCollectionsResponseWrapper extends Structure {
        public boolean success;
        public String results;
        public String error;
        public int request_id;
        
        public ListCollectionsResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
    }

    public static class Collection {
        public String name;
        public String type;
        public Object options;
        public CollectionInfo info;
        public CollectionIndex idIndex;

        public static class CollectionInfo {
            public boolean readOnly;
            public String uuid;
        }

        public static class CollectionIndex {
            public int v;
            public IndexKey key;
            public String name;
        }

        public static class IndexKey {
            public int _id;
        }
    }

    public static class User {
        public String id;
        public String name;
        public String username;
        public String email;
        public String[] roles;
    }

    public static class UserStructure extends Structure {
        public String id;
        public String name;
        public String username;
        public String email;
        public Pointer roles;
        
        public UserStructure(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("id", "name", "username", "email", "roles");
        }
    }
}
