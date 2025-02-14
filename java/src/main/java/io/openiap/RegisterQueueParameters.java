package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class RegisterQueueParameters extends Structure {
    public String queuename;
    public int request_id;

    public RegisterQueueParameters() {
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "queuename", "request_id"
        );
    }

    public static class Builder {
        private RegisterQueueParameters instance = new RegisterQueueParameters();

        public Builder queuename(String queuename) {
            instance.queuename = queuename;
            return this;
        }

        public RegisterQueueParameters build() {
            instance.write();
            return instance;
        }
    }
}
