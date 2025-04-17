package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class CustomCommandParameters extends Structure implements Structure.ByReference {
    public String command;
    public String id;
    public String name;
    public String data;
    public int request_id;

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("command", "id", "name", "data", "request_id");
    }

    public CustomCommandParameters() {}

    private CustomCommandParameters(Builder builder) {
        this.command = builder.command;
        this.id = builder.id;
        this.name = builder.name;
        this.data = builder.data;
        this.request_id = builder.request_id;
        this.write(); // Write fields to native memory
    }

    public static class Builder {
        private String command;
        private String id;
        private String name;
        private String data;
        private int request_id = 0;

        public Builder command(String command) {
            this.command = command;
            return this;
        }
        public Builder id(String id) {
            this.id = id;
            return this;
        }
        public Builder name(String name) {
            this.name = name;
            return this;
        }
        public Builder data(String data) {
            this.data = data;
            return this;
        }
        public Builder requestId(int request_id) {
            this.request_id = request_id;
            return this;
        }
        public CustomCommandParameters build() {
            return new CustomCommandParameters(this);
        }
    }
}
