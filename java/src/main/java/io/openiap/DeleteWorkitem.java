package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class DeleteWorkitem extends Structure {
    public String id;
    public int request_id;

    public DeleteWorkitem() {
        super();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("id", "request_id");
    }

    public static class Builder {
        private DeleteWorkitem instance = new DeleteWorkitem();

        public Builder(String id) {
            instance.id = id;
        }

        public DeleteWorkitem build() {
            instance.write();
            return instance;
        }
    }
}
