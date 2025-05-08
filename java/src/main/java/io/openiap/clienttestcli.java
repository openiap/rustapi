package io.openiap;

import java.util.Scanner;
import java.util.List;
import java.io.File;
import java.util.Arrays;
import com.fasterxml.jackson.core.type.TypeReference;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.Future;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class clienttestcli {
    private static Client client;
    private static volatile boolean running = true;
    private static Scanner scanner;
    private static ExecutorService executor;
    private static Future<?> runningTask;
    private static AtomicBoolean taskRunning = new AtomicBoolean(false);
    private static ExecutorService replyExecutor;
    private static ScheduledExecutorService f64Handler = null;
    private static ScheduledExecutorService u64Handler = null;
    private static ScheduledExecutorService i64Handler = null;

    public static void main(String[] args) {
        // System.out.println("CLI initializing...");
        String libpath = NativeLoader.loadLibrary("openiap");
        client = new Client(libpath);
        scanner = new Scanner(System.in);
        executor = Executors.newSingleThreadExecutor();
        replyExecutor = Executors.newFixedThreadPool(4); // Adjust pool size as needed
        try {
            // client.enableTracing("openiap=trace", "new");
            client.enableTracing("openiap=info", "");
            client.start();
            client.connect("");
            client.info("Connected to server, successfully.");
            System.out.println("? for help");
            if(System.getenv("oidc_config") != null && System.getenv("oidc_config") != "") {
                handleStartTask();
            }
            while (running) {
                System.out.print("> ");
                String command = scanner.nextLine().trim().toLowerCase();
                handleCommand(command);
            }
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            cleanup();
        }
    }

    private static void handleCommand(String command) throws Exception {
        switch (command) {
            case "?":
                showHelp();
                break;
            case "t":
                clienttestclass.RunAll();
                break;
            case "q":
                handleQuery();
                break;
            case "m":
                handleQueueMessage();
                break;
            case "r2":
                handleRPCMessage();
                break;
            case "r3":
                handleRPCASyncMessage();
                break;
            case "r":
                handleRegisterQueue();
                break;
            case "qq":
                handleQueryAll();
                break;
            case "di":
                handleDistinct();
                break;
            case "p":
                handlePopWorkitem();
                break;
            case "p1":
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
            case "pd":
                handleDeleteWorkitem();
                break;
            case "o":
                handleF64Observable();
                break;
            case "o2":
                handleU64Observable();
                break;
            case "o3":
                handleI64Observable();
                break;
            case "cc":
                handleCustomCommand();
                break;
            case "quit":
                running = false;
                break;
            case "g":
                System.out.println("Client state: " + client.getState());
                break;
            case "rpa":
                handleInvokeOpenRPA();
                break;
            default:
                System.out.println("Unknown command. Type ? for help.");
                break;
        }
    }

    private static void showHelp() {
        System.out.println("Available commands:");
        System.out.println("  ?    - Show this help");
        System.out.println("  t    - Run all tests");
        System.out.println("  q    - Query with filter");
        System.out.println("  qq   - Query all");
        System.out.println("  di   - Distinct types");
        System.out.println("  p1   - PushWorkitem");
        System.out.println("  p2   - PushWorkitem second test");
        System.out.println("  p    - Pop/Update workitem state");
        System.out.println("  pd   - Pop/Delete workitem");
        System.out.println("  s    - Sign in as guest");
        System.out.println("  s2   - Sign in as testuser");
        System.out.println("  i    - Insert one");
        System.out.println("  im   - Insert many");
        System.out.println("  w    - Watch collection");
        System.out.println("  st   - Start/stop task (workitem processing)");
        System.out.println("  st2  - Start/stop task (continuous testing)");
        System.out.println("  o    - Toggle f64 observable gauge");
        System.out.println("  o2   - Toggle u64 observable gauge");
        System.out.println("  o3   - Toggle i64 observable gauge");
        System.out.println("  r    - Register queue");
        System.out.println("  m    - Send message to queue");
        System.out.println("  r2   - Send RPC message");
        System.out.println("  cc   - Get Clients using custom_command");
        System.out.println("  rpa  - Invoke \"Who am I\" on robot \"allan5\"");
        System.out.println("  quit - Exit program");
    }

    private static void handleQuery() {
        try {
            List<clienttestclass.Entity> results = client.query(new TypeReference<List<clienttestclass.Entity>>() {}.getType(),
                new QueryParameters.Builder()
                    .collectionname("entities")
                    .query("{\"_type\":\"test\"}")
                    .top(10)
                    .build());
            if (results != null) {
                for (clienttestclass.Entity item : results) {
                    System.out.println("Item: " + item._type + " " + item._id + " " + item.name);
                }
            }
        } catch (Exception e) {
            System.out.println("Query error: " + e.getMessage());
        }
    }
    private static void handleQueueMessage() {
        try {
            client.queueMessage(
                new QueueMessageParameters.Builder()
                    .queuename("test2queue")
                    .striptoken(true)
                    .message("{\"find\":\"Allan\"}")
                    .build()
            );
        } catch (Exception e) {
            System.out.println("QueueMessage error: " + e.getMessage());
        }
    }
    private static void handleRPCMessage() {
        // sparn thread to handle the reply
        new Thread(() -> {
            try {
                var result = client.rpc(
                    new QueueMessageParameters.Builder()
                    .queuename("test2queue")
                    .striptoken(true)
                    .message("{\"find\":\"Allan\"}")
                    .build(),
                    1
                );
                System.out.println("RPC message sent: " + result);
            } catch (Exception e) {
                System.out.println("RPCMessage error: " + e.getMessage());
            }
        }).start();
    }
    private static void handleRPCASyncMessage() {
        // sparn thread to handle the reply
        new Thread(() -> {
            try {
                var result = client.rpcAsync(
                    new QueueMessageParameters.Builder()
                    .queuename("test2queue")
                    .striptoken(true)
                    .message("{\"find\":\"Allan\"}")
                    .build(),
                    10
                );
                System.out.println("RPC message sent: " + result);
            } catch (Exception e) {
                System.out.println("RPCMessage error: " + e.getMessage());
            }
        }).start();
    }
    private static void handleRegisterQueue() {
        try {
            var queuename = client.registerQueueAsync(
                new RegisterQueueParameters.Builder()
                    .queuename("test2queue")
                    .build(),
                (eventPtr) -> {
                    RegisterQueueResponseWrapper.QueueEventWrapper wrapper = new RegisterQueueResponseWrapper.QueueEventWrapper(eventPtr);
                    wrapper.read();
                    QueueEvent event = new QueueEvent();
                    event.setData(wrapper.data);
                    event.setQueuename(wrapper.queuename);
                    event.setReplyto(wrapper.replyto);
                    System.out.println("Received message on queue: " + event.getQueuename() +
                                     ", from: " + event.getReplyto() +
                                     ", data: " + event.getData());
                    return "{\"payload\": {\"response\":\"Bettina\"}}";
                }
            );
            System.out.println("Registered queue as: " + queuename);
        } catch (Exception e) {
            System.out.println("RegisterQueue error: " + e.getMessage());
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
            clienttestclass.Entity entity = new clienttestclass.Entity();
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
            List<String> files = Arrays.asList("testfile.csv"
            // , "/home/allan/Documents/assistant-linux-x86_64.AppImage"
            );
            clienttestclass.Entity entity = new clienttestclass.Entity();
            entity.name = "CLI Test";
            entity._type = "test";

            // Create builder and build workitem
            PushWorkitem.Builder builder = new PushWorkitem.Builder("q2")
                .name(entity.name)
                .itemFromObject(entity)
                .files(files);

            PushWorkitem pushWorkitem = builder.build();
            try {
                // Push the workitem and get back a typed response
                Workitem result = client.pushWorkitem(Workitem.class, pushWorkitem);
                
                System.out.println("Pushed workitem: " + result.id + " name: " + result.name);
                if (result.files != null) {
                    System.out.println("Files: " + result.files.size());
                    for (WorkitemFile f : result.files) {
                        System.out.println("  - " + f.filename + " (id: " + f.id + ")");
                    }
                }
            } finally {
                // Clean up after push is complete
                builder.cleanup();
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
            clienttestclass.Entity entity = new clienttestclass.Entity();
            entity.name = "CLI Test";
            entity._type = "test";
            clienttestclass.Entity result = client.insertOne(clienttestclass.Entity.class,
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
            List<clienttestclass.Entity> results = client.insertMany(new TypeReference<List<clienttestclass.Entity>>() {}.getType(),
                new InsertManyParameters.Builder()
                    .collectionname("entities")
                    .items(jsonItems)
                    .build());
            if (results != null) {
                for (clienttestclass.Entity entity : results) {
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

    private static void handleStartTask() {
        if (taskRunning.get()) {
            System.out.println("Stopping running task.");
            if (runningTask != null) {
                runningTask.cancel(true);
            }
            taskRunning.set(false);
            return;
        }
        System.out.println("Starting task...");
        taskRunning.set(true);
        runningTask = executor.submit(() -> {
            System.out.println("Task started, begin loop...");
            Runtime runtime = Runtime.getRuntime();
            int x = 0;
            while (taskRunning.get() && !Thread.currentThread().isInterrupted()) {
                try {
                    x++;
                    // Add memory usage logging every 100 iterations
                    if (x % 100 == 0) {
                        long usedMemory = (runtime.totalMemory() - runtime.freeMemory()) / 1024 / 1024;
                        System.out.println("Memory usage: " + usedMemory + "MB");
                    }
                    
                    PopWorkitem popRequest = new PopWorkitem.Builder("q2").build();
                    Workitem workitem = client.popWorkitem(Workitem.class, popRequest, "downloads");
                    
                    Thread.sleep(1);
                    
                    if (workitem != null) {
                        System.out.println("Updating " + workitem.id + " " + workitem.name);
                        workitem.state = "successful";
                        UpdateWorkitem.Builder builder = new UpdateWorkitem.Builder(workitem);
                        UpdateWorkitem updateRequest = builder.build();
                        workitem = client.updateWorkitem(Workitem.class, updateRequest);
                    } else {
                        if (x % 500 == 0) {
                            System.out.println("No new workitem " + new java.util.Date());
                            System.gc();
                        }
                    }
                } catch (Exception e) {
                    System.out.println("Error in task loop: ");
                    e.printStackTrace(System.out);  // Print full stack trace
                    try {
                        Thread.sleep(5000); // Add delay after error
                    } catch (InterruptedException ie) {
                        Thread.currentThread().interrupt();
                        break;
                    }
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
                    clienttestclass.RunAll();
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

    private static void handlePopWorkitem() {
        try {
            // ensure folder downloads exits
            File downloadsFolder = new File("downloads");
            if (!downloadsFolder.exists()) {
                downloadsFolder.mkdir();
            }
            PopWorkitem popRequest = new PopWorkitem.Builder("q2").build();
            Workitem workitem = client.popWorkitem(Workitem.class, popRequest, "downloads");
            
            if (workitem != null) {
                System.out.println("Updating workitem: " + workitem.id);
                
                // Update the workitem state
                workitem.state = "successful";
                workitem.name = "Updated by CLI";
                
                // Create update request
                UpdateWorkitem.Builder builder = new UpdateWorkitem.Builder(workitem);
                builder.files(
                    Arrays.asList("/home/allan/Documents/export.csv")
                    // Arrays.asList("/home/allan/Documents/export.csv", "downloads/testfile.csv")
                );
                UpdateWorkitem updateRequest = builder.build();
                
                try {
                    // Send update
                    workitem = client.updateWorkitem(Workitem.class, updateRequest);
                    System.out.println("Updated workitem state to: " + workitem.state);
                } finally {
                    builder.cleanup();
                }
            } else {
                System.out.println("No workitem available to update");
            }
        } catch (Exception e) {
            System.out.println("UpdateWorkitem error: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void handleDeleteWorkitem() {
        try {
            // First pop a workitem to delete
            PopWorkitem popRequest = new PopWorkitem.Builder("q2").build();
            Workitem workitem = client.popWorkitem(Workitem.class, popRequest, "downloads");
            
            if (workitem != null) {
                System.out.println("Deleting workitem: " + workitem.id);
                
                // Create delete request
                DeleteWorkitem deleteRequest = new DeleteWorkitem.Builder(workitem.id).build();
                
                // Send delete
                boolean success = client.deleteWorkitem(deleteRequest);
                if (success) {
                    System.out.println("Workitem deleted successfully");
                } else {
                    System.out.println("Failed to delete workitem");
                }
            } else {
                System.out.println("No workitem available to delete");
            }
        } catch (Exception e) {
            System.out.println("DeleteWorkitem error: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private static void handleF64Observable() {
        if (f64Handler != null) {
            client.disable_observable_gauge("test_f64");
            f64Handler.shutdown();
            f64Handler = null;
            client.info("stopped test_f64");
            return;
        }
        
        client.set_f64_observable_gauge("test_f64", 42.7, "test");
        client.info("started test_f64 to 42.7");
        
        f64Handler = Executors.newSingleThreadScheduledExecutor();
        f64Handler.scheduleAtFixedRate(() -> {
            double random = Math.random() * 50;
            client.info("Setting test_f64 to " + random);
            client.set_f64_observable_gauge("test_f64", random, "test");
        }, 0, 30, TimeUnit.SECONDS);
    }

    private static void handleU64Observable() {
        if (u64Handler != null) {
            client.disable_observable_gauge("test_u64");
            u64Handler.shutdown();
            u64Handler = null;
            client.info("stopped test_u64");
            return;
        }
        
        client.set_u64_observable_gauge("test_u64", 42, "test");
        client.info("started test_u64 to 42");
        
        u64Handler = Executors.newSingleThreadScheduledExecutor();
        u64Handler.scheduleAtFixedRate(() -> {
            long random = (long)(Math.random() * 50);
            client.info("Setting test_u64 to " + random);
            client.set_u64_observable_gauge("test_u64", random, "test");
        }, 0, 30, TimeUnit.SECONDS);
    }

    private static void handleI64Observable() {
        if (i64Handler != null) {
            client.disable_observable_gauge("test_i64");
            i64Handler.shutdown();
            i64Handler = null;
            client.info("stopped test_i64");
            return;
        }
        
        client.set_i64_observable_gauge("test_i64", 42, "test");
        client.info("started test_i64 to 42");
        
        i64Handler = Executors.newSingleThreadScheduledExecutor();
        i64Handler.scheduleAtFixedRate(() -> {
            long random = (long)(Math.random() * 50);
            client.info("Setting test_i64 to " + random);
            client.set_i64_observable_gauge("test_i64", random, "test");
        }, 0, 30, TimeUnit.SECONDS);
    }

    private static void handleCustomCommand() {
        try {
            String result = client.customCommand(
                new CustomCommandParameters.Builder()
                    .command("getclients")
                    .build(),
                    10
            );
            System.out.println("CustomCommand result: " + result);
            // Optionally, parse JSON and print details if needed
            // ObjectMapper mapper = new ObjectMapper();
            // List<?> clients = mapper.readValue(result, List.class);
            // System.out.println("Client count: " + clients.size());
        } catch (Exception e) {
            System.out.println("CustomCommand error: " + e.getMessage());
        }
    }
    private static void handleInvokeOpenRPA() {
        try {
            String result = client.invokeOpenRPA(
                new InvokeOpenRPAParameters.Builder()
                    .robotid("5ce94386320b9ce0bc2c3d07")
                    .workflowid("5e0b52194f910e30ce9e3e49")
                    .payload("{\"test\":\"test\"}")
                    .rpc(false)
                    .build(),
                    10
            );
            System.out.println("InvokeOpenRPA result: " + result);
        } catch (Exception e) {
            System.out.println("InvokeOpenRPA error: " + e.getMessage());
        }
    }

    private static void cleanup() {
        if (executor != null) {
            executor.shutdownNow();
        }
        if (client != null) {
            client.disconnect();
        }
        if (scanner != null) {
            scanner.close();
        }
        if (replyExecutor != null) {
            replyExecutor.shutdown();
        }
        if (f64Handler != null) {
            f64Handler.shutdownNow();
        }
        if (u64Handler != null) {
            u64Handler.shutdownNow();
        }
        if (i64Handler != null) {
            i64Handler.shutdownNow();
        }
    }
}

