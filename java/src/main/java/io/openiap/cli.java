package io.openiap;

public class cli {
    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");

        Client client = new Client(libpath);
        client.enableTracing("openiap=debug", "");
        client.connect("");
        client.start();
        client.hello();

        client.disableTracing();
        System.out.println("CLI executed successfully!");
    }
}
