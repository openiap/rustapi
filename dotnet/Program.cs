using System;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Text.Json;
using System.Dynamic;

class Program
{
    static async Task Main(string[] args)
    {
        // Set up exception handlers
        AppDomain.CurrentDomain.UnhandledException += (sender, eventArgs) =>
        {
            Console.WriteLine("Unhandled Exception occurred:");
            if (eventArgs.ExceptionObject is Exception ex)
            {
                Console.WriteLine(ex.ToString());
            }
            else
            {
                Console.WriteLine("Unknown exception occurred.");
            }
        };

        TaskScheduler.UnobservedTaskException += (sender, eventArgs) =>
        {
            Console.WriteLine("Unobserved Task Exception occurred:");
            Console.WriteLine(eventArgs.Exception.ToString());
            eventArgs.SetObserved();
        };

        // Initialize the client
        Client client = new Client();
        client.enabletracing("info", "");
        client.enabletracing("openiap=debug", "new");
        await client.connect();
        if (!client.connected())
        {
            client.info("Client connection error: " + client.connectionerror());
            return;
        }
        client.info("Client connection success: " + client.connected());

        // Command handling loop
        Console.WriteLine("? for help");
        string input = "";
        string watchId = "";
        while (input.ToLower() != "quit")
        {
            Console.Write("Enter command: ");
            input = (Console.ReadLine()?.ToLower() ?? "").Trim();

            switch (input)
            {
                case "?":
                    Console.WriteLine("Commands:");
                    Console.WriteLine("quit - Exit the application");
                    Console.WriteLine("s - Sign in as guest");
                    Console.WriteLine("q - Query entities");
                    Console.WriteLine("i - Insert an entity");
                    Console.WriteLine("w - Watch for changes");
                    Console.WriteLine("uw - Unwatch");
                    break;
                case "dis":
                    try {
                        var t = System.Threading.Tasks.Task.Run(() => {
                            client.disconnect();
                        });
                        // client.disconnect();
                    }
                    catch (Exception e)
                    {
                        client.info("Error disconnecting: " + e.Message);
                    }
                    break;
                case "s":
                    try {
                        var (jwt, error, success) = await client.Signin();
                        client.info("Signin JWT: " + jwt);
                    } catch (Exception e) {
                        client.info("Error signing in: " + e.Message);
                    }
                    break;

                case "q":
                    try {
                        string results = await client.Query("entities", "{}", "{\"name\": 1}");
                        client.info("Query results: " + results);
                    }
                    catch (System.Exception e)
                    {
                        client.info("Error querying: " + e.Message);
                    }
                    break;

                case "i":
                    try {
                        var workitem = new Workitem { name = "test from dotnet", payload = "{\"name\": \"test from dotnet\"}" };
                        var insertResult = await client.InsertOne("entities", JsonSerializer.Serialize(workitem));
                        client.info("Insert result: " + insertResult);
                    }
                    catch (System.Exception e)
                    {
                        client.info("Error inserting: " + e.Message);
                    }
                    break;

                case "im":
                    try {
                        var items = new[] {
                            new { name = "Allan", _type = "test" },
                            new { name = "Allan2", _type = "test" }
                        };
                        var insertManyResult = await client.InsertMany("entities", JsonSerializer.Serialize(items));
                        if (insertManyResult == null)
                        {
                            client.info("Failed to insert many.");
                        }
                        else
                        {
                            client.info("Inserted items: " + insertManyResult);
                        }
                    }
                    catch (Exception e)
                    {
                        client.info("Error inserting many: " + e.Message);
                    }
                    break;
                case "d":
                    try {
                        var deleteResult = await client.download("fs.files", "65a3aaf66d52b8c15131aebd");
                        if (deleteResult == null)
                        {
                            client.info("Failed to download.");
                        }
                        else
                        {
                            client.info("Downloaded as: " + deleteResult);
                        }
                    }
                    catch (Exception e)
                    {
                        client.info("Download error: " + e.Message);
                    }
                    break;
                case "u":
                    try {
                        var uploadResult = await client.upload("train.csv", "train.csv");
                        if (uploadResult == null)
                        {
                            client.info("Failed to upload.");
                        }
                        else
                        {
                            client.info("Uploaded as: " + uploadResult);
                        }
                    }
                    catch (Exception e)
                    {
                        client.info("Error disconnecting: " + e.Message);
                    }
                    break;
                case "w":
                    try {
                        watchId = await client.watch( "entities", "[]", e => {
                            client.info("Watch event: " + e.operation + " " + e.id, e.document);
                        });
                        client.info("Watch registered with id: " + watchId);
                    }
                    catch (Exception e)
                    {
                        client.info("Watch error: " + e.Message);
                    }
                    break;
                case "uw":
                    try {
                        if (string.IsNullOrEmpty(watchId))
                        {
                            client.info("No watch ID to remove.");
                            break;
                        }
                        client.off_client_event(watchId);
                        client.info("Removed watch ID: " + watchId);
                    }
                    catch (Exception e)
                    {
                        client.info("Watch error: " + e.Message);                        
                    }
                    break;
                case "r":
                    try {
                        var queueId = await client.RegisterQueue("test2queue", e => {
                            client.info("Queue event received from " + e.queuename + " with data: " + e.data);
                        });
                        client.info("Queue registered with id: " + queueId);
                    }
                    catch (Exception e)
                    {
                        client.info("Error disconnecting: " + e.Message);                        
                    }
                    break;
                case "m":
                    try {
                        var message = "{\"message\": \"Hello from dotnet\"}";
                        await client.QueueMessage(message, "test2queue", striptoken: true);
                    }
                    catch (Exception e)
                    {
                        client.info("Error disconnecting: " + e.Message);
                    }
                    break;
                case "quit":
                    try {
                        client.Dispose();
                        Console.WriteLine("Client disposed. Exiting...");
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Error disposing client: " + e.Message);
                    }
                    break;

                default:
                    // Console.WriteLine("Unknown command. Type '?' for help.");
                    break;
            }
        }
    }
}
