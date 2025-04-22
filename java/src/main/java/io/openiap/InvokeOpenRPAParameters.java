package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class InvokeOpenRPAParameters extends Structure {
    public String robotid;
    public String workflowid;
    public String payload;
    public boolean rpc;
    public int request_id;

    public static class ByReference extends InvokeOpenRPAParameters implements Structure.ByReference {}

    public InvokeOpenRPAParameters() {}

    public InvokeOpenRPAParameters(String robotid, String workflowid, String payload, boolean rpc, int request_id) {
        this.robotid = robotid;
        this.workflowid = workflowid;
        this.payload = payload;
        this.rpc = rpc;
        this.request_id = request_id;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("robotid", "workflowid", "payload", "rpc", "request_id");
    }

    public static class Builder {
        private String robotid;
        private String workflowid;
        private String payload;
        private boolean rpc = false;
        private int request_id = 0;

        public Builder robotid(String robotid) { this.robotid = robotid; return this; }
        public Builder workflowid(String workflowid) { this.workflowid = workflowid; return this; }
        public Builder payload(String payload) { this.payload = payload; return this; }
        public Builder rpc(boolean rpc) { this.rpc = rpc; return this; }
        public Builder request_id(int request_id) { this.request_id = request_id; return this; }

        public InvokeOpenRPAParameters build() {
            return new InvokeOpenRPAParameters(robotid, workflowid, payload, rpc, request_id);
        }
    }
}