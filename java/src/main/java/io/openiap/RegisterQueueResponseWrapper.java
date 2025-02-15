package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

public class RegisterQueueResponseWrapper {
    public static class Response extends Structure {
        public boolean success;
        public String queuename;
        public String error;

        public Response(Pointer p) {
            super(p);
            read();
        }

        public boolean getSuccess() {
            return success;
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "queuename", "error");
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
        void invoke(Pointer response);
    }
}
