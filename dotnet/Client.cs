using System;
using System.Data;
using System.Runtime.InteropServices;

public class WatchEvent {
    public required string id { get; set; }
    public required string operation { get; set; }
    public required object document { get; set; }

}
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
    public delegate void SigninCallback(IntPtr responsePtr);



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
    [StructLayout(LayoutKind.Sequential)]
    public struct DownloadRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr id;
        public IntPtr folder;
        public IntPtr filename;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct DownloadResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr filename;
        public IntPtr error;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UploadRequestWrapper
    {
        public IntPtr filepath;
        public IntPtr filename;
        public IntPtr mimetype;
        public IntPtr metadata;
        public IntPtr collectionname;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UploadResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr id;
        public IntPtr error;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct WatchRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr paths;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct WatchResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr watchid;
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
        if (System.IO.File.Exists(libPath))
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

    public delegate void ConnectCallback(IntPtr clientWrapperPtr);
    // Import the Rust library functions
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_connect(string url, ConnectCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_client(IntPtr client);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_signin(IntPtr client, ref SigninRequestWrapper request, SigninCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_signin_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_query(IntPtr client, ref QueryRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_query_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_download(IntPtr client, ref DownloadRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_download_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_upload(IntPtr client, ref UploadRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_upload_response(IntPtr response);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void WatchCallback(IntPtr eventStr);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_watch(IntPtr client, ref WatchRequestWrapper request, WatchCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_watch_response(IntPtr response);

    public IntPtr clientPtr;
    ClientWrapper client;


    public Client()
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
    }
    public async Task connect(string url = "")
    {
        var tcs = new TaskCompletionSource<ClientWrapper>();

        void Callback(IntPtr clientWrapperPtr)
        {
            try
            {
                var clientWrapper = Marshal.PtrToStructure<ClientWrapper>(clientWrapperPtr);
                if (!clientWrapper.success)
                {
                    var errorMsg = Marshal.PtrToStringAnsi(clientWrapper.error) ?? "Unknown error";
                    tcs.SetException(new ClientCreationError(errorMsg));
                }
                else
                {
                    clientPtr = clientWrapperPtr;
                    client = clientWrapper;
                    tcs.SetResult(clientWrapper);
                }
            }
            catch (Exception ex)
            {
                tcs.SetException(ex);
            }
        }

        var callbackDelegate = new ConnectCallback(Callback);

        client_connect(url, callbackDelegate);

        client = await tcs.Task;
    }
    public bool connected()
    {
        return client.success;
    }
    public string connectionerror()
    {
        var res = Marshal.PtrToStringAnsi(client.error);
        if (res == null)
        {
            return "";
        }
        return res;
    }

    public Task<(string jwt, string error, bool success)> Signin(string username = "", string password = "")
    {
        var tcs = new TaskCompletionSource<(string jwt, string error, bool success)>();

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

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Signin failed or user is null"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<SigninResponseWrapper>(responsePtr);
                    string jwt = Marshal.PtrToStringAnsi(response.jwt) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_signin_response(responsePtr);

                    tcs.SetResult((jwt, error, success));
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new SigninCallback(Callback);

            client_signin(clientPtr, ref request, callbackDelegate);
        }
        catch (Exception ex)
        {
            tcs.SetException(ex);
            Marshal.FreeHGlobal(usernamePtr);
            Marshal.FreeHGlobal(passwordPtr);
            Marshal.FreeHGlobal(jwtPtr);
            Marshal.FreeHGlobal(agentPtr);
            Marshal.FreeHGlobal(versionPtr);
        }

        return tcs.Task;
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

    public string download(string collectionname, string id, string folder, string filename)
    {
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr idPtr = Marshal.StringToHGlobalAnsi(id);
        IntPtr folderPtr = Marshal.StringToHGlobalAnsi(folder);
        IntPtr filenamePtr = Marshal.StringToHGlobalAnsi(filename);

        try
        {
            DownloadRequestWrapper request = new DownloadRequestWrapper
            {
                collectionname = collectionnamePtr,
                id = idPtr,
                folder = folderPtr,
                filename = filenamePtr
            };

            IntPtr responsePtr = client_download(clientPtr, ref request);

            if (responsePtr == IntPtr.Zero)
            {
                throw new ClientError("Download failed or response is null");
            }

            DownloadResponseWrapper response = Marshal.PtrToStructure<DownloadResponseWrapper>(responsePtr);
            string result = Marshal.PtrToStringAnsi(response.filename) ?? string.Empty;
            free_download_response(responsePtr);

            return result;
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(idPtr);
            Marshal.FreeHGlobal(folderPtr);
            Marshal.FreeHGlobal(filenamePtr);
        }
    }

    public string upload(string filepath, string filename, string mimetype, string metadata, string collectionname)
    {
        IntPtr filepathPtr = Marshal.StringToHGlobalAnsi(filepath);
        IntPtr filenamePtr = Marshal.StringToHGlobalAnsi(filename);
        IntPtr mimetypePtr = Marshal.StringToHGlobalAnsi(mimetype);
        IntPtr metadataPtr = Marshal.StringToHGlobalAnsi(metadata);
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);

        try
        {
            UploadRequestWrapper request = new UploadRequestWrapper
            {
                filepath = filepathPtr,
                filename = filenamePtr,
                mimetype = mimetypePtr,
                metadata = metadataPtr,
                collectionname = collectionnamePtr
            };

            IntPtr responsePtr = client_upload(clientPtr, ref request);

            if (responsePtr == IntPtr.Zero)
            {
                throw new ClientError("Upload failed or response is null");
            }

            UploadResponseWrapper response = Marshal.PtrToStructure<UploadResponseWrapper>(responsePtr);
            string result = Marshal.PtrToStringAnsi(response.id) ?? string.Empty;
            free_upload_response(responsePtr);

            return result;
        }
        finally
        {
            Marshal.FreeHGlobal(filepathPtr);
            Marshal.FreeHGlobal(filenamePtr);
            Marshal.FreeHGlobal(mimetypePtr);
            Marshal.FreeHGlobal(metadataPtr);
            Marshal.FreeHGlobal(collectionnamePtr);
        }
    }
    public string watch(string collectionname, string paths, Action<WatchEvent> eventHandler)
    {
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr pathsPtr = Marshal.StringToHGlobalAnsi(paths);

        try
        {
            WatchRequestWrapper request = new WatchRequestWrapper
            {
                collectionname = collectionnamePtr,
                paths = pathsPtr
            };

            WatchCallback callback = new WatchCallback((IntPtr eventStr) =>
           {
            string eventJson = Marshal.PtrToStringAnsi(eventStr) ?? string.Empty;
               if (eventJson != null && eventJson != "")
               {
                   var eventObj = System.Text.Json.JsonSerializer.Deserialize<WatchEvent>(eventJson);
                   if(eventObj != null) {                    
                    eventHandler?.Invoke(eventObj);                    
                   }                   
               }
           });

            IntPtr responsePtr = client_watch(clientPtr, ref request, callback);

            if (responsePtr == IntPtr.Zero)
            {
                throw new ClientError("Watch failed or response is null");
            }

            WatchResponseWrapper response = Marshal.PtrToStructure<WatchResponseWrapper>(responsePtr);
            string result = Marshal.PtrToStringAnsi(response.watchid) ?? string.Empty;
            free_watch_response(responsePtr);

            return result;
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(pathsPtr);
        }
    }


    public void Dispose()
    {
        free_client(clientPtr);
    }
}
