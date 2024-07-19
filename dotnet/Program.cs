using System;
using System.Threading;
using System.Runtime.InteropServices;

class Program
{
    static void Main(string[] args)
    {
        try
        {
            Client client = new Client("http://localhost:50051");

            Console.WriteLine("Client connection success: " + client.client.success);
            Console.WriteLine("Client connection error: " + Marshal.PtrToStringAnsi(client.client.error));

            string jwt = client.Signin();
            Console.WriteLine("Signin JWT: " + jwt);

            client.Dispose();
        }
        catch (Client.ClientError e)
        {
            Console.WriteLine($"An error occurred: {e.Message}");
        }
    }
}
