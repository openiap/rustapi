package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

public class WatchResponseWrapper {
        public static class Response extends Structure {
        public byte success;
        public String watchid;
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
            return Arrays.asList("success", "watchid", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }
    public interface WatchCallback extends Callback {
        void invoke(Pointer response);
    }

    public static class WatchEventWrapper extends Structure {
        public String id;
        public String operation;
        public String document;
        public int request_id;

        public WatchEventWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("id", "operation", "document", "request_id");
        }
    }

    public interface WatchEventCallback extends Callback {
        void invoke(Pointer eventPtr);
    }
}
