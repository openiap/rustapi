using System;
using System.Data;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

public class WatchEvent
{
    public string? id { get; set; }
    public string? operation { get; set; }
    public object? document { get; set; }

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
    public delegate void QueryCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct AggregateRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr aggregates;
        public IntPtr queryas;
        public IntPtr hint;
        [MarshalAs(UnmanagedType.I1)]
        public bool explain;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct AggregateResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
    }
    public delegate void AggregateCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct CountRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr query;
        public IntPtr queryas;
        [MarshalAs(UnmanagedType.I1)]
        public bool explain;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct CountResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public int count;        
        public IntPtr error;
    }
    public delegate void CountCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DistinctRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr field;
        public IntPtr query;
        public IntPtr queryas;
        [MarshalAs(UnmanagedType.I1)]
        public bool explain;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct DistinctResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public UIntPtr results_count;
        public IntPtr error;
    }
    public delegate void DistinctCallback(IntPtr responsePtr);


    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOneRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr item;
        public int w;
        public bool j;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOneResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr result;
        public IntPtr error;
    }
    public delegate void InsertOneCallback(IntPtr responsePtr);

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
    public delegate void DownloadCallback(IntPtr responsePtr);

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
    public delegate void UploadCallback(IntPtr responsePtr);
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
    public static extern void client_signin(IntPtr client, ref SigninRequestWrapper request, SigninCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_signin_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_query(IntPtr client, ref QueryRequestWrapper request, QueryCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_query_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_aggregate(IntPtr client, ref AggregateRequestWrapper request, AggregateCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_aggregate_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_count(IntPtr client, ref CountRequestWrapper request, CountCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_count_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_distinct(IntPtr client, ref DistinctRequestWrapper request, DistinctCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_distinct_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_insert_one(IntPtr client, ref InsertOneRequestWrapper request, InsertOneCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_insert_one_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_download(IntPtr client, ref DownloadRequestWrapper request, DownloadCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_download_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_upload(IntPtr client, ref UploadRequestWrapper request, UploadCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_upload_response(IntPtr response);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void WatchCallback(IntPtr eventStr);
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void WatchEventCallback(IntPtr eventStr);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr client_watch(IntPtr client, ref WatchRequestWrapper request, WatchCallback callback, WatchEventCallback event_callback);

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
                        tcs.SetException(new ClientError("Callback got null response"));
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

    public Task<string> Query(string collectionname, string query, string projection = "", string orderby = "", string queryas = "", bool explain = false, int skip = 0, int top = 100)
    {
        var tcs = new TaskCompletionSource<string>();

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

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<QueryResponseWrapper>(responsePtr);
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_query_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(results);
                    }

                    // tcs.SetResult((results, error, success));
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new QueryCallback(Callback);

            client_query(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(projectionPtr);
            Marshal.FreeHGlobal(orderbyPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }
        return tcs.Task;
    }

    public Task<string> Aggregate(string collectionname, string aggregates, string queryas = "", string hint = "", bool explain = false)
    {
        var tcs = new TaskCompletionSource<string>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr aggregatesPtr = Marshal.StringToHGlobalAnsi(aggregates);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);
        IntPtr hintPtr = Marshal.StringToHGlobalAnsi(hint);

        try
        {
            AggregateRequestWrapper request = new AggregateRequestWrapper
            {
                collectionname = collectionnamePtr,
                aggregates = aggregatesPtr,
                queryas = queryasPtr,
                hint = hintPtr,
                explain = explain
            };

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<AggregateResponseWrapper>(responsePtr);
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_aggregate_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(results);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new AggregateCallback(Callback);

            client_aggregate(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(aggregatesPtr);
            Marshal.FreeHGlobal(queryasPtr);
            Marshal.FreeHGlobal(hintPtr);
        }
        return tcs.Task;
    }

    public Task<int> Count(string collectionname, string query = "", string queryas = "", bool explain = false)
    {
        var tcs = new TaskCompletionSource<int>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);

        try
        {
            CountRequestWrapper request = new CountRequestWrapper
            {
                collectionname = collectionnamePtr,
                query = queryPtr,
                queryas = queryasPtr,
                explain = explain
            };

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<CountResponseWrapper>(responsePtr);
                    // result: i32, in rust
                    int count = (int)response.count;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_count_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(count);
                    }                    
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new CountCallback(Callback);

            client_count(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }
        return tcs.Task;
    }

    public Task<string[]> Distinct(string collectionname, string field, string query = "", string queryas = "", bool explain = false)
    {
        var tcs = new TaskCompletionSource<string[]>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr fieldPtr = Marshal.StringToHGlobalAnsi(field);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);

        try
        {
            DistinctRequestWrapper request = new DistinctRequestWrapper
            {
                collectionname = collectionnamePtr,
                field = fieldPtr,
                query = queryPtr,
                queryas = queryasPtr,
                explain = explain
            };

            void Callback(IntPtr responsePtr)
            {
                try
                {

                                       if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<DistinctResponseWrapper>(responsePtr);
                    bool success = response.success;

                    if (!success)
                    {
                        string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        int resultsCount = (int)response.results_count;
                        string[] results = new string[resultsCount];
                        IntPtr[] resultPtrs = new IntPtr[resultsCount];
                        Marshal.Copy(response.results, resultPtrs, 0, resultsCount);

                        for (int i = 0; i < resultsCount; i++)
                        {
                            results[i] = Marshal.PtrToStringAnsi(resultPtrs[i]) ?? string.Empty;
                        }

                        tcs.SetResult(results);
                    }

                    free_distinct_response(responsePtr);

                    
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new DistinctCallback(Callback);

            client_distinct(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(fieldPtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }
        return tcs.Task;
    }

    public Task<string> InsertOne(string collectionname, string item, int w = 1, bool j = false)
    {
        var tcs = new TaskCompletionSource<string>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr itemPtr = Marshal.StringToHGlobalAnsi(item);

        try
        {
            InsertOneRequestWrapper request = new InsertOneRequestWrapper
            {
                collectionname = collectionnamePtr,
                item = itemPtr,
                w = w,
                j = j
            };

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<InsertOneResponseWrapper>(responsePtr);
                    string result = Marshal.PtrToStringAnsi(response.result) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_insert_one_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(result);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new InsertOneCallback(Callback);

            client_insert_one(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(itemPtr);
        }
        return tcs.Task;
    }

    public Task<string> download(string collectionname, string id, string folder, string filename)
    {
        var tcs = new TaskCompletionSource<string>();

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
            void Callback(IntPtr responsePtr)
            {
                if (responsePtr == IntPtr.Zero)
                {
                    tcs.SetException(new ClientError("Callback got null response"));
                    return;
                }

                var response = Marshal.PtrToStructure<DownloadResponseWrapper>(responsePtr);
                string filename = Marshal.PtrToStringAnsi(response.filename) ?? string.Empty;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;
                free_query_response(responsePtr);

                if (!success)
                {
                    tcs.SetException(new ClientError(error));
                }
                else
                {
                    tcs.SetResult(filename);
                }
            }
            var callbackDelegate = new DownloadCallback(Callback);

            client_download(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(idPtr);
            Marshal.FreeHGlobal(folderPtr);
            Marshal.FreeHGlobal(filenamePtr);
        }
        return tcs.Task;
    }

    public Task<string> upload(string filepath, string filename, string mimetype, string metadata, string collectionname)
    {
        var tcs = new TaskCompletionSource<string>();

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

            void Callback(IntPtr responsePtr)
            {
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<UploadResponseWrapper>(responsePtr);
                    string id = Marshal.PtrToStringAnsi(response.id) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_upload_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(id);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var callbackDelegate = new UploadCallback(Callback);

            client_upload(clientPtr, ref request, callbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(filepathPtr);
            Marshal.FreeHGlobal(filenamePtr);
            Marshal.FreeHGlobal(mimetypePtr);
            Marshal.FreeHGlobal(metadataPtr);
            Marshal.FreeHGlobal(collectionnamePtr);
        }
        return tcs.Task;
    }
    public Task<string> watch(string collectionname, string paths, Action<WatchEvent> eventHandler)
    {
        var tcs = new TaskCompletionSource<string>();
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr pathsPtr = Marshal.StringToHGlobalAnsi(paths);

        try
        {
            WatchRequestWrapper request = new WatchRequestWrapper
            {
                collectionname = collectionnamePtr,
                paths = pathsPtr
            };

            var callback = new WatchEventCallback((IntPtr eventStr) =>
            {
                Console.WriteLine("dotnet: watch event callback");
                string eventJson = Marshal.PtrToStringAnsi(eventStr) ?? string.Empty;
                if (eventJson != null && eventJson != "")
                {
                    var eventObj = System.Text.Json.JsonSerializer.Deserialize<WatchEvent>(eventJson);
                    if (eventObj != null)
                    {
                        eventHandler?.Invoke(eventObj);
                    }
                }
            });

            void Callback(IntPtr responsePtr)
            {
                Console.WriteLine("dotnet: register watch callback");
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<WatchResponseWrapper>(responsePtr);
                    string watchid = Marshal.PtrToStringAnsi(response.watchid) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_watch_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(watchid);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }
            var callbackDelegate = new WatchCallback(Callback);

            client_watch(clientPtr, ref request, callbackDelegate, callback);

        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(pathsPtr);
        }
        return tcs.Task;
    }


    public void Dispose()
    {
        Console.WriteLine("Dotnet: Dispose client");
        free_client(clientPtr);
    }
}
