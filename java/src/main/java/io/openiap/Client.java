package io.openiap;

import com.sun.jna.NativeLibrary;
import com.sun.jna.Function;
import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class Client {
    private final Function createClientFunc;
    private final Function clientConnectFunc;
    private Pointer clientPtr;

    public Client(String fullLibPath) {
        System.out.println("GetInstance of: " + fullLibPath);
        NativeLibrary lib = NativeLibrary.getInstance(fullLibPath);
        createClientFunc = lib.getFunction("create_client");
        clientConnectFunc = lib.getFunction("client_connect");
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

    public void connect(String serverUrl) {
        if (clientPtr == null) {
            clientPtr = (Pointer) createClientFunc.invoke(Pointer.class, new Object[]{});
        }
        
        Pointer responsePtr = (Pointer) clientConnectFunc.invoke(Pointer.class, 
            new Object[]{clientPtr, serverUrl});
        
        if (responsePtr == null) {
            throw new RuntimeException("Connection attempt returned null response");
        }

        ConnectResponseWrapper response = new ConnectResponseWrapper(responsePtr);

        if (!response.success) {
            String errorMsg = response.error != null ? response.error : "Unknown error";
            throw new RuntimeException("Failed to connect to server: " + errorMsg);
        }

        System.out.println("Successfully connected to server: " + serverUrl);
    }

    public void hello() {
        System.out.println("Hello from Client!");
    }
}
