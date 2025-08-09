using System;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Text.Json;
using System.Dynamic;
using OpenIAP;

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
    Console.WriteLine($"Creating client, Thread ID: {Thread.CurrentThread.ManagedThreadId}");
    Client client = new Client();
    client.enabletracing("openiap=info", "");
    // client.enabletracing("openiap=trace", "new");
    await client.connect();
    // client.connect();
    if (!client.connected())
    {
      client.error("Client connection error: " + client.connectionerror());
      return;
    }
    client.info("Client connection success: " + client.connected());

    // Add handler state variables
    Timer? f64_handler = null;
    Timer? u64_handler = null;
    Timer? i64_handler = null;

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
          Console.WriteLine("o - Toggle f64 observable gauge");
          Console.WriteLine("o2 - Toggle u64 observable gauge");
          Console.WriteLine("o3 - Toggle i64 observable gauge");
          Console.WriteLine("cc - Custom command example");
          Console.WriteLine("g - Get client state");
          Console.WriteLine("l - List collections");
          Console.WriteLine("c - Count entities");
          Console.WriteLine("dd - Distinct entities");
          Console.WriteLine("cc2 - Create a timeseries collection");
          Console.WriteLine("cc3 - Create a collection, index, and insert an entity");
          Console.WriteLine("dc - Drop collections");
          Console.WriteLine("gi - Get indexes");
          Console.WriteLine("im - Insert many entities");
          Console.WriteLine("u1 - Update an entity");
          Console.WriteLine("d - Download a file");
          Console.WriteLine("u - Upload a file");
          Console.WriteLine("m - Send a message to a queue");
          Console.WriteLine("r - Register a queue");
          Console.WriteLine("r2 - Send a message to a queue and wait for a response");
          Console.WriteLine("p - Pop a workitem from the queue");
          Console.WriteLine("pp - Push a workitem to the queue");
          Console.WriteLine("st - Start a task to pop workitems");
          Console.WriteLine("st2 - Start a task to run a test");
          Console.WriteLine("gc - Force garbage collection");
          Console.WriteLine("dis - Disconnect from the server");
          Console.WriteLine("0-4 - Disable or Enable tracing level");
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
        case "info":
          client.info("Info message from dotnet client.");
          break;
        case "warn":
          client.warn("Warning message from dotnet client.");
          break;          
        case "error":
          client.error("Error message from dotnet client.");
          break;
        case "debug":
          client.debug("Debug message from dotnet client.");
          break;
        case "trace":
          client.trace("Trace message from dotnet client.");
          break;
        case "pp":
          var pushwi = new Workitem { name = "test from dotnet", payload = "{\"_type\": \"test\"}" };
          // var pushwires = await client.PushWorkitem("q2", pushwi,new string[] {"2023_State of the Union address_multilingual.pdf"});
          var pushwires = await client.PushWorkitem("q2", pushwi, new string[] { "assistant-linux-x86_64.AppImage" });
          // var pushwires = await client.PushWorkitem("q2", pushwi,new string[] {"../testfile.csv"});
          Console.WriteLine("Pushed workitem: {0} {1}", pushwires.id, pushwires.name);
          break;
        case "p":
          if (System.IO.Directory.Exists("downloads"))
          {
            System.IO.Directory.Delete("downloads", true);
          }
          System.IO.Directory.CreateDirectory("downloads");
          var popwi = await client.PopWorkitem("q2", downloadfolder: "downloads");
          if (popwi != null)
          {
            Console.WriteLine("Popped workitem: ", popwi.id, popwi.name);
            for (var i = 0; i < popwi.files.Length; i++)
            {
              Console.WriteLine("File: ", popwi.files[i]);
              System.IO.File.Copy("downloads/" + popwi.files[i].filename, "downloads/" + popwi.files[i].filename + ".copy");
            }
          }
          else
          {
            Console.WriteLine("No workitem to pop.");
          }
          break;
        case "dis":
          try
          {
            var t = System.Threading.Tasks.Task.Run(() =>
            {
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
          var res = Task.Run(async () =>
          {
            await TestClass.Test(client);
          });
          break;
        case "st":
          if (!token.IsCancellationRequested)
          {
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
        case "st2":
          if (!token.IsCancellationRequested)
          {
            Console.WriteLine("Stopping running task.");
            cancellationTokenSource.Cancel();
            break;
          }
          cancellationTokenSource = new CancellationTokenSource();
          token = cancellationTokenSource.Token;
          int x2 = 0;
          var task2 = Task.Run(async () =>
          {
            Console.WriteLine("Task started, begin loop...");
            while (!token.IsCancellationRequested)
            {
              try
              {
                x2++;
                Thread.Sleep(1);
                await TestClass.Test(client);
                if (x2 % 500 == 0)
                {
                  Console.WriteLine("No new workitem", DateTime.Now);
                  GC.Collect();
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
        case "s1":
          try
          {
            var (jwt, error, success) = await client.Signin();
            Console.WriteLine("Signin JWT: " + jwt);
          }
          catch (Exception e)
          {
            Console.WriteLine("Error signing in: " + e.Message);
          }
          break;
        case "s2":
          try
          {
            var (jwt, error, success) = await client.Signin("testuser", "testuser");
            Console.WriteLine("Signin JWT: " + jwt);
          }
          catch (Exception e)
          {
            Console.WriteLine("Error signing in: " + e.Message);
          }
          break;

        case "q2":
          try
          {
            var t = Task.Run(async () =>
            {
              Console.WriteLine($"Creating client, Thread ID: {Thread.CurrentThread.ManagedThreadId}");
              var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
              Console.WriteLine("Query returned " + results.Count + " results.");
              for (int i = 0; i < results.Count; i++)
              {
                Console.WriteLine(results[i]._id, " ", results[i].name);
                if (i > 10)
                {
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
          try
          {
            Console.WriteLine($"Creating client, Thread ID: {Thread.CurrentThread.ManagedThreadId}");
            var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
            Console.WriteLine("Query returned " + results.Count + " results.");
            for (int i = 0; i < results.Count; i++)
            {
              Console.WriteLine(results[i]._id, " ", results[i].name);
              if (i > 10)
              {
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

          try
          {
            var results = Task.Run(async () =>
            {
              Console.WriteLine($"Creating client, Thread ID: {Thread.CurrentThread.ManagedThreadId}");
              var results = await client.Query<List<Base>>("entities", "{}", "{\"name\": 1}");
              return results;
            }).Result;
            Console.WriteLine("Query returned " + results.Count + " results.");
            for (int i = 0; i < results.Count; i++)
            {
              Console.WriteLine(results[i]._id, " ", results[i].name);
              if (i > 10)
              {
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
          try
          {
            var results = await client.Aggregate<List<Base>>("entities", "[{\"$match\": {\"_type\": \"test\"}}]");
            Console.WriteLine("Aggregation returned " + results.Count + " results.");
            for (int i = 0; i < results.Count; i++)
            {
              Console.WriteLine(results[i]._id, " ", results[i].name);
              if (i > 10)
              {
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
          try
          {
            // string results = await client.ListCollections<string>();
            // Console.WriteLine("Query results: " + results);
            var results = await client.ListCollections<List<Collection>>();
            Console.WriteLine("ListCollections returned " + results.Count + " results.");
            for (int i = 0; i < results.Count; i++)
            {
              Console.WriteLine(results[i].name);
              if (i > 10)
              {
                break;
              }
            }
          }
          catch (System.Exception e)
          {
            Console.WriteLine("Error querying: " + e.Message);
          }
          break;
        case "c":
          try
          {
            var count_res = await client.Count("entities", "{}");
            Console.WriteLine("Count result: " + count_res);
          }
          catch (System.Exception e)
          {
            Console.WriteLine("Error creating collection: " + e.Message);
          }
          break;
        case "dd":
          try
          {
            var distinct_res = await client.Distinct("entities", "name", "{}");
            for (int i = 0; i < distinct_res.Count(); i++)
            {
              Console.WriteLine(distinct_res[i]);
              if (i > 10)
              {
                break;
              }
            }
          }
          catch (System.Exception e)
          {
            Console.WriteLine("Error creating collection: " + e.Message);
          }
          break;
        case "cc":
          try
          {
            // Example 1: Using custom_command with generic string return type
            Console.WriteLine("Example 1: Custom command with string return");
            var stringResult = await client.custom_command<string>("getclients", "{}");
            Console.WriteLine($"Custom command result (string): {stringResult}");

            // Example 2: Using custom_command with dynamic return type for JSON objects
            Console.WriteLine("\nExample 2: Custom command with Base object return");
            var data = JsonSerializer.Serialize(new { name = "test" });
            var typedResult = await client.custom_command<Base>("getclients", data);
            if (typedResult != null)
            {
              Console.WriteLine($"Custom command result (typed): ID={typedResult._id}, Name={typedResult.name}");
            }

            // Example 3: Using custom_command with JWT token
            Console.WriteLine("\nExample 3: Custom command with JWT token");
            var (jwt, _, _) = await client.Signin();
            var secureResult = await client.custom_command<string>("getclients", "{\"action\":\"check\"}", jwt);
            Console.WriteLine($"Custom command result with JWT: {secureResult}");
          }
          catch (System.Exception e)
          {
            Console.WriteLine("Error executing custom command: " + e.Message);
          }
          break;
        case "cc2":
          try
          {
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
          try
          {
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
          try
          {
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
          try
          {
            var result = await client.GetIndexes<string>("entities");
            Console.WriteLine("GetIndexes result: " + result);
          }
          catch (System.Exception e)
          {
            Console.WriteLine("Error inserting: " + e.Message);
          }
          break;

        case "i":
          try
          {
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
          try
          {
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
          try
          {
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
          try
          {
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
          try
          {
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
            Console.WriteLine("Error uploading file: " + e.Message);
          }
          break;
        case "w":
          try
          {
            watchId = await client.watch("entities", "[]", e =>
            {
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
          try
          {
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
          try
          {
            var queueId = client.RegisterQueue("test2queue", e =>
            {
              Console.WriteLine("Queue event received from " + e.queuename + " with data: " + e.data);
              //return "{\"payload\": \"Bettina\"}";
              return Task.FromResult("{\"payload\": \"Bettina\"}");
            });
            Console.WriteLine("Queue registered with id: " + queueId);
          }
          catch (Exception e)
          {
            Console.WriteLine("Error registering queue: " + e.Message);
          }
          break;
        case "r2":
          try
          {
            var message = "{\"message\": \"Hello from dotnet\"}";
            var response = await client.Rpc(message, "test2queue", striptoken: true);
            Console.WriteLine("RPC response: " + response);
          }
          catch (Exception e)
          {
            Console.WriteLine("Error doing rcp: " + e.Message);
          }
          break;
        case "m":
          try
          {
            var message = "{\"message\": \"Hello from dotnet\"}";
            await client.QueueMessage(message, "test2queue", striptoken: true);
          }
          catch (Exception e)
          {
            Console.WriteLine("Error sending message: " + e.Message);
          }
          break;
        case "o":
          if (f64_handler != null)
          {
            client.disable_observable_gauge("test_f64");
            client.info("stopped test_f64");
            f64_handler.Dispose();
            f64_handler = null;
          }
          else
          {
            client.set_f64_observable_gauge("test_f64", 42.7, "test");
            client.info("started test_f64 to 42.7");
            var random = new Random();
            f64_handler = new Timer(state =>
            {
              var value = random.NextDouble() * 50;
              client.info($"Setting test_f64 to {value}");
              client.set_f64_observable_gauge("test_f64", value, "test");
            }, null, TimeSpan.Zero, TimeSpan.FromSeconds(30));
          }
          break;

        case "o2":
          if (u64_handler != null)
          {
            client.disable_observable_gauge("test_u64");
            client.info("stopped test_u64");
            u64_handler.Dispose();
            u64_handler = null;
          }
          else
          {
            client.set_u64_observable_gauge("test_u64", 42, "test");
            client.info("started test_u64 to 42");
            var random = new Random();
            u64_handler = new Timer(state =>
            {
              var value = (ulong)random.Next(0, 50);
              client.info($"Setting test_u64 to {value}");
              client.set_u64_observable_gauge("test_u64", value, "test");
            }, null, TimeSpan.Zero, TimeSpan.FromSeconds(30));
          }
          break;

        case "o3":
          if (i64_handler != null)
          {
            client.disable_observable_gauge("test_i64");
            client.info("stopped test_i64");
            i64_handler.Dispose();
            i64_handler = null;
          }
          else
          {
            client.set_i64_observable_gauge("test_i64", 42, "test");
            client.info("started test_i64 to 42");
            var random = new Random();
            i64_handler = new Timer(state =>
            {
              var value = (long)random.Next(0, 50);
              client.info($"Setting test_i64 to {value}");
              client.set_i64_observable_gauge("test_i64", value, "test");
            }, null, TimeSpan.Zero, TimeSpan.FromSeconds(30));
          }
          break;
        case "rpa":
          try
          {
            var rparesult = client.InvokeOpenRPA("5ce94386320b9ce0bc2c3d07",
                "61a1b281-648e-4c40-bb40-2c9db71efef6", "{\"test\":\"test\", \"num\":7}", true, 10);
            client.info("rpa result " + rparesult);
          }
          catch (System.Exception ex)
          {
            client.error("Client connection error: " + ex.Message);
          }
          break;
        case "g":
          try
          {
            var state = client.get_state();
            Console.WriteLine("Client state: " + state);
            var timeout = client.get_default_timeout();
            Console.WriteLine("Client default timeout " + timeout + " seconds.");
            client.set_default_timeout(2);
            Console.WriteLine("Updated client default timeout to 2 seconds.");
            timeout = client.get_default_timeout();
            if (timeout == 2)
            {
              Console.WriteLine("Client default timeout is now " + timeout + " seconds.");
            }
            else
            {
              Console.WriteLine("Client default timeout is not 2 seconds, it is " + timeout + " seconds.");
            }
          }
          catch (System.Exception ex)
          {
            client.error("Client connection error: " + ex.Message);
          }
          break;
        case "quit":
          // Cleanup handlers
          f64_handler?.Dispose();
          u64_handler?.Dispose();
          i64_handler?.Dispose();
          try
          {
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
