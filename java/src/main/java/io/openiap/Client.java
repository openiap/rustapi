package io.openiap;

import com.sun.jna.NativeLibrary;
import com.sun.jna.Function;
import com.sun.jna.Pointer;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import java.lang.reflect.Type;

public class Client {
    private final NativeLibrary lib;
    private final ObjectMapper objectMapper;
    private final Function createClientFunc;
    private final Function clientConnectFunc;
    private final Function setAgentNameFunc;
    private final Function freeConnectResponseFunc;
    private final Function disconnectFunc;
    private final Function freeClientFunc;
    private final Function listCollectionsFunc;
    private final Function freeListCollectionsResponseFunc;
    private Pointer clientPtr;

    public Client(String fullLibPath) {
        System.out.println("GetInstance of: " + fullLibPath);
        this.lib = NativeLibrary.getInstance(fullLibPath);
        this.objectMapper = new ObjectMapper();
        createClientFunc = lib.getFunction("create_client");
        clientConnectFunc = lib.getFunction("client_connect");
        setAgentNameFunc = lib.getFunction("client_set_agent_name");
        freeConnectResponseFunc = lib.getFunction("free_connect_response");
        disconnectFunc = lib.getFunction("client_disconnect");
        freeClientFunc = lib.getFunction("free_client");
        listCollectionsFunc = lib.getFunction("list_collections");
        freeListCollectionsResponseFunc = lib.getFunction("free_list_collections_response");
    }

    public void start() {
        // Call the create_client function
        clientPtr = (Pointer) createClientFunc.invoke(Pointer.class, new Object[]{});
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
        setAgentNameFunc.invoke(void.class, new Object[]{clientPtr, agentName});
    }

    public void connect(String serverUrl) {
        setAgentName("java");
        if (clientPtr == null) {
            clientPtr = (Pointer) createClientFunc.invoke(Pointer.class, new Object[]{});
        }
        
        Pointer responsePtr = (Pointer) clientConnectFunc.invoke(Pointer.class, 
            new Object[]{clientPtr, serverUrl});
        
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
            freeConnectResponseFunc.invoke(void.class, new Object[]{responsePtr});
        }
    }

    public String listCollections(boolean includeHist) throws Exception {
        if (clientPtr == null) {
            throw new RuntimeException("Client not initialized");
        }
        
        Pointer responsePtr = (Pointer) listCollectionsFunc.invoke(Pointer.class, 
            new Object[]{clientPtr, includeHist});
        
        if (responsePtr == null) {
            throw new RuntimeException("List collections returned null response");
        }

        Wrappers.ListCollectionsResponseWrapper response = new Wrappers.ListCollectionsResponseWrapper(responsePtr);
        try {
            if (!response.success) {
                String errorMsg = response.error != null ? response.error : "Unknown error";
                throw new RuntimeException("Failed to list collections: " + errorMsg);
            }
            return response.results;
        } finally {
            freeListCollectionsResponseFunc.invoke(void.class, new Object[]{responsePtr});
        }
    }

    public <T> T listCollections(Type type, boolean includeHist) throws Exception {
        String jsonResponse = listCollections(includeHist);
        if (type instanceof Class && type == String.class) {
            return (T) jsonResponse;
        }
        return objectMapper.readValue(jsonResponse, objectMapper.constructType(type));
    }

    public void enableTracing(String rustLog, String tracing) {
        lib.getFunction("enable_tracing")
            .invoke(void.class, new Object[]{rustLog, tracing});
    }

    public void disableTracing() {
        lib.getFunction("disable_tracing")
            .invoke(void.class, new Object[]{});
    }

    public void hello() {
        System.out.println("Hello from Client!");
    }

    public void disconnect() {
        if (clientPtr != null) {
            disconnectFunc.invoke(void.class, new Object[]{clientPtr});
        }
    }

    private void freeClient() {
        if (clientPtr != null) {
            freeClientFunc.invoke(void.class, new Object[]{clientPtr});
            clientPtr = null;
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
