package io.openiap;

import java.util.Scanner;
import java.util.List;
import java.util.Arrays;
import com.fasterxml.jackson.core.type.TypeReference;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.Future;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class cli {
    private static Client client;
    private static volatile boolean running = true;
    private static Scanner scanner;
    private static ExecutorService executor;
    private static Future<?> runningTask;
    private static AtomicBoolean taskRunning = new AtomicBoolean(false);

    public static void main(String[] args) {
        System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");
        client = new Client(libpath);
        scanner = new Scanner(System.in);
        executor = Executors.newSingleThreadExecutor();
        try {
            client.enableTracing("openiap=trace", "");
            // client.enableTracing("openiap=info", "");
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
            if (executor != null) {
                executor.shutdownNow();
            }
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
            case "t":
                test.RunAll();
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
            case "p":
                handlePushWorkitem();
                break;
            case "p2":
                handlePushWorkitem2();
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
            case "st":
                handleStartTask();
                break;
            case "st2":
                handleStartTask2();
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
        System.out.println("  t  - Run all tests");
        System.out.println("  q  - Query with filter");
        System.out.println("  qq - Query all");
        System.out.println("  di - Distinct types");
        System.out.println("  p  - PushWorkitem");
        System.out.println("  p2  - PushWorkitem second test");
        System.out.println("  s  - Sign in as guest");
        System.out.println("  s2 - Sign in as testuser");
        System.out.println("  i  - Insert one");
        System.out.println("  im - Insert many");
        System.out.println("  w  - Watch collection");
        System.out.println("  st  - Start/stop task (workitem processing)");
        System.out.println("  st2 - Start/stop task (continuous testing)");
        System.out.println("  quit - Exit program");
    }

    private static void handleQuery() {
        try {
            List<test.Entity> results = client.query(new TypeReference<List<test.Entity>>() {}.getType(),
                new QueryParameters.Builder()
                    .collectionname("entities")
                    .query("{\"_type\":\"test\"}")
                    .top(10)
                    .build());
            if (results != null) {
                for (test.Entity item : results) {
                    System.out.println("Item: " + item._type + " " + item._id + " " + item.name);
                }
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

    private static void handlePushWorkitem() {
        try {
            test.Entity entity = new test.Entity();
            entity.name = "CLI Test";
            entity._type = "test";
            var result = client.pushWorkitem(new PushWorkitem.Builder("q2")
                .name("CLI Test")
                //.payload("{\"_type\":\"test\"}")
                .itemFromObject(entity)
                // .nextrun(System.currentTimeMillis() + 10000)
                .priority(1)
                .build());
            System.out.println("Pushed workitem: " + result);
        } catch (Exception e) {
            System.out.println("PushWorkitem error: " + e.getMessage());
        }
    }

    private static void handlePushWorkitem2() {
        try {
            // Create workitem using the builder
            Workitem workitem = new Workitem.Builder()
                .name("Test Workitem")
                .payload("{\"test\": \"value\"}")
                .priority(1)
                .wiq("q2")
                .build();
            // Push the workitem and get back a typed response
            Workitem result = client.pushWorkitem(Workitem.class, new PushWorkitem.Builder("q2")
                .name(workitem.name)
                .itemFromObject(workitem)
                .priority(workitem.priority)
                .build());
            System.out.println("Pushed workitem: " + result.id + " name: " + result.name);
            if (result.files != null) {
                System.out.println("Files: " + result.files.size());
                for (WorkitemFile f : result.files) {
                    System.out.println("  - " + f.filename + " (id: " + f.id + ")");
                }
            }
        } catch (Exception e) {
            System.out.println("PushWorkitem error: " + e.getMessage());
            e.printStackTrace();
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
            test.Entity result = client.insertOne(test.Entity.class,
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
            List<test.Entity> results = client.insertMany(new TypeReference<List<test.Entity>>() {}.getType(),
                new InsertManyParameters.Builder()
                    .collectionname("entities")
                    .items(jsonItems)
                    .build());
            if (results != null) {
                for (test.Entity entity : results) {
                    System.out.println("Inserted: " + entity._id + " - " + entity.name);
                }
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

    @SuppressWarnings("unused")
    private static void handleStartTask() {
        if (taskRunning.get()) {
            System.out.println("Stopping running task.");
            if (runningTask != null) {
                runningTask.cancel(true);
            }
            taskRunning.set(false);
            return;
        }

        taskRunning.set(true);
        runningTask = executor.submit(() -> {
            System.out.println("Task started, begin loop...");
            int x = 0;
            while (taskRunning.get() && !Thread.currentThread().isInterrupted()) {
                try {
                    x++;
                    // var workitem = client.popWorkitem(new PopWorkitemParameters.Builder()
                    //         .workqueue("q2")
                    //         .build());
                    // Thread.sleep(1);
                    // if (workitem != null) {
                    //     System.out.println("Updating " + workitem.id + " " + workitem.name);
                    //     workitem.state = "successful";
                    //     workitem = client.updateWorkitem(workitem);
                    // } else {
                    //     if (x % 500 == 0) {
                    //         System.out.println("No new workitem " + new java.util.Date());
                    //         System.gc();
                    //     }
                    // }
                } catch (Exception e) {
                    System.out.println("Error: " + e.toString());
                }
            }
            System.out.println("Task canceled.");
        });
    }

    private static void handleStartTask2() {
        if (taskRunning.get()) {
            System.out.println("Stopping running task.");
            if (runningTask != null) {
                runningTask.cancel(true);
            }
            taskRunning.set(false);
            return;
        }

        taskRunning.set(true);
        runningTask = executor.submit(() -> {
            System.out.println("Task started, begin loop...");
            int x = 0;
            while (taskRunning.get() && !Thread.currentThread().isInterrupted()) {
                try {
                    x++;
                    Thread.sleep(1);
                    test.RunAll();
                    if (x % 500 == 0) {
                        System.out.println("No new workitem " + new java.util.Date());
                        System.gc();
                    }
                } catch (Exception e) {
                    System.out.println("Error: " + e.toString());
                }
            }
            System.out.println("Task canceled.");
        });
    }
}

