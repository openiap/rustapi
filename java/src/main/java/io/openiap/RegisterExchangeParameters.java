package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class RegisterExchangeParameters extends Structure {
    public String exchangename;
    public String algorithm;
    public String routingkey;
    public boolean addqueue;
    public int request_id;

    public RegisterExchangeParameters() {
        algorithm = "";
        routingkey = "";
        addqueue = false;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "exchangename", "algorithm", "routingkey", "addqueue", "request_id"
        );
    }

    public static class Builder {
        private RegisterExchangeParameters instance = new RegisterExchangeParameters();

        public Builder exchangename(String exchangename) {
            instance.exchangename = exchangename;
            return this;
        }

        public Builder algorithm(String algorithm) {
            instance.algorithm = algorithm;
            return this;
        }

        public Builder routingkey(String routingkey) {
            instance.routingkey = routingkey;
            return this;
        }

        public Builder addqueue(boolean addqueue) {
            instance.addqueue = addqueue;
            return this;
        }

        public RegisterExchangeParameters build() {
            instance.write();
            return instance;
        }
    }
}
