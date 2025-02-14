package io.openiap;

import java.util.Arrays;
import java.util.List;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Callback;

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

    public static class UploadResponseWrapper extends Structure {
        public byte success;
        public String id;
        public String error;
        public int request_id;

        public UploadResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "id", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class WatchResponseWrapper extends Structure {
        public byte success;
        public String watchid;
        public String error;
        public int request_id;

        public WatchResponseWrapper() {
            // Default constructor is required for JNA
        }

        public WatchResponseWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "watchid", "error", "request_id");
        }
        public boolean getSuccess() {
            return success != 0;
        }
    }

    public interface WatchCallback extends Callback {
        void invoke(WatchResponseWrapper response);
    }

    public static class WatchEventWrapper extends Structure {
        public String id;
        public String operation;
        public String document;
        public int request_id;

        public WatchEventWrapper(Pointer p) {
            super(p);
            read();
        }
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("id", "operation", "document", "request_id");
        }
    }

    public interface WatchEventCallback extends Callback {
        void invoke(Pointer eventPtr);
    }

    public static class UnWatchResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public UnWatchResponseWrapper() {
            // Default constructor is required for JNA
        }

        public UnWatchResponseWrapper(Pointer p) {
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

    public static class SigninResponseWrapper extends Structure {
        public byte success;
        public String jwt;
        public String error;
        public int request_id;

        public SigninResponseWrapper() {
            // Default constructor is required for JNA
        }

        public SigninResponseWrapper(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "jwt", "error", "request_id");
        }

        public boolean getSuccess() {
            return success != 0;
        }
    }

    public static class GetIndexesResponseWrapper extends Structure {
        public byte success;
        public String results;
        public String error;
        public int request_id;

        public GetIndexesResponseWrapper() {
            // Default constructor is required for JNA
        }

        public GetIndexesResponseWrapper(Pointer p) {
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

    public static class DropIndexResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public DropIndexResponseWrapper() {
            // Default constructor is required for JNA
        }

        public DropIndexResponseWrapper(Pointer p) {
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

    public static class CreateIndexResponseWrapper extends Structure {
        public byte success;
        public String error;
        public int request_id;

        public CreateIndexResponseWrapper() {
            // Default constructor is required for JNA
        }

        public CreateIndexResponseWrapper(Pointer p) {
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

    public static class CountResponseWrapper extends Structure {
        public byte success;
        public int result;
        public String error;
        public int request_id;

        public CountResponseWrapper() {
            // Default constructor is required for JNA
        }

        public CountResponseWrapper(Pointer p) {
            super(p);
            read();
        }

        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "result", "error", "request_id");
        }

        public boolean getSuccess() {
            return success != 0;
        }
    }
}
