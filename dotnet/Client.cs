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
        libDir = AppDomain.CurrentDomain.BaseDirectory + "/../../../lib";
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

    public IntPtr clientPtr;
    public ClientWrapper client;

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

    public string Signin(string username = "", string password = "")
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
            free_signin_response(userPtr);

            return jwt;
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

    public void Dispose()
    {
        free_client(clientPtr);
    }
}
