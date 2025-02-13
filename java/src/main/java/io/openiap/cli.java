package io.openiap;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.core.type.TypeReference;

import io.openiap.ColTimeseriesWrapper.TimeUnit;

public class cli {

    @JsonIgnoreProperties(ignoreUnknown = true)
    public static class Entity {
        public String _type;
        public String _id;
        public String name;
    }

    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");

        Client client = new Client(libpath);
        try {
            client.enableTracing("openiap=debug", "");
            client.start();
            client.connect("");


            // QueryParameters queryParams = new QueryParameters.Builder()
            //     .collectionname("entities")
            //     .query("{\"_type\":\"test\"}")
            //     .top(10)
            //     .request_id(123)
            //     .build();

            // List<Entity> results = client.query(new TypeReference<List<Entity>>() {}.getType(), queryParams);
            // for (Entity item : results) {
            //     System.out.println("Item: " + item._type + " " + item._id + " " + item.name);
            // }

            // // Example of querying and getting the raw JSON string
            // // queryParams.query = "{}";
            // queryParams.query = "{\"_type\":\"test\"}";
            // String jsonResult = client.query(String.class, queryParams);
            // System.out.println("Raw JSON Result: " + jsonResult);

            // AggregateParameters aggregateParams = new AggregateParameters.Builder()
            //     .collectionname("entities")
            //     .aggregates("[{\"$match\": {\"_type\": \"test\"}}, {\"$limit\": 10}]")
            //     .request_id(456)
            //     .build();

            // String aggregateJsonResult = client.aggregate(String.class, aggregateParams);
            // System.out.println("Raw JSON Aggregate Result: " + aggregateJsonResult);
            // List<Entity> aggregate = client.aggregate(new TypeReference<List<Entity>>() {}.getType(), aggregateParams);
            // for (Entity item : aggregate) {
            //     System.out.println("Item: " + item._type + " " + item._id + " " + item.name);
            // }

            // CreateCollection createColParams = new CreateCollection.Builder("testjavacollection")
            //     .build();
            // boolean Colcreated = client.createCollection(createColParams);
            // if (Colcreated) {
            //     System.out.println("Collection created successfully!");
            // } else {
            //     System.err.println("Failed to create collection!");
            // }
            // client.dropCollection("testjavacollection");

            CreateCollection createExpColParams = new CreateCollection.Builder("testjavaexpcollection")
            .expire(60)                
            .build();
            boolean ExpColcreated = client.createCollection(createExpColParams);
            if (ExpColcreated) {
                System.out.println("Collection created successfully!");
            } else {
                System.err.println("Failed to create collection!");
            }
            // client.dropCollection("testjavaexpcollection");

            ColTimeseriesWrapper timeseries = new ColTimeseriesWrapper(TimeUnit.MINUTES, "ts");
            CreateCollection createTSColParams = new CreateCollection.Builder("testjavatscollection")
                .timeseries(timeseries)
                .build();
            boolean TSColcreated = client.createCollection(createTSColParams);
            if (TSColcreated) {
                System.out.println("Collection created successfully!");
            } else {
                System.err.println("Failed to create collection!");
            }
            // client.dropCollection("testjavatscollection");
            ColTimeseriesWrapper timeseries2 = new ColTimeseriesWrapper(TimeUnit.MINUTES, "ts", "metadata");
            CreateCollection createTSColParams2 = new CreateCollection.Builder("testjavats2collection")
                .timeseries(timeseries2)
                .build();
            boolean TSColcreated2 = client.createCollection(createTSColParams2);
            if (TSColcreated2) {
                System.out.println("Collection created successfully!");
            } else {
                System.err.println("Failed to create collection!");
            }
            // client.dropCollection("testjavats2collection");

            // var str_collections = client.listCollections(false);
            // System.out.println("Collections: " + str_collections);
            // List<Collection> collections = client.listCollections(
            //     new TypeReference<List<Collection>>(){}.getType(), 
            //     false
            // );
            
            // // Print collection details
            // for (Collection collection : collections) {
            //     System.out.println("Collection name: " + collection.name);
            //     System.out.println("Type: " + collection.type);
            //     if (collection.info != null && collection.idIndex != null) {
            //         System.out.println("UUID: " + collection.info.uuid + " ReadOnly: " + collection.info.readOnly + " _id index: " + collection.idIndex.name);
            //     } else if (collection.info != null) {
            //         System.out.println("UUID: " + collection.info.uuid + " ReadOnly: " + collection.info.readOnly);
            //     } else if (collection.idIndex != null) {
            //         System.out.println("_id index: " + collection.idIndex.name);
            //     }
            //     System.out.println("---");
            // }

            // User user = client.getUser();
            // if (user != null) {
            //     System.out.println("User ID: " + user.id);
            //     System.out.println("User Name: " + user.name);
            //     System.out.println("User Username: " + user.username);
            //     System.out.println("User Email: " + user.email);
            //     System.out.println("User Roles Pointer: " + user.roles);
            //     var roles = user.getRoleList();
            //     for (int i = 0; i < roles.size(); i++) {
            //         System.out.println("Role[" + i + "]: " + roles.get(i));
            //     }
        
            // } else {
            //     System.out.println("No user found.");
            // }
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            client.disconnect();
            System.out.println("CLI executed successfully!");
        }
    }
}
