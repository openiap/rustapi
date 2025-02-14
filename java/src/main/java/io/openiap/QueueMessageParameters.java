package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class QueueMessageParameters extends Structure {
    public String queuename;
    public String correlation_id;
    public String replyto;
    public String routingkey;
    public String exchangename;
    public String message;
    public int request_id;

    public QueueMessageParameters() {}

    public static class Builder {
        private String queuename;
        private String correlation_id;
        private String replyto;
        private String routingkey;
        private String exchangename;
        private String message;
        private int request_id;

        public Builder() {}

        public Builder queuename(String queuename) {
            this.queuename = queuename;
            return this;
        }

        public Builder correlation_id(String correlation_id) {
            this.correlation_id = correlation_id;
            return this;
        }

        public Builder replyto(String replyto) {
            this.replyto = replyto;
            return this;
        }

        public Builder routingkey(String routingkey) {
            this.routingkey = routingkey;
            return this;
        }

        public Builder exchangename(String exchangename) {
            this.exchangename = exchangename;
            return this;
        }

        public Builder message(String message) {
            this.message = message;
            return this;
        }

        public QueueMessageParameters build() {
            QueueMessageParameters params = new QueueMessageParameters();
            params.queuename = this.queuename;
            params.correlation_id = this.correlation_id;
            params.replyto = this.replyto;
            params.routingkey = this.routingkey;
            params.exchangename = this.exchangename;
            params.message = this.message;
            params.request_id = this.request_id;
            return params;
        }
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("queuename", "correlation_id", "replyto", "routingkey", "exchangename", "message", "request_id");
    }
}
