package io.openiap;

import com.sun.jna.Callback;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class RpcResponseWrapper {
    public static class Response extends Structure {
        public boolean success;
        public String result;
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
            return java.util.Arrays.asList("success", "result", "error");
        }
        public class RpcResponse {
            public final boolean success;
            public final String result;
            public final String error;
        
            public RpcResponse(String error, boolean success, String result) {
                this.error = error;
                this.success = success;
                this.result = result;
            }
        }
    }
    public interface RpcResponseCallback extends Callback {
        void invoke(Pointer responsePtr);
    }
}
