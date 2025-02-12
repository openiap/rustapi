package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Native;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.lang.reflect.Type;
import com.sun.jna.Library;

interface CLib extends Library {
    // CLib INSTANCE = (CLib) Native.load("openiap", CLib.class);
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
    // void free_collections_result(Pointer response);
}

public class Client {
    private final ObjectMapper objectMapper;
    private Pointer clientPtr;
    private CLib clibInstance;

    public Client(String fullLibPath) {
        System.out.println("GetInstance of: " + fullLibPath);
        // this.lib = NativeLibrary.getInstance(fullLibPath);
        this.objectMapper = new ObjectMapper();
        clibInstance = (CLib) Native.load(fullLibPath, CLib.class);
    }

    public void start() {
        // Call the create_client function
        clientPtr = clibInstance.create_client();
        if (clientPtr != null) {
            System.out.println("Client created successfully: " + clientPtr);
        } else {
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
            if (!response.success) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException("Failed to connect to server: " + errorMsg);
            }
            System.out.println("Successfully connected to server: " + serverUrl);
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
            if (!response.success) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException("Failed to list collections: " + errorMsg);
            }
            return response.results;
        } finally {
            clibInstance.free_list_collections_response(responsePtr);
        }
    }

    // public void freeCollectionsResult(Pointer response) {
    //     clibInstance.free_collections_result(response);
    // }

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

    public void hello() {
        System.out.println("Hello from Client!");
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
