package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Native;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.lang.reflect.Type;
import com.sun.jna.Memory;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.List;
import com.fasterxml.jackson.core.type.TypeReference;

import com.sun.jna.Library;

interface CLib extends Library {
    CLib INSTANCE = (CLib) Native.load("openiap", CLib.class);

    Pointer client_user(Pointer client);
    void free_user(Pointer user);
    Pointer create_client();
    Pointer client_connect(Pointer client, String serverUrl);
    void client_set_agent_name(Pointer client, String agentName);
    void free_connect_response(Pointer response);
    void client_disconnect(Pointer client);
    void free_client(Pointer client);
    Pointer list_collections(Pointer client, boolean includeHist);
    void free_list_collections_response(Pointer response);
    void enable_tracing(String rustLog, String tracing);
    void disable_tracing();
    Pointer query(Pointer client, QueryParameters options);
    void free_query_response(Pointer response);
    Pointer aggregate(Pointer client, AggregateParameters options);
    void free_aggregate_response(Pointer response);
    Pointer create_collection(Pointer client, CreateCollection options);
    void free_create_collection_response(Pointer response);
    Pointer drop_collection(Pointer client, String collectionName);
    void free_drop_collection_response(Pointer response);
    Pointer insert_one(Pointer client, InsertOneParameters options);
    void free_insert_one_response(Pointer response);
    Pointer update_one(Pointer client, UpdateOneParameters options);
    void free_update_one_response(Pointer response);
    Pointer insert_or_update_one(Pointer client, InsertOrUpdateOneParameters options);
    void free_insert_or_update_one_response(Pointer response);
    Pointer insert_many(Pointer client, InsertManyParameters options);
    void free_insert_many_response(Pointer response);
    Pointer delete_one(Pointer client, DeleteOneParameters options);
    void free_delete_one_response(Pointer response);
    Pointer delete_many(Pointer client, DeleteManyParameters options);
    void free_delete_many_response(Pointer response);
    Pointer download(Pointer client, DownloadParameters options);
    void free_download_response(Pointer response);
    Pointer upload(Pointer client, UploadParameters options);
    void free_upload_response(Pointer response);
    void watch_async_async(Pointer client, WatchParameters options, WatchResponseWrapper.WatchCallback callback, WatchResponseWrapper.WatchEventCallback event_callback);
    void free_watch_response(Pointer response);
    Pointer next_watch_event(String watchid);
    void free_watch_event(Pointer event);
    Pointer unwatch(Pointer client, String watchid);
    void free_unwatch_response(Pointer response);
    Pointer signin(Pointer client, SigninParameters options);
    void free_signin_response(Pointer response);
    Pointer get_indexes(Pointer client, String collectionName);
    void free_get_indexes_response(Pointer response);
    Pointer drop_index(Pointer client, String collectionName, String indexName);
    void free_drop_index_response(Pointer response);
    Pointer create_index(Pointer client, CreateIndexParameters options);
    void free_create_index_response(Pointer response);
    Pointer count(Pointer client, CountParameters options);
    void free_count_response(Pointer response);
    Pointer distinct(Pointer client, DistinctParameters options);
    void free_distinct_response(Pointer response);
    void register_queue_async(Pointer client, RegisterQueueParameters options, RegisterQueueResponseWrapper.QueueEventCallback event_callback);
    void register_exchange_async(Pointer client, RegisterExchangeParameters options, RegisterQueueResponseWrapper.QueueEventCallback event_callback);
}

public class Client {
    private final ObjectMapper objectMapper;
    private Pointer clientPtr;
    private CLib clibInstance;

    public Client(String fullLibPath) {
        this.objectMapper = new ObjectMapper();
        clibInstance = (CLib) Native.load(fullLibPath, CLib.class);
    }

    public void start() {
        clientPtr = clibInstance.create_client();
        if (clientPtr == null) {
            throw new RuntimeException("Failed to create client.");
        }
    }


    public void setAgentName(String agentName) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        clibInstance.client_set_agent_name(clientPtr, agentName);
    }

    public void connect(String serverUrl) {
        setAgentName("java");
        if (clientPtr == null) {
            clientPtr = clibInstance.create_client();
        }
        
        Pointer responsePtr = clibInstance.client_connect(clientPtr, serverUrl);
        
        if (responsePtr == null) {
            throw new RuntimeException("Connection attempt returned null response");
        }

        ConnectResponseWrapper.Response response = new ConnectResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
        } finally {
            clibInstance.free_connect_response(responsePtr);
        }
    }

    public String listCollections(boolean includeHist) throws Exception {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        
        Pointer responsePtr = clibInstance.list_collections(clientPtr, includeHist);
        
        if (responsePtr == null) {
            throw new RuntimeException("List collections returned null response");
        }

        ListCollectionsResponseWrapper.Response response = new ListCollectionsResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_list_collections_response(responsePtr);
        }
    }

   @SuppressWarnings("unchecked")
    public <T> T listCollections(Type type, boolean includeHist) throws Exception {
        String jsonResponse = listCollections(includeHist);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public void enableTracing(String rustLog, String tracing) {
        clibInstance.enable_tracing(rustLog, tracing);
    }

    public void disableTracing() {
        clibInstance.disable_tracing();
    }

    public void disconnect() {
        if (clientPtr != null) {
            clibInstance.client_disconnect(clientPtr);
        }
    }

    private void freeClient() {
        if (clientPtr != null) {
            clibInstance.free_client(clientPtr);
            clientPtr = null;
        }
    }

    public User getUser() {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }

        Pointer userPtr = clibInstance.client_user(clientPtr);

        if (userPtr == null) {
            return null;
        }

        User user = new User(userPtr);
        // clibInstance.free_user(userPtr);
        return user;
    }

    public String query(QueryParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.query(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("Query returned null response");
        }
        QueryResponseWrapper.Response response = new QueryResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_query_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T query(Type type, QueryParameters options) throws Exception {
        String jsonResponse = query(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public String aggregate(AggregateParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.aggregate(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("Aggregate returned null response");
        }
        AggregateResponseWrapper.Response response = new AggregateResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_aggregate_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T aggregate(Type type, AggregateParameters options) throws Exception {
        String jsonResponse = aggregate(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public boolean createCollection(CreateCollection options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.create_collection(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("createCollection returned null response");
        }

        CreateCollectionResponseWrapper.Response response = new CreateCollectionResponseWrapper.Response(responsePtr);
        try {
            if(!response.getSuccess() || response.error != null) {
                throw new RuntimeException(response.error);
            }
            return response.getSuccess();
        } finally {
            clibInstance.free_create_collection_response(responsePtr);
        }
    }

    public boolean dropCollection(String collectionName) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.drop_collection(clientPtr, collectionName);
        if (responsePtr == null) {
            throw new RuntimeException("dropCollection returned null response");
        }

        DropCollectionResponseWrapper.Response response = new DropCollectionResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.getSuccess();
        } finally {
            clibInstance.free_drop_collection_response(responsePtr);
        }
    }

    public String insertOne(InsertOneParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.insert_one(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("InsertOne returned null response");
        }
        QueryResponseWrapper.Response response = new QueryResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_query_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T insertOne(Type type, InsertOneParameters options) throws Exception {
        String jsonResponse = insertOne(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public String updateOne(UpdateOneParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.update_one(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("UpdateOne returned null response");
        }
        QueryResponseWrapper.Response response = new QueryResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_query_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T updateOne(Type type, UpdateOneParameters options) throws Exception {
        String jsonResponse = updateOne(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public String insertOrUpdateOne(InsertOrUpdateOneParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.insert_or_update_one(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("InsertOrUpdateOne returned null response");
        }
        QueryResponseWrapper.Response response = new QueryResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_query_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T insertOrUpdateOne(Type type, InsertOrUpdateOneParameters options) throws Exception {
        String jsonResponse = insertOrUpdateOne(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public String insertMany(InsertManyParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.insert_many(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("InsertMany returned null response");
        }
        QueryResponseWrapper.Response response = new QueryResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_query_response(responsePtr);
        }
    }

    @SuppressWarnings("unchecked")
    public <T> T insertMany(Type type, InsertManyParameters options) throws Exception {
        String jsonResponse = insertMany(options);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public int deleteOne(DeleteOneParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.delete_one(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("DeleteOne returned null response");
        }
        DeleteOneResponseWrapper.Response response = new DeleteOneResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.affectedrows;
        } finally {
            clibInstance.free_delete_one_response(responsePtr);
        }
    }

    public int deleteMany(DeleteManyParameters options, String[] ids) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }

        Pointer idsPtr = null;
        try {
            if (ids != null && ids.length > 0) {
                long size = Native.POINTER_SIZE * ids.length;
                idsPtr = new Memory(size);
                Pointer[] pointers = new Pointer[ids.length];
                for (int i = 0; i < ids.length; i++) {
                    Memory strBuf = new Memory(Native.toByteArray(ids[i]).length + 1);
                    strBuf.setString(0, ids[i]);
                    pointers[i] = strBuf;
                    idsPtr.setPointer(i * Native.POINTER_SIZE, strBuf);
                }
                options.ids = idsPtr;
            } else {
                options.ids = null;
            }

            Pointer responsePtr = clibInstance.delete_many(clientPtr, options);
            if (responsePtr == null) {
                throw new RuntimeException("DeleteMany returned null response");
            }
            DeleteManyResponseWrapper.Response response = new DeleteManyResponseWrapper.Response(responsePtr);
            try {
                if (!response.getSuccess() || response.error != null) {
                    String errorMsg = response.error != null ? response.error : "Unknown error";
                    throw new RuntimeException(errorMsg);
                }
                return response.affectedrows;
            } finally {
                clibInstance.free_delete_many_response(responsePtr);
            }
        } finally {
            
        }
    }

    public String download(DownloadParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.download(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("Download returned null response");
        }
        DownloadResponseWrapper.Response response = new DownloadResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.filename;
        } finally {
            clibInstance.free_download_response(responsePtr);
        }
    }

    public String upload(UploadParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.upload(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("Upload returned null response");
        }
        UploadResponseWrapper.Response response = new UploadResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.id;
        } finally {
            clibInstance.free_upload_response(responsePtr);
        }
    }

    public String watchAsync(WatchParameters options, final WatchEventCallback eventCallback) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }

        final String[] watchIdResult = new String[1];
        final CountDownLatch latch = new CountDownLatch(1);

        WatchResponseWrapper.WatchCallback watchCallback = new WatchResponseWrapper.WatchCallback() {
            @Override
            public void invoke(Pointer responsePtr) {
                WatchResponseWrapper.Response r = new WatchResponseWrapper.Response(responsePtr);
                r.read();
                if (!r.getSuccess()) {
                    System.err.println("Watch failed: " + r.error);
                    return;
                }
                watchIdResult[0] = r.watchid;
                latch.countDown();
            }
        };

        WatchResponseWrapper.WatchEventCallback nativeEventCallback = new WatchResponseWrapper.WatchEventCallback() {
            @Override
            public void invoke(Pointer eventPtr) {
                if (eventPtr == null) {
                    return;
                }
                WatchResponseWrapper.WatchEventWrapper eventWrapper = new WatchResponseWrapper.WatchEventWrapper(eventPtr);
                eventWrapper.read();
                try {
                    WatchEvent event = new WatchEvent();
                    event.id = eventWrapper.id;
                    event.operation = eventWrapper.operation;
                    event.document = eventWrapper.document;
                    eventCallback.onEvent(event);
                } finally {
                    clibInstance.free_watch_event(eventPtr);
                }
            }
        };

        clibInstance.watch_async_async(clientPtr, options, watchCallback, nativeEventCallback);
        try {
            latch.await(10, TimeUnit.SECONDS); // Wait for the watchid or timeout
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return null; // Or throw an exception
        }
        return watchIdResult[0];
    }

    public boolean unwatch(String watchid) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.unwatch(clientPtr, watchid);
         if (responsePtr == null) {
             throw new RuntimeException("Unwatch returned null response");
         }

         UnWatchResponseWrapper.Response response = new UnWatchResponseWrapper.Response(responsePtr);
        try {
            return response.getSuccess();
        } finally {
            clibInstance.free_unwatch_response(responsePtr);
        }
    }

    public String signin(SigninParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.signin(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("Signin returned null response");
        }

        SigninResponseWrapper.Response response = new SigninResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            return response.jwt;
        } finally {
            clibInstance.free_signin_response(responsePtr);
        }
    }

    public List<Index> getIndexes(String collectionName) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.get_indexes(clientPtr, collectionName);
        if (responsePtr == null) {
            throw new RuntimeException("getIndexes returned null response");
        }

        GetIndexesResponseWrapper.Response response = new GetIndexesResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            ObjectMapper mapper = new ObjectMapper();
            return mapper.readValue(response.results, new TypeReference<List<Index>>(){});
        } catch (Exception e) {
            throw new RuntimeException(e);
        } finally {
            clibInstance.free_get_indexes_response(responsePtr);
        }
    }

    public boolean dropIndex(String collectionName, String indexName) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.drop_index(clientPtr, collectionName, indexName);
        if (responsePtr == null) {
            throw new RuntimeException("dropIndex returned null response");
        }

        DropIndexResponseWrapper.Response response = new DropIndexResponseWrapper.Response(responsePtr);
        try {
            return response.getSuccess();
        } finally {
            clibInstance.free_drop_index_response(responsePtr);
        }
    }

    public boolean createIndex(CreateIndexParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.create_index(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("createIndex returned null response");
        }

        CreateIndexResponseWrapper.Response response = new CreateIndexResponseWrapper.Response(responsePtr);
        try {
            return response.getSuccess();
        } finally {
            clibInstance.free_create_index_response(responsePtr);
        }
    }

    public int count(CountParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.count(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("count returned null response");
        }

        CountResponseWrapper.Response response = new CountResponseWrapper.Response(responsePtr);
        try {
            return response.result;
        } finally {
            clibInstance.free_count_response(responsePtr);
        }
    }

    public List<String> distinct(DistinctParameters options) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        Pointer responsePtr = clibInstance.distinct(clientPtr, options);
        if (responsePtr == null) {
            throw new RuntimeException("distinct returned null response");
        }

        DistinctResponseWrapper.Response response = new DistinctResponseWrapper.Response(responsePtr);
        try {
            if (!response.getSuccess() || response.error != null) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException(errorMsg);
            }
            List<String> results = response.getResults();
            return results;
        } finally {
            clibInstance.free_distinct_response(responsePtr);
        }
    }

    public String registerQueueAsync(RegisterQueueParameters options, final QueueEventCallback eventCallback) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }

        // final String[] queueNameResult = new String[1]; // Removed: Not used
        // final CountDownLatch latch = new CountDownLatch(1); // Removed: Not used

        RegisterQueueResponseWrapper.QueueEventCallback nativeEventCallback = new RegisterQueueResponseWrapper.QueueEventCallback() {
            @Override
            public void invoke(Pointer eventPtr) {
                if (eventPtr == null) {
                    return;
                }
                RegisterQueueResponseWrapper.QueueEventWrapper eventWrapper = new RegisterQueueResponseWrapper.QueueEventWrapper(eventPtr);
                eventWrapper.read();
                try {
                    QueueEvent event = new QueueEvent();
                    event.queuename = eventWrapper.queuename;
                    event.correlation_id = eventWrapper.correlation_id;
                    event.replyto = eventWrapper.replyto;
                    event.routingkey = eventWrapper.routingkey;
                    event.exchangename = eventWrapper.exchangename;
                    event.data = eventWrapper.data;
                    eventCallback.onEvent(event);
                } finally {
                    // Assuming there's a free_queue_event function in the C library
                    // clibInstance.free_queue_event(eventPtr);
                }
            }
        };

        clibInstance.register_queue_async(clientPtr, options, nativeEventCallback);
        // try { // Removed: Not used
        //     latch.await(10, TimeUnit.SECONDS); // Wait for the queue name or timeout
        // } catch (InterruptedException e) {
        //     Thread.currentThread().interrupt();
        //     return null; // Or throw an exception
        // }
        return "OK"; //queueNameResult[0]; // Modified: Return a default value
    }

    public String registerExchangeAsync(RegisterExchangeParameters options, final QueueEventCallback eventCallback) {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }

        RegisterQueueResponseWrapper.QueueEventCallback nativeEventCallback = new RegisterQueueResponseWrapper.QueueEventCallback() {
            @Override
            public void invoke(Pointer eventPtr) {
                if (eventPtr == null) {
                    return;
                }
                RegisterQueueResponseWrapper.QueueEventWrapper eventWrapper = new RegisterQueueResponseWrapper.QueueEventWrapper(eventPtr);
                eventWrapper.read();
                try {
                    QueueEvent event = new QueueEvent();
                    event.queuename = eventWrapper.queuename;
                    event.correlation_id = eventWrapper.correlation_id;
                    event.replyto = eventWrapper.replyto;
                    event.routingkey = eventWrapper.routingkey;
                    event.exchangename = eventWrapper.exchangename;
                    event.data = eventWrapper.data;
                    eventCallback.onEvent(event);
                } finally {
                    // Assuming there's a free_queue_event function in the C library
                    // clibInstance.free_queue_event(eventPtr);
                }
            }
        };

        clibInstance.register_exchange_async(clientPtr, options, nativeEventCallback);
        return "OK";
    }

    public interface WatchEventCallback {
        void onEvent(WatchEvent event);
    }

    public interface QueueEventCallback {
        void onEvent(QueueEvent event);
    }

    @SuppressWarnings("removal")
    @Override
    protected void finalize() throws Throwable {
        try {
            disconnect();
            freeClient();
        } finally {
            super.finalize();
        }
    }
}
