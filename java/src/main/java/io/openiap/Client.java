package io.openiap;


import com.sun.jna.Pointer;
import com.sun.jna.Native;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.lang.reflect.Type;
import java.util.HashMap;
import java.util.Map;

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

        Collection.ListCollectionsResponseWrapper response = new Collection.ListCollectionsResponseWrapper(responsePtr);
        try {
            if (!response.success || response.error != null) {
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
