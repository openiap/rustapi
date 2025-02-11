package io.openiap;

import com.sun.jna.NativeLibrary;
import com.sun.jna.Function;
import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class Client {
    private final NativeLibrary lib;
    private final Function createClientFunc;
    private final Function clientConnectFunc;
    private final Function setAgentNameFunc;
    private final Function freeConnectResponseFunc;
    private final Function disconnectFunc;
    private final Function freeClientFunc;
    private Pointer clientPtr;

    public Client(String fullLibPath) {
        System.out.println("GetInstance of: " + fullLibPath);
        this.lib = NativeLibrary.getInstance(fullLibPath);
        createClientFunc = lib.getFunction("create_client");
        clientConnectFunc = lib.getFunction("client_connect");
        setAgentNameFunc = lib.getFunction("client_set_agent_name");
        freeConnectResponseFunc = lib.getFunction("free_connect_response");
        disconnectFunc = lib.getFunction("client_disconnect");
        freeClientFunc = lib.getFunction("free_client");
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

    public static class ConnectResponseWrapper extends Structure {
        public boolean success;
        public String error;
        public int request_id;
        
        public ConnectResponseWrapper(Pointer p) {
            super(p);
            read(); // Read the data from native memory
        }
        
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("success", "error", "request_id");
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

        ConnectResponseWrapper response = new ConnectResponseWrapper(responsePtr);
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
