package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;
import com.sun.jna.Pointer;
import java.util.ArrayList;

public class DistinctResponseWrapper {
    public static class Response extends Structure {
        public byte success;
        public Pointer results;
        public String error;
        public int results_len;
        public int request_id;

        public Response() {
            // Default constructor is required for JNA
        }
        private transient List<String> resultsList;
        public List<String> getResults() {
            return resultsList;
        }

        public Response(Pointer p) {
            super(p);
            read();
            readResults();
        }

        private void readResults() {
            resultsList = new ArrayList<>();
            if (results != null) {
                for (int i = 0; i < results_len; i++) {
                    Pointer ptr = results.getPointer(i * com.sun.jna.Native.POINTER_SIZE);
                    String role = ptr.getString(0);
                    resultsList.add(role);
                }
            }
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "results_len", "request_id");
        }

        public boolean getSuccess() {
            return success != 0;
        }
    }
}