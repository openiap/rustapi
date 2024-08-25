using System;
using System.Threading;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Dynamic;

class Program
{
     static void Main(string[] args)
    {
          // Subscribe to the UnhandledException event
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

        // Subscribe to the UnobservedTaskException event
        TaskScheduler.UnobservedTaskException += (sender, eventArgs) =>
        {
            Console.WriteLine("Unobserved Task Exception occurred:");
            Console.WriteLine(eventArgs.Exception.ToString());
            // Optionally, mark the exception as observed
            eventArgs.SetObserved();
        };
        
        MainAsync(args).GetAwaiter().GetResult();
    }
    static async Task MainAsync(string[] args)
    {
        try
        {
            Client client = new Client();
            // client.enabletracing("openiap=trace", "new");
            // client.enabletracing("openiap=debug", "new");
            client.enabletracing("info", "");
            await client.connect();
            if(!client.connected() ) {
                Console.WriteLine("Client connection error: " + client.connectionerror());
                return;
            }
            Console.WriteLine("Client connection success: " + client.connected());


            var (jwt, error, success) = await client.Signin();
            Console.WriteLine("Signin JWT: " + jwt);


            var files = new string[] { "testfile.csv" };
            if(!System.IO.File.Exists("testfile.csv")) {
                files = new string[] { "../testfile.csv" };
            }
            // files = new string[] {};

            var workitem = new Workitem { name = "test from dotnet 1", payload = "{\"name\": \"test from dotnet 1\"}" };
            var push_workitem_result = await client.PushWorkitem("rustqueue", workitem, files);
            Console.WriteLine("PushWorkitem: ", push_workitem_result);

            workitem = await client.PopWorkitem("rustqueue");
            Console.WriteLine("PopWorkitem: ", workitem);
            workitem.state = "successful";
            workitem = await client.UpdateWorkitem(workitem, new string[] { });
            Console.WriteLine("UpdateWorkitem: ", workitem);

            await client.DeleteWorkitem(workitem.id);


            string results = await client.Query("entities", "{}", "{\"name\": 1}");
            Console.WriteLine("results: " + results);

            for(var y = 0; y < 5; y++) {
                var promises = new List<Task<string>>();
                for(var x = 0; x < 15; x++) {
                    promises.Add(client.Query("entities", "{}", "{\"name\": 1}"));
                }
                var result = await Task.WhenAll(promises);
                Console.WriteLine("results: " + result.Length);
            }

            // // System.Threading.Thread.Sleep(120000);

            var aggregate_results = await client.Aggregate("entities", "[]");
            Console.WriteLine("aggregate results: " + aggregate_results);

            var insert_one_result = await client.InsertOne("entities", "{\"name\": \"test from dotnet\", \"_type\": \"test\"}");
            Console.WriteLine("insert one result: " + insert_one_result);

            dynamic? item = JsonSerializer.Deserialize<ExpandoObject>(insert_one_result, new JsonSerializerOptions { IncludeFields = true });
            if(item == null) throw new Exception("Failed to deserialize insert_one_result");
            item.name = "test from dotnet updated";
            System.Text.Json.JsonElement itemid = item._id;
            var _id = itemid.GetString();
            if(string.IsNullOrEmpty(_id)) throw new Exception("Failed to get _id from insert_one_result");
            
            insert_one_result = JsonSerializer.Serialize(item);

            var update_one_result = await client.UpdateOne("entities", insert_one_result);
            Console.WriteLine("update one result: " + update_one_result);

            var delete_one_result = await client.DeleteOne("entities", _id);
            Console.WriteLine("delete one result: " + delete_one_result);

            var insert_or_update_one_result2 = await client.InsertOne("entities", "{\"name\": \"test insert or update from dotnet\", \"_type\": \"test\"}");
            Console.WriteLine("insert one result: " + insert_or_update_one_result2);
            dynamic? item2 = JsonSerializer.Deserialize<ExpandoObject>(insert_or_update_one_result2, new JsonSerializerOptions { IncludeFields = true });
            if(item2 == null) throw new Exception("Failed to deserialize insert_one_result");
            item2.name = "test insert or update from dotnet updated";
            insert_or_update_one_result2 = System.Text.Json.JsonSerializer.Serialize(item2);
            insert_or_update_one_result2 = await client.InsertOrUpdateOne("entities", insert_or_update_one_result2);

            System.Text.Json.JsonElement itemid2 = item2._id;
            var _id2 = itemid2.GetString();
            if(string.IsNullOrEmpty(_id2)) throw new Exception("Failed to get _id from insert_one_result");

            var delete_many_by_ids_result = await client.DeleteMany("entities", ids: new string[] { _id2 });
            Console.WriteLine("delete many by ids result: " + delete_many_by_ids_result);

            await client.InsertOne("entities", "{\"name\": \"test delete many from dotnet\", \"_type\": \"test\"}");
            var delete_many_by_query_result = await client.DeleteMany("entities", query: "{\"name\": \"test delete many from dotnet\"}");
            Console.WriteLine("delete many by query result: " + delete_many_by_query_result);

            await client.download("fs.files", "65a3aaf66d52b8c15131aebd", folder: "", filename: "");

            var filepath = "testfile.csv";
            if(!System.IO.File.Exists(filepath))
            {
                filepath = "../testfile.csv";
            }
            var upload_response = await client.upload(filepath, "dotnet-test.csv", "", "", "fs.files");
            Console.WriteLine("Dotnet: upload success as " +  upload_response);

            var eventcount = 0;
            var watch_response = await client.watch("entities", "", (eventObj) => {
                Console.WriteLine("watch event " + eventObj.operation + " on " + eventObj.document);
                eventcount++;
            });
            Console.WriteLine("Dotnet: watch registered success as " +  watch_response);

            var insert_many_result = await client.InsertMany("entities", "[{\"name\": \"test from dotnet 1 \", \"_type\": \"test\"}, {\"name\": \"test from dotnet 2\", \"_type\": \"test\"}]");

            while (eventcount < 2)
            {
                await Task.Delay(1000);
            }
            client.UnWatch(watch_response);

            var queuecount = 0;
            var register_queue_response = await client.RegisterQueue("test2queue", (eventObj) => {
                Console.WriteLine("watch event " + eventObj.queuename + " on " + eventObj.data);
                queuecount++;
            });
            Console.WriteLine("Dotnet: registered queue success as " + register_queue_response);

            await client.QueueMessage("{\"name\": \"test message 1 \"}", "test2queue");
            await client.QueueMessage("{\"name\": \"test message 2 \"}", "test2queue");

            while (queuecount < 2)
            {
                await Task.Delay(1000);
            }
            client.UnRegisterQueue(register_queue_response);

            var exchangecount = 0;
            var register_exchange_response = await client.RegisterExchange("testexc", eventHandler: (eventObj) => {
                Console.WriteLine("watch event " + eventObj.queuename + " on " + eventObj.data);
                exchangecount++;
            });
            Console.WriteLine("Dotnet: registered exchange success, using queue " + register_exchange_response);

            await client.QueueMessage("{\"name\": \"test message 1 \"}", exchangename: "testexc");
            await client.QueueMessage("{\"name\": \"test message 2 \"}", exchangename: "testexc");

            while (exchangecount < 2)
            {
                await Task.Delay(1000);
            }
            client.UnRegisterQueue(register_exchange_response);


            var count_response = await client.Count("entities", "");
            Console.WriteLine("Dotnet: count success as " +  count_response);

            var distinct_response = await client.Distinct("entities", "_type");
            Console.WriteLine("Dotnet: distinct success as " + string.Join(",", distinct_response));

            client.Dispose();
        }
        catch (Client.ClientError e)
        {
            Console.WriteLine($"An error occurred: {e.Message}");
        }
    }
}
