package io.openiap;

public class cli {
    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        // Ensure the native library is loaded first
        String libpath = NativeLoader.loadLibrary("openiap");

        // Now use the client
        Client client = new Client(libpath);
        client.connect("");
        client.start();
        client.hello();

        System.out.println("CLI executed successfully!");
    }
}
