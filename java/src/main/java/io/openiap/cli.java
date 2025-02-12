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
            
            // Get current user info
            Wrappers.User currentUser = client.getUser();
            if (currentUser != null) {
                System.out.println("\nCurrent user:");
                System.out.println("ID: " + currentUser.id);
                System.out.println("Name: " + currentUser.name);
                System.out.println("Username: " + currentUser.username);
                System.out.println("Email: " + currentUser.email);
                if (currentUser.roles != null) {
                    System.out.println("Roles: " + String.join(", ", currentUser.roles));
                }
                System.out.println("---");
            } else {
                System.out.println("No user information available");
            }

            // Get raw JSON
            // String collectionsJson = client.listCollectionsAsJson(false);

            // Get as JSON string
            String collectionsJson = client.listCollections(false);
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
