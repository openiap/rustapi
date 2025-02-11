package io.openiap;

import java.util.List;
import com.fasterxml.jackson.core.type.TypeReference;

public class cli {
    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");

        Client client = new Client(libpath);
        try {
            client.enableTracing("openiap=debug", "");
            client.start();
            client.connect("");
            
            // Get raw JSON
            String collectionsJson = client.listCollectionsAsJson(false);
            System.out.println("Collections (JSON): " + collectionsJson);
            
            // Get as List of Collection objects
            List<Wrappers.Collection> collections = client.listCollections(
                new TypeReference<List<Wrappers.Collection>>(){}.getType(), 
                false
            );
            
            // Print collection details
            for (Wrappers.Collection collection : collections) {
                System.out.println("Collection name: " + collection.name);
                System.out.println("Type: " + collection.type);
                if (collection.info != null && collection.idIndex != null) {
                    System.out.println("UUID: " + collection.info.uuid + " ReadOnly: " + collection.info.readOnly + " _id index: " + collection.idIndex.name);
                } else if (collection.info != null) {
                    System.out.println("UUID: " + collection.info.uuid + " ReadOnly: " + collection.info.readOnly);
                } else if (collection.idIndex != null) {
                    System.out.println("_id index: " + collection.idIndex.name);
                }
                System.out.println("---");
            }
            
            client.hello();
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            client.disconnect();
            System.out.println("CLI executed successfully!");
        }
    }
}
