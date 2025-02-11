package io.openiap;

public class cli {
    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");

        Client client = new Client(libpath);
        try {
            client.enableTracing("openiap=debug", "");
            client.start();
            client.connect("");
            client.hello();
        } finally {
            client.disconnect();
            // client.disableTracing();
            System.out.println("CLI executed successfully!");
        }
    }
}
