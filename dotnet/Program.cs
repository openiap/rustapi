using System;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Text.Json;
using System.Dynamic;


class Collection 
{
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
    public string name { get; set; }
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
}
class Base
{
#pragma warning disable CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
    public string _id { get; set; }
    public string _type { get; set; }
    public string name { get; set; }
#pragma warning restore CS8618 // Non-nullable field must contain a non-null value when exiting constructor. Consider adding the 'required' modifier or declaring as nullable.
}
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
        // client.enabletracing("info", "");
        // client.enabletracing("openiap=debug", "new");
        await client.connect();
        if (!client.connected())
        {
            Console.WriteLine("Client connection error: " + client.connectionerror());
            return;
        }
        Console.WriteLine("Client connection success: " + client.connected());

        // Command handling loop
        Console.WriteLine("? for help");
        string input = "";
        string watchId = "";
        var cancellationTokenSource = new CancellationTokenSource();
        CancellationToken token = cancellationTokenSource.Token;
        cancellationTokenSource.Cancel();
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
                case "0":
                    client.disabletracing();
                    break;
                case "1":
                    client.enabletracing("openiap=info", "");
                    break;
                case "2":
                    client.enabletracing("openiap=debug", "new");
                    break;
                case "3":
                    client.enabletracing("openiap=trace", "new");
                    break;
                case "4":
                    client.enabletracing("trace", "new");
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
                        Console.WriteLine("Error disconnecting: " + e.Message);
                    }
                    break;
                case "gc":
                    GC.Collect();
                    break;
                case "t":
                    Task.Run(async () => {
                        await test.Test(client);
                    });
                    break;
                case "st":
                    if(!token.IsCancellationRequested) {
                        Console.WriteLine("Stopping running task.");
                        cancellationTokenSource.Cancel();
                        break;
                    }
                    cancellationTokenSource = new CancellationTokenSource();
                    token = cancellationTokenSource.Token;
                    int x = 0;
                    var task = Task.Run(async () => 
                    {
                        Console.WriteLine("Task started, begin loop...");
                        while (!token.IsCancellationRequested)
                        {
                            try
                            {
                                x++;
                                var workitem = await client.PopWorkitem("q2");
                                Thread.Sleep(1);
                                if (workitem != null)
                                {                          

                                    Console.WriteLine("Updating ", workitem.id, workitem.name);
                                    workitem.state = "successful";
                                    workitem = await client.UpdateWorkitem(workitem, new string[] { });
                                }
                                else
                                {
                                    if (x % 500 == 0)
                                    {
                                        Console.WriteLine("No new workitem", DateTime.Now);
                                        GC.Collect();
                                    }
                                }
                            }
                            catch (System.Exception ex)
                            {   
                                Console.WriteLine("Error: ", ex.ToString());
                            }
                        }
                        Console.WriteLine("Task canceled.");
                    }, token);
                    break;
                case "s":
                    try {
                        var (jwt, error, success) = await client.Signin();
                        Console.WriteLine("Signin JWT: " + jwt);
                    } catch (Exception e) {
                        Console.WriteLine("Error signing in: " + e.Message);
                    }
                    break;

                case "q2":
                    try {
                        var t = Task.Run(async () => {
                            var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
                            Console.WriteLine("Query returned " + results.Count + " results.");
                            for (int i = 0; i < results.Count; i++)
                            {
                                Console.WriteLine(results[i]._id, " ", results[i].name);
                                if (i > 10) {
                                    break;
                                }
                            }
                        });
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error querying: " + e.Message);
                    }
                    break;
                case "qq":
                    try {
                        var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
                        Console.WriteLine("Query returned " + results.Count + " results.");
                        for (int i = 0; i < results.Count; i++)
                        {
                            Console.WriteLine(results[i]._id, " ", results[i].name);
                            if (i > 10) {
                                break;
                            }
                        }

                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error querying: " + e.Message);
                    }
                    break;
                case "q":

                    try {
                        var results = Task.Run(async () => {
                            var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
                            return results;
                        }).Result;
                        Console.WriteLine("Query returned " + results.Count + " results.");
                        for (int i = 0; i < results.Count; i++)
                        {
                            Console.WriteLine(results[i]._id, " ", results[i].name);
                            if (i > 10) {
                                break;
                            }
                        }

                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error querying: " + e.Message);
                    }
                    break;
                case "a":
                    try {
                        var results = await client.Aggregate<List<Base>>("entities", "[{\"$match\": {\"_type\": \"test\"}}]");
                        Console.WriteLine("Aggregation returned " + results.Count + " results.");
                        for (int i = 0; i < results.Count; i++)
                        {
                            Console.WriteLine(results[i]._id, " ", results[i].name);
                            if (i > 10) {
                                break;
                            }
                        }
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error querying: " + e.Message);
                    }
                    break;

                case "l":
                    try {
                        // string results = await client.ListCollections<string>();
                        // Console.WriteLine("Query results: " + results);
                        var results = await client.ListCollections<List<Collection>>();
                        Console.WriteLine("ListCollections returned " + results.Count + " results.");
                        for (int i = 0; i < results.Count; i++)
                        {
                            Console.WriteLine(results[i].name);
                            if (i > 10) {
                                break;
                            }
                        }
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error querying: " + e.Message);
                    }
                    break;
                case "cc":
                    try {
                        await client.CreateCollection("testdotnetcollection");
                        Console.WriteLine("Create testdotnetcollection Collection success.");
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error creating collection: " + e.Message);
                    }
                    break;
                case "cc2":
                    try {
                        await client.CreateCollection("testdotnettscollection", timeseries: 
                        new Client.ColTimeseriesWrapper("ts", "", "minutes"));
                        Console.WriteLine("Create testdotnettscollection Collection success.");
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error creating collection: " + e.Message);
                    }
                    break;
                case "cc3":
                    try {
                        await client.CreateCollection("testdotnetcollection");
                        Console.WriteLine("Create testdotnetcollection Collection success.");
                        await client.CreateIndex("testdotnetcollection", "{\"name\": 1}");
                        Console.WriteLine("Create index on testdotnetcollection Collection success.");
                        await client.InsertOne<Base>("testdotnetcollection", "{\"name\": \"test from dotnet\", \"_type\": \"test\"}");
                        Console.WriteLine("Insert test entity into testdotnetcollection Collection success.");
                        var results = await client.GetIndexes<string>("testdotnetcollection");
                        Console.WriteLine("GetIndexes ", results);
                        await client.DropIndex("testdotnetcollection", "name_1");
                        Console.WriteLine("Drop index on testdotnetcollection Collection success.");
                        results = await client.GetIndexes<string>("testdotnetcollection");
                        Console.WriteLine("GetIndexes ", results);
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error creating collection: " + e.Message);
                    }
                    break;
                case "dc":
                    try {
                        await client.DropCollection("testdotnetcollection");
                        Console.WriteLine("Drop testdotnetcollection Collection success.");
                        await client.DropCollection("testdotnettscollection");
                        Console.WriteLine("Drop testdotnettscollection Collection success.");
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error dropping collection: " + e.Message);
                    }
                    break;
                case "gi":
                    try {
                        var result = await client.GetIndexes<string>("entities");
                        Console.WriteLine("GetIndexes result: " + result);
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error inserting: " + e.Message);
                    }
                    break;

                case "i":
                    try {
                        var item = new { name = "test from dotnet", _type = "test" };
                        var result = await client.InsertOne<Base>("entities", JsonSerializer.Serialize(item));
                        Console.WriteLine(result._id, " ", result.name);
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error inserting: " + e.Message);
                    }
                    break;
                case "u1":
                    try {
                        var item = new Base { name = "test from dotnet", _type = "test" };
                        item = await client.InsertOne<Base>("entities", JsonSerializer.Serialize(item));
                        Console.WriteLine(item._id, " ", item.name);
                        item.name = "updated from dotnet";
                        var result = await client.UpdateOne<Base>("entities", JsonSerializer.Serialize(item));
                        Console.WriteLine(result._id, " ", result.name);
                    }
                    catch (System.Exception e)
                    {
                        Console.WriteLine("Error updating: " + e.Message);
                    }
                    break;

                case "im":
                    try {
                        var items = new[] {
                            new { name = "Allan", _type = "test" },
                            new { name = "Allan2", _type = "test" }
                        };
                        // var insertManyResult = await client.InsertMany("entities", JsonSerializer.Serialize(items));
                        // if (insertManyResult == null)
                        // {
                        //     Console.WriteLine("Failed to insert many.");
                        // }
                        // else
                        // {
                        //     Console.WriteLine("Inserted items: " + insertManyResult);
                        // }
                        var results = await client.InsertMany<List<Base>>("entities", JsonSerializer.Serialize(items));
                        for (int i = 0; i < results.Count; i++)
                        {
                            Console.WriteLine(results[i]._id, " ", results[i].name);
                        }
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Error inserting many: " + e.Message);
                    }
                    break;
                case "d":
                    try {
                        var deleteResult = await client.download("fs.files", "65a3aaf66d52b8c15131aebd");
                        if (deleteResult == null)
                        {
                            Console.WriteLine("Failed to download.");
                        }
                        else
                        {
                            Console.WriteLine("Downloaded as: " + deleteResult);
                        }
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Download error: " + e.Message);
                    }
                    break;
                case "u":
                    try {
                        var uploadResult = await client.upload("train.csv", "train.csv");
                        if (uploadResult == null)
                        {
                            Console.WriteLine("Failed to upload.");
                        }
                        else
                        {
                            Console.WriteLine("Uploaded as: " + uploadResult);
                        }
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Error disconnecting: " + e.Message);
                    }
                    break;
                case "w":
                    try {
                        watchId = await client.watch( "entities", "[]", e => {
                            Console.WriteLine("Watch event: " + e.operation + " " + e.id, e.document);
                        });
                        Console.WriteLine("Watch registered with id: " + watchId);
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Watch error: " + e.Message);
                    }
                    break;
                case "uw":
                    try {
                        if (string.IsNullOrEmpty(watchId))
                        {
                            Console.WriteLine("No watch ID to remove.");
                            break;
                        }
                        client.off_client_event(watchId);
                        Console.WriteLine("Removed watch ID: " + watchId);
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Watch error: " + e.Message);                        
                    }
                    break;
                case "r":
                    try {
                        var queueId = await client.RegisterQueue("test2queue", e => {
                            Console.WriteLine("Queue event received from " + e.queuename + " with data: " + e.data);
                        });
                        Console.WriteLine("Queue registered with id: " + queueId);
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Error disconnecting: " + e.Message);                        
                    }
                    break;
                case "m":
                    try {
                        var message = "{\"message\": \"Hello from dotnet\"}";
                        await client.QueueMessage(message, "test2queue", striptoken: true);
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine("Error disconnecting: " + e.Message);
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
