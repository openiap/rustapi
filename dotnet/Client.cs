using System;
using System.Runtime.InteropServices;

public class Client : IDisposable
{
    // Define the ClientWrapper struct
    [StructLayout(LayoutKind.Sequential)]
    public struct ClientWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public IntPtr client;
        public IntPtr runtime;
    }

    // Define the SigninRequestWrapper struct
    [StructLayout(LayoutKind.Sequential)]
    public struct SigninRequestWrapper
    {
        public IntPtr username;
        public IntPtr password;
        public IntPtr jwt;
        public IntPtr agent;
        public IntPtr version;
        [MarshalAs(UnmanagedType.I1)]
        public bool longtoken;
        [MarshalAs(UnmanagedType.I1)]
        public bool validateonly;
        [MarshalAs(UnmanagedType.I1)]
        public bool ping;
    }

    // Define the SigninResponseWrapper struct
    [StructLayout(LayoutKind.Sequential)]
    public struct SigninResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr jwt;
        public IntPtr error;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct QueryRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr query;
        public IntPtr projection;
        public IntPtr orderby;
        public IntPtr queryas;
        [MarshalAs(UnmanagedType.I1)]
        public bool explain;
        [MarshalAs(UnmanagedType.I4)]
        public int skip;
        [MarshalAs(UnmanagedType.I4)]
        public int top;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct QueryResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
    }

    // Custom exception classes
    public class ClientError : Exception
    {
        public ClientError(string message) : base(message) { }
    }

    public class LibraryLoadError : ClientError
    {
        public LibraryLoadError(string message) : base(message) { }
    }

    public class ClientCreationError : ClientError
    {
        public ClientCreationError(string message) : base(message) { }
    }

    // Function to load the correct library file based on the operating system
    private static string GetLibraryPath()
    {
        string libDir = AppDomain.CurrentDomain.BaseDirectory + "/lib";
        string libPath;

        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.dll");
                break;
            case PlatformID.MacOSX:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.dylib");
                break;
            default:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.so");
                break;
        }
        if(System.IO.File.Exists(libPath))
        {
            return libPath;
        }
        // when testing before publishing
        libDir = AppDomain.CurrentDomain.BaseDirectory + "../../../../target/debug/";
        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.dll");
                break;
            case PlatformID.MacOSX:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.dylib");
                break;
            default:
                libPath = System.IO.Path.Combine(libDir, "libopeniap.so");
                break;
        }
        return libPath;
    }

    // Import the Rust library functions
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_connect(string url);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_client(IntPtr client);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_signin(IntPtr client, ref SigninRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_signin_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_query(IntPtr client, ref QueryRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_query_response(IntPtr response);

    public IntPtr clientPtr;
    ClientWrapper client;


    public Client(): this("") { }

    public Client(string url)
    {
        string libPath = GetLibraryPath();
        NativeLibrary.SetDllImportResolver(typeof(Client).Assembly, (name, assembly, path) =>
        {
            if (name == "libopeniap")
            {
                return NativeLibrary.Load(libPath);
            }
            return IntPtr.Zero;
        });

        clientPtr = client_connect(url);
        client = Marshal.PtrToStructure<ClientWrapper>(clientPtr);

        if (!client.success)
        {
            throw new ClientCreationError(Marshal.PtrToStringAnsi(client.error) ?? "Unknown error");
        }
    }
    public bool connected() {
        return client.success;
    }
    public string connectionerror() {
        var res = Marshal.PtrToStringAnsi(client.error);
        if(res == null) {
            return "";
        }
        return res;
    }

    public (string jwt, string error, bool success) Signin(string username = "", string password = "")
    {
        IntPtr usernamePtr = Marshal.StringToHGlobalAnsi(username);
        IntPtr passwordPtr = Marshal.StringToHGlobalAnsi(password);
        IntPtr jwtPtr = Marshal.StringToHGlobalAnsi("");
        IntPtr agentPtr = Marshal.StringToHGlobalAnsi("dotnet");
        IntPtr versionPtr = Marshal.StringToHGlobalAnsi("");

        try
        {
            SigninRequestWrapper request = new SigninRequestWrapper
            {
                username = usernamePtr,
                password = passwordPtr,
                jwt = jwtPtr,
                agent = agentPtr,
                version = versionPtr,
                longtoken = false,
                validateonly = false,
                ping = false
            };

            IntPtr userPtr = client_signin(clientPtr, ref request);

            if (userPtr == IntPtr.Zero)
            {
                throw new ClientError("Signin failed or user is null");
            }

            SigninResponseWrapper user = Marshal.PtrToStructure<SigninResponseWrapper>(userPtr);
            string jwt = Marshal.PtrToStringAnsi(user.jwt) ?? string.Empty;
            string error = Marshal.PtrToStringAnsi(user.error) ?? string.Empty;
            string success = user.success ? "true" : "false";
            free_signin_response(userPtr);

            return (jwt, error, user.success);
        }
        finally
        {
            Marshal.FreeHGlobal(usernamePtr);
            Marshal.FreeHGlobal(passwordPtr);
            Marshal.FreeHGlobal(jwtPtr);
            Marshal.FreeHGlobal(agentPtr);
            Marshal.FreeHGlobal(versionPtr);
        }
    }

    public string Query(string collectionname, string query, string projection, string orderby, string queryas, bool explain, int skip, int top)
    {
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        IntPtr projectionPtr = Marshal.StringToHGlobalAnsi(projection);
        IntPtr orderbyPtr = Marshal.StringToHGlobalAnsi(orderby);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);

        try
        {
            QueryRequestWrapper request = new QueryRequestWrapper
            {
                collectionname = collectionnamePtr,
                query = queryPtr,
                projection = projectionPtr,
                orderby = orderbyPtr,
                queryas = queryasPtr,
                explain = explain,
                skip = skip,
                top = top
            };

            IntPtr responsePtr = client_query(clientPtr, ref request);

            if (responsePtr == IntPtr.Zero)
            {
                throw new ClientError("Query failed or response is null");
            }

            QueryResponseWrapper response = Marshal.PtrToStructure<QueryResponseWrapper>(responsePtr);
            string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
            free_query_response(responsePtr);

            return results;
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(projectionPtr);
            Marshal.FreeHGlobal(orderbyPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }
    }


    public void Dispose()
    {
        free_client(clientPtr);
    }
}
