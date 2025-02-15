package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class ClientEventWrapper {
    public static class ClientEventStruct extends Structure {
        public String event;
        public String reason;

        public ClientEventStruct(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected java.util.List<String> getFieldOrder() {
            return java.util.Arrays.asList("event", "reason");
        }
    }
}
