package io.openiap;

import java.util.ArrayList;
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
        public String java;
    }

    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");

        Client client = new Client(libpath);
        try {
            client.enableTracing("openiap=info", "");
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

            // CreateCollection createExpColParams = new CreateCollection.Builder("testjavaexpcollection")
            // .expire(60)                
            // .build();
            // boolean ExpColcreated = client.createCollection(createExpColParams);
            // if (ExpColcreated) {
            //     System.out.println("Collection created successfully!");
            // } else {
            //     System.err.println("Failed to create collection!");
            // }
            // // client.dropCollection("testjavaexpcollection");

            // ColTimeseriesWrapper timeseries = new ColTimeseriesWrapper(TimeUnit.MINUTES, "ts");
            // CreateCollection createTSColParams = new CreateCollection.Builder("testjavatscollection")
            //     .timeseries(timeseries)
            //     .build();
            // boolean TSColcreated = client.createCollection(createTSColParams);
            // if (TSColcreated) {
            //     System.out.println("Collection created successfully!");
            // } else {
            //     System.err.println("Failed to create collection!");
            // }
            // // client.dropCollection("testjavatscollection");
            // ColTimeseriesWrapper timeseries2 = new ColTimeseriesWrapper(TimeUnit.MINUTES, "ts", "metadata");
            // CreateCollection createTSColParams2 = new CreateCollection.Builder("testjavats2collection")
            //     .timeseries(timeseries2)
            //     .build();
            // boolean TSColcreated2 = client.createCollection(createTSColParams2);
            // if (TSColcreated2) {
            //     System.out.println("Collection created successfully!");
            // } else {
            //     System.err.println("Failed to create collection!");
            // }
            // // client.dropCollection("testjavats2collection");

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

            // InsertOneParameters insertOneParams = new InsertOneParameters.Builder()
            //     .collectionname("entities")
            //     .item("{\"_type\":\"test\", \"name\":\"test01\"}")
            //     .build();

            // String insertOneResult = client.insertOne(insertOneParams);
            // System.out.println("InsertOne Result (JSON): " + insertOneResult);

            // InsertOneParameters insertOneParams2 = new InsertOneParameters.Builder()
            //     .collectionname("entities")
            //     .item("{\"_type\":\"test\", \"name\":\"test02\"}")
            //     .build();

            // Entity insertedEntity = client.insertOne(Entity.class, insertOneParams2);
            // System.out.println("InsertOne Result (Entity): " + insertedEntity.name + " id: " + insertedEntity._id);
            
            // insertedEntity._id = null;
            // InsertOneParameters insertOneParams3 = new InsertOneParameters.Builder()
            //     .collectionname("entities")
            //     .itemFromObject(insertedEntity)
            //     .build();

            // Entity insertedEntity3 = client.insertOne(Entity.class, insertOneParams3);
            // System.out.println("InsertOne Result (Entity): " + insertedEntity3.name + " id: " + insertedEntity3._id);


            // UpdateOneParameters updateOneParams = new UpdateOneParameters.Builder()
            //     .collectionname("entities")
            //     .item("{\"_id\":\"" + insertedEntity3._id + "\", \"name\":\"test02-updated\"}")
            //     .build();

            // String updateOneResult = client.updateOne(updateOneParams);
            // System.out.println("UpdateOne Result (JSON): " + updateOneResult);

            // insertedEntity3.name = "test02-updated-again";
            // UpdateOneParameters updateOneParams2 = new UpdateOneParameters.Builder()
            //     .collectionname("entities")
            //     .itemFromObject(insertedEntity3)
            //     .build();

            // Entity updatedEntity = client.updateOne(Entity.class, updateOneParams2);
            // System.out.println("UpdateOne Result (Entity): " + updatedEntity.name + " id: " + updatedEntity._id);

            // InsertOrUpdateOneParameters insertOrUpdateOneParams = new InsertOrUpdateOneParameters.Builder()
            //     .collectionname("entities")
            //     .uniqeness("name")
            //     .item("{\"_type\":\"test\", \"name\":\"test01-uniqene\", \"now\":\"" + System.currentTimeMillis() + "\"}")
            //     .build();

            // String insertOrUpdateOneResult = client.insertOrUpdateOne(insertOrUpdateOneParams);
            // System.out.println("InsertOrUpdateOne Result (JSON): " + insertOrUpdateOneResult);

            // InsertOrUpdateOneParameters insertOrUpdateOneParams2 = new InsertOrUpdateOneParameters.Builder()
            //     .collectionname("entities")
            //     .uniqeness("name")
            //     .item("{\"_type\":\"test\", \"name\":\"test01-uniqene\", \"now\":\"" + System.currentTimeMillis() + "\"}")
            //     .build();

            // updatedEntity = client.insertOrUpdateOne(Entity.class, insertOrUpdateOneParams2);
            // System.out.println("InsertOrUpdateOne Result (Entity): " + updatedEntity.name + " id: " + updatedEntity._id);


            // List<Object> entities = new ArrayList<>();
            // entities.add(new Entity(){{name = "insertmany1"; _type = "test"; java = "many"; }});
            // entities.add(new Entity(){{name = "insertmany2"; _type = "test"; java = "many"; }});

            // InsertManyParameters insertManyParams = new InsertManyParameters.Builder()
            //     .collectionname("entities")
            //     .itemsFromObjects(entities)
            //     .build();

            // String insertManyResult = client.insertMany(insertManyParams);
            // System.out.println("InsertMany Result (JSON): " + insertManyResult);

            // String jsonItems = "[{\"_type\":\"test\", \"java\":\"many\", \"name\":\"insertmany3\"}, {\"_type\":\"test\", \"java\":\"many\", \"name\":\"insertmany4\"}]";
            // InsertManyParameters insertManyParams2 = new InsertManyParameters.Builder()
            //     .collectionname("entities")
            //     .items(jsonItems)
            //     .build();

            // List<Entity> insertedEntities = client.insertMany(new TypeReference<List<Entity>>() {}.getType(), insertManyParams2);
            // System.out.println("InsertMany Result (Entity List):");
            // for (Entity entity : insertedEntities) {
            //     System.out.println("  " + entity.name + " id: " + entity._id);
            //     client.deleteOne(
            //         new DeleteOneParameters.Builder()
            //             .collectionname("entities")
            //             .id(entity._id)
            //             .build()
            //     );
            // }

            // var deletecount = client.deleteMany(
            //     new DeleteManyParameters.Builder()
            //         .collectionname("entities")
            //         .query("{\"java\":\"many\"}")
            //         .build(),
            //     null // or an array of ids
            // );
            // if(deletecount == 0) {
            //     System.out.println("No entities deleted.");
            // } else {
            //     System.out.println("Deleted " + deletecount + " entities.");
            // }


            client.watchAsync(
                new WatchParameters.Builder()
                    .collectionname("entities")
                    .build(),
                (result) -> {
                    System.out.println("Watch result: " + result.operation + " on " + result.id + " " + result.document);
                }
            );

            InsertOneParameters insertOneParams3 = new InsertOneParameters.Builder()
                .collectionname("entities")
                .itemFromObject(new Entity(){{name = "watchtest"; _type = "test"; java = "many"; }})
                .build();

            Entity insertedEntity3 = client.insertOne(Entity.class, insertOneParams3);
            System.out.println("InsertOne Result (Entity): " + insertedEntity3.name + " id: " + insertedEntity3._id);

            
            var id = client.upload(
                new UploadParameters.Builder()
                    .filepath("testfile.csv")   
                    .filename("testfile.csv")
                    .metadata("{\"_type\":\"test\"}")
                    .collectionname("fs.files")
                    .build()
            );
            System.out.println("testfile.csv uploaded as " + id);
            var filename = client.download(
                new DownloadParameters.Builder()
                    .collectionname("fs.files")
                    .filename("train.csv")
                    .id(id)
                    .build()
            );
            System.out.println(id + " downloaded as " + filename);

            // wait for 10 seconds
            Thread.sleep(10000);
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            client.disconnect();
            System.out.println("CLI executed successfully!");
        }
    }
}
