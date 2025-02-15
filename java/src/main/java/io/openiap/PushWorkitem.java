package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import java.util.Arrays;
import java.util.List;
import com.fasterxml.jackson.databind.ObjectMapper;

public class PushWorkitem extends Structure {
    public String wiq;
    public String wiqid;
    public String name;
    public String payload;
    public long nextrun;
    public String success_wiqid;
    public String failed_wiqid;
    public String success_wiq;
    public String failed_wiq;
    public int priority;
    public Pointer files;
    public int files_len;
    public int request_id;

    public PushWorkitem() {
        super();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "wiq", "wiqid", "name", "payload", "nextrun", 
            "success_wiqid", "failed_wiqid", "success_wiq", "failed_wiq",
            "priority", "files", "files_len", "request_id"
        );
    }

    public static class Builder {
        private PushWorkitem instance = new PushWorkitem();
        private static final ObjectMapper objectMapper = new ObjectMapper();

        public Builder(String wiq) {
            instance.wiq = wiq;
        }

        public Builder wiqid(String wiqid) {
            instance.wiqid = wiqid;
            return this;
        }

        public Builder name(String name) {
            instance.name = name;
            return this;
        }

        public Builder payload(String payload) {
            instance.payload = payload;
            return this;
        }

        public Builder nextrun(long nextrun) {
            instance.nextrun = nextrun;
            return this;
        }

        public Builder priority(int priority) {
            instance.priority = priority;
            return this;
        }

        public Builder successWiq(String wiq) {
            instance.success_wiq = wiq;
            return this;
        }

        public Builder failedWiq(String wiq) {
            instance.failed_wiq = wiq;
            return this;
        }

        public Builder itemFromObject(Object item) {
            try {
                String json = objectMapper.writeValueAsString(item);
                instance.payload = json;
                return this;
            } catch (Exception e) {
                throw new RuntimeException("Failed to serialize object to JSON", e);
            }
        }

        public PushWorkitem build() {
            instance.write();
            return instance;
        }
    }
}
