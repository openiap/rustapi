package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

public class RegisterQueueResponseWrapper {
    public static class Response extends Structure {
        public byte success;
        public String queuename;
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
            return Arrays.asList("success", "queuename", "error", "request_id");
        }

        public boolean getSuccess() {
            return success != 0;
        }
    }
    public static class QueueEventWrapper extends Structure {
        public String queuename;
        public String correlation_id;
        public String replyto;
        public String routingkey;
        public String exchangename;
        public String data;
        public int request_id;

        public QueueEventWrapper(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("queuename", "correlation_id", "replyto", "routingkey", "exchangename", "data", "request_id");
        }
    }

    public interface QueueEventCallback extends Callback {
        void invoke(Pointer eventPtr);
    }

    public interface RegisterQueueCallback extends Callback {
        void invoke(Pointer responsePtr);
    }
}
