package io.openiap;

import java.util.Scanner;
import com.fasterxml.jackson.core.type.TypeReference;

public class cli {
    private static Client client;
    private static volatile boolean running = true;
    private static Scanner scanner;

    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");
        client = new Client(libpath);
        scanner = new Scanner(System.in);

        try {
            client.enableTracing("openiap=info", "");
            client.start();
            client.connect("");

            System.out.println("? for help");
            while (running) {
                System.out.print("> ");
                String command = scanner.nextLine().trim().toLowerCase();
                handleCommand(command);
            }

        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            if (client != null) {
                client.disconnect();
            }
            if (scanner != null) {
                scanner.close();
            }
        }
    }

    private static void handleCommand(String command) throws Exception {
        switch (command) {
            case "?":
                showHelp();
                break;
            case "q":
                handleQuery();
                break;
            case "qq":
                handleQueryAll();
                break;
            case "di":
                handleDistinct();
                break;
            case "s":
                handleSignInGuest();
                break;
            case "s2":
                handleSignInTestUser();
                break;
            case "i":
                handleInsertOne();
                break;
            case "im":
                handleInsertMany();
                break;
            case "w":
                handleWatch();
                break;
            case "quit":
                running = false;
                break;
            default:
                System.out.println("Unknown command. Type ? for help.");
                break;
        }
    }

    private static void showHelp() {
        System.out.println("Available commands:");
        System.out.println("  ?  - Show this help");
        System.out.println("  q  - Query with filter");
        System.out.println("  qq - Query all");
        System.out.println("  di - Distinct types");
        System.out.println("  s  - Sign in as guest");
        System.out.println("  s2 - Sign in as testuser");
        System.out.println("  i  - Insert one");
        System.out.println("  im - Insert many");
        System.out.println("  w  - Watch collection");
        System.out.println("  quit - Exit program");
    }

    private static void handleQuery() {
        try {
            var results = client.query(new TypeReference<java.util.List<test.Entity>>() {}.getType(),
                new QueryParameters.Builder()
                    .collectionname("entities")
                    .query("{\"_type\":\"test\"}")
                    .top(10)
                    .build());
            for (test.Entity item : results) {
                System.out.println("Item: " + item._type + " " + item._id + " " + item.name);
            }
        } catch (Exception e) {
            System.out.println("Query error: " + e.getMessage());
        }
    }

    private static void handleQueryAll() {
        try {
            String jsonResult = client.query(String.class, 
                new QueryParameters.Builder()
                    .collectionname("entities")
                    .query("{}")
                    .build());
            System.out.println("Results: " + jsonResult);
        } catch (Exception e) {
            System.out.println("Query error: " + e.getMessage());
        }
    }

    private static void handleDistinct() {
        try {
            var distinct = client.distinct(
                new DistinctParameters.Builder()
                    .collectionname("entities")
                    .field("_type")
                    .build());
            System.out.println("Distinct types: " + distinct);
        } catch (Exception e) {
            System.out.println("Distinct error: " + e.getMessage());
        }
    }

    private static void handleSignInGuest() {
        try {
            var result = client.signin(new SigninParameters.Builder()
                .username("guest")
                .password("guest")
                .build());
            System.out.println("Signin result: " + result);
        } catch (Exception e) {
            System.out.println("Signin error: " + e.getMessage());
        }
    }

    private static void handleSignInTestUser() {
        try {
            var result = client.signin(new SigninParameters.Builder()
                .username(System.getenv("testusername"))
                .password(System.getenv("testpassword"))
                .build());
            System.out.println("Signin result: " + result);
        } catch (Exception e) {
            System.out.println("Signin error: " + e.getMessage());
        }
    }

    private static void handleInsertOne() {
        try {
            test.Entity entity = new test.Entity();
            entity.name = "CLI Test";
            entity._type = "test";
            
            var result = client.insertOne(test.Entity.class,
                new InsertOneParameters.Builder()
                    .collectionname("entities")
                    .itemFromObject(entity)
                    .build());
            System.out.println("Inserted: " + result._id);
        } catch (Exception e) {
            System.out.println("Insert error: " + e.getMessage());
        }
    }

    private static void handleInsertMany() {
        try {
            String jsonItems = "[{\"_type\":\"test\", \"name\":\"cli-many-1\"}, {\"_type\":\"test\", \"name\":\"cli-many-2\"}]";
            var results = client.insertMany(new TypeReference<java.util.List<test.Entity>>() {}.getType(),
                new InsertManyParameters.Builder()
                    .collectionname("entities")
                    .items(jsonItems)
                    .build());
            for (test.Entity entity : results) {
                System.out.println("Inserted: " + entity._id + " - " + entity.name);
            }
        } catch (Exception e) {
            System.out.println("Insert many error: " + e.getMessage());
        }
    }

    private static void handleWatch() {
        try {
            String watchId = client.watchAsync(
                new WatchParameters.Builder()
                    .collectionname("entities")
                    .build(),
                (event) -> {
                    System.out.println("Watch event: " + event.operation + " on " + event.id);
                    System.out.println("Document: " + event.document);
                });
            System.out.println("Watch started with ID: " + watchId);
            System.out.println("(Events will appear as they happen. Start a new operation to trigger events)");
        } catch (Exception e) {
            System.out.println("Watch error: " + e.getMessage());
        }
    }
}
