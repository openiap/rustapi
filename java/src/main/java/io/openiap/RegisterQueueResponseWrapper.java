package io.openiap;

import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

public class RegisterQueueResponseWrapper {
    // Make this public so Client can access it
    public static final Map<String, QueueEventCallback> activeCallbacks = new ConcurrentHashMap<>();

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

    public static void registerCallback(String queueName, QueueEventCallback callback) {
        activeCallbacks.put(queueName, callback);
    }

    public static void unregisterCallback(String queueName) {
        activeCallbacks.remove(queueName);
    }

    public static Set<String> getActiveQueues() {
        return activeCallbacks.keySet();
    }

    // Modify QueueEventCallback to match Client's callback type
    public interface QueueEventCallback extends Callback {
        String invoke(Pointer eventPtr);
    }

    public interface ExchangeEventCallback extends Callback {
        void invoke(Pointer eventPtr);
    }
    public interface ClientEventCallback extends Callback {
        void invoke(Pointer event);
    }

    public interface RegisterQueueCallback extends Callback {
        void invoke(Pointer response);
    }
}
