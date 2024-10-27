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
            input = Console.ReadLine().ToLower();

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

                case "s":
                    var (jwt, error, success) = await client.Signin();
                    client.info("Signin JWT: " + jwt);
                    break;

                case "q":
                    string results = await client.Query("entities", "{}", "{\"name\": 1}");
                    client.info("Query results: " + results);
                    break;

                case "i":
                    var workitem = new Workitem { name = "test from dotnet", payload = "{\"name\": \"test from dotnet\"}" };
                    var insertResult = await client.InsertOne("entities", JsonSerializer.Serialize(workitem));
                    client.info("Insert result: " + insertResult);
                    break;

                case "im":
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
                    break;

                case "d":
                    var deleteResult = await client.download("fs.files", "65a3aaf66d52b8c15131aebd");
                    if (deleteResult == null)
                    {
                        client.info("Failed to download.");
                    }
                    else
                    {
                        client.info("Downloaded as: " + deleteResult);
                    }
                    break;

                case "u":
                    Console.Write("Enter file path to upload: ");
                    var uploadResult = await client.upload("train.csv", "train.csv");
                    if (uploadResult == null)
                    {
                        client.info("Failed to upload.");
                    }
                    else
                    {
                        client.info("Uploaded as: " + uploadResult);
                    }
                    break;

                case "w":
                    watchId = await client.watch( "entities", "[]", e => {
                        client.info("Watch event: " + e.operation + " " + e.id, e.document);
                    });
                    client.info("Watch registered with id: " + watchId);
                    break;

                case "uw":
                    if (string.IsNullOrEmpty(watchId))
                    {
                        client.info("No watch ID to remove.");
                        break;
                    }
                    client.off_client_event(watchId);
                    client.info("Removed watch ID: " + watchId);
                    break;
                case "r":
                    var queueId = await client.RegisterQueue("test2queue", e => {
                        client.info("Queue event received from " + e.queuename + " with data: " + e.data);
                    });
                    client.info("Queue registered with id: " + queueId);
                    break;
                case "m":
                    var message = "{\"message\": \"Hello from dotnet\"}";
                    await client.QueueMessage(message, "test2queue", striptoken: true);
                    break;
                case "quit":
                    client.Dispose();
                    Console.WriteLine("Client disposed. Exiting...");
                    break;

                default:
                    // Console.WriteLine("Unknown command. Type '?' for help.");
                    break;
            }
        }
    }
}
