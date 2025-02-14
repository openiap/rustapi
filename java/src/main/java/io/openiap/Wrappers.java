package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;

public class Wrappers {

    public static class ConnectResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public ConnectResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class QueryResponseWrapper extends Structure {
        public byte success;
        public String results;
        public String error;
        public int request_id;

        public QueryResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class AggregateResponseWrapper extends Structure {
        public byte success;
        public String results;
        public String error;
        public int request_id;

        public AggregateResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }
    public static class ListCollectionsResponseWrapper extends Structure {
        public byte success;
        public String results;
        public String error;
        public int request_id;
        
        public ListCollectionsResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "results", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }
    public static class CreateCollectionResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public CreateCollectionResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class DropCollectionResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public DropCollectionResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class DeleteOneResponseWrapper extends Structure {
        public byte success;
        public int affectedrows;
        public String error;
        public int request_id;

        public DeleteOneResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "affectedrows", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class DeleteManyResponseWrapper extends Structure {
        public byte success;
        public int affectedrows;
        public String error;
        public int request_id;

        public DeleteManyResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "affectedrows", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class DownloadResponseWrapper extends Structure {
        public byte success;
        public String filename;
        public String error;
        public int request_id;

        public DownloadResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "filename", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }
}
