﻿using System;
using System.Threading;
using System.Runtime.InteropServices;

class Program
{
    static void Main(string[] args)
    {
        try
        {
            // Client client = new Client("grpc://grpc.demo.openiap.io:443");
            Client client = new Client("");
            Console.WriteLine("Client connection success: " + client.connected());
            Console.WriteLine("Client connection error: " + client.connectionerror());

            var (jwt, error, success) = client.Signin();
            Console.WriteLine("Signin JWT: " + jwt);

            string results = client.Query("entities", "{}", "{\"name\": 1}", "", "", false, 0, 0);
            Console.WriteLine("results: " + results);


            // System.Threading.Thread.Sleep(120000);

            client.Dispose();
        }
        catch (Client.ClientError e)
        {
            Console.WriteLine($"An error occurred: {e.Message}");
        }
    }
}
