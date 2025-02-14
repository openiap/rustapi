package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Native;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.lang.reflect.Type;
import com.sun.jna.Memory;

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
}

public class Client {
    private final ObjectMapper objectMapper;
    private Pointer clientPtr;
    private CLib clibInstance;

    public Client(String fullLibPath) {
        this.objectMapper = new ObjectMapper();
        // Map<String, Object> options = new HashMap<>();
        // options.put(Library.OPTION_TYPE_MAPPER, new BooleanTypeMapper());
        // clibInstance = (CLib) Native.load(fullLibPath, CLib.class, options);
        clibInstance = (CLib) Native.load(fullLibPath, CLib.class);
    }

    public void start() {
        clientPtr = clibInstance.create_client();
        if (clientPtr != null) {
            System.out.println("Failed to create client.");
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

        Wrappers.ConnectResponseWrapper response = new Wrappers.ConnectResponseWrapper(responsePtr);
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

        Wrappers.ListCollectionsResponseWrapper response = new Wrappers.ListCollectionsResponseWrapper(responsePtr);
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
        Wrappers.QueryResponseWrapper response = new Wrappers.QueryResponseWrapper(responsePtr);
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
        Wrappers.AggregateResponseWrapper response = new Wrappers.AggregateResponseWrapper(responsePtr);
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

        Wrappers.CreateCollectionResponseWrapper response = new Wrappers.CreateCollectionResponseWrapper(responsePtr);
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

        Wrappers.DropCollectionResponseWrapper response = new Wrappers.DropCollectionResponseWrapper(responsePtr);
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
        Wrappers.QueryResponseWrapper response = new Wrappers.QueryResponseWrapper(responsePtr);
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
        Wrappers.QueryResponseWrapper response = new Wrappers.QueryResponseWrapper(responsePtr);
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
        Wrappers.QueryResponseWrapper response = new Wrappers.QueryResponseWrapper(responsePtr);
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
        Wrappers.QueryResponseWrapper response = new Wrappers.QueryResponseWrapper(responsePtr);
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
        Wrappers.DeleteOneResponseWrapper response = new Wrappers.DeleteOneResponseWrapper(responsePtr);
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
            Wrappers.DeleteManyResponseWrapper response = new Wrappers.DeleteManyResponseWrapper(responsePtr);
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
            if (idsPtr != null) {
                for (int i = 0; i < ids.length; i++) {
                    Pointer p = idsPtr.getPointer(i * Native.POINTER_SIZE);
                    // release memory for each string
                }
                // release memory for the array of pointers
            }
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
        Wrappers.DownloadResponseWrapper response = new Wrappers.DownloadResponseWrapper(responsePtr);
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
