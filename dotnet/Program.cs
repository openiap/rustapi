using System;
using System.Threading;
using System.Runtime.InteropServices;

class Program
{
     static void Main(string[] args)
    {
        MainAsync(args).GetAwaiter().GetResult();
    }
    static async Task MainAsync(string[] args)
    {
        try
        {
            Client client = new Client();
            await client.connect();
            if(client.connected() == false) {
                Console.WriteLine("Client connection error: " + client.connectionerror());
                return;
            }
            Console.WriteLine("Client connection success: " + client.connected());

            // var (jwt, error, success) = await client.Signin();
            // Console.WriteLine("Signin JWT: " + jwt);

            // string results = await client.Query("entities", "{}", "{\"name\": 1}");
            // Console.WriteLine("results: " + results);

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

            await client.download("fs.files", "65a3aaf66d52b8c15131aebd", folder: "", filename: "");

            var filepath = "testfile.csv";
            if(!System.IO.File.Exists(filepath))
            {
                filepath = "../testfile.csv";
            }
            var upload_response = await client.upload(filepath, "dotnet-test.csv", "", "", "fs.files");
            Console.WriteLine("Dotnet: upload success as " +  upload_response);

            var eventcount = 0;
            var watch_response =  await client.watch("entities", "", (eventObj) => {
                Console.WriteLine("watch event " + eventObj.operation + " on " + eventObj.document);
                eventcount++;
            });
            Console.WriteLine("Dotnet: watch registered success as " +  watch_response);

            while (eventcount < 2)
            {
                await Task.Delay(1000);
            }

            client.Dispose();
        }
        catch (Client.ClientError e)
        {
            Console.WriteLine($"An error occurred: {e.Message}");
        }
    }
}
