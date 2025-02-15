package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class PopWorkitem extends Structure {
    public String wiq;
    public String wiqid;
    public int request_id;

    public PopWorkitem() {
        super();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("wiq", "wiqid", "request_id");
    }

    public static class Builder {
        private PopWorkitem instance = new PopWorkitem();

        public Builder(String wiq) {
            instance.wiq = wiq;
        }

        public Builder wiqid(String wiqid) {
            instance.wiqid = wiqid;
            return this;
        }

        public PopWorkitem build() {
            instance.write();
            return instance;
        }
    }
}
