using System;
using System.Collections.Concurrent;
using System.Collections.ObjectModel;
using System.Data;
using System.Data.Common;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text.Json;
public partial class Client : IDisposable
{
    // private ConcurrentDictionary<int, TaskCompletionSource<Workitem?>> _PopWorkitemCallbackRegistry = new ConcurrentDictionary<int, TaskCompletionSource<Workitem?>>(); 
    private CallbackRegistry CallbackRegistry = new CallbackRegistry();
    private int CallbackRegistryNextRequestId = 0;
    private readonly PopWorkitemCallback _PopWorkitemCallbackDelegate;
    private readonly UpdateWorkitemCallback _UpdateWorkitemCallbackDelegate;
    private readonly DeleteWorkitemCallback _DeleteWorkitemCallbackDelegate;
    private readonly PushWorkitemCallback _PushWorkitemCallbackDelegate;
    private readonly DeleteManyCallback _DeleteManyCallbackDelegate;
    private readonly DeleteOneCallback _DeleteOneCallbackDelegate;
    private readonly InsertOrUpdateOneCallback _InsertOrUpdateOneCallbackDelegate;
    private readonly UploadCallback _UploadCallbackDelegate;
    private readonly DownloadCallback _DownloadCallbackDelegate;
    private readonly UpdateOneCallback _UpdateOneCallbackDelegate;
    private readonly InsertManyCallback _InsertManyCallbackDelegate;
    private readonly InsertOneCallback _InsertOneCallbackDelegate;
    private readonly DistinctCallback _DistinctCallbackDelegate;
    private readonly CountCallback _CountCallbackDelegate;
    private readonly AggregateCallback _AggregateCallbackDelegate;
    private readonly QueryCallback _QueryCallbackDelegate;
    private readonly DropIndexCallback _DropIndexCallbackDelegate;
    private readonly CreateIndexCallback _CreateIndexCallbackDelegate;
    private readonly GetIndexesCallback _GetIndexesCallbackDelegate;
    private readonly SigninCallback _SigninCallbackDelegate;
    private readonly CreateCollectionCallback _CreateCollectionCallbackDelegate;
    private readonly ListCollectionsCallback _ListCollectionsCallbackDelegate;
    private readonly ConnectCallback _ConnectCallbackDelegate;


    public IntPtr clientPtr;
    ClientWrapper client;
    bool tracing { get; set; } = false;
    bool informing { get; set; } = false;
    bool verbosing { get; set; } = false;

    #region Structs
    [StructLayout(LayoutKind.Sequential)]
    public struct ClientWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public IntPtr client;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct ConnectResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ClientEventResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr eventid;
        public IntPtr error;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct OffClientEventResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
    }


    [StructLayout(LayoutKind.Sequential)]
    public struct ClientEventWrapper
    {
        public IntPtr evt;
        public IntPtr reason;
    }

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
        public int request_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct SigninResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr jwt;
        public IntPtr error;
        public int request_id;
    }


    [StructLayout(LayoutKind.Sequential)]
    public struct ListCollectionsResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int request_id;
    }
    public delegate void ListCollectionsCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct ColCollationWrapper : IDisposable
    {
        ColCollationWrapper(string locale, bool case_level, string case_first, int strength, bool numeric_ordering, string alternate, string max_variable, bool backwards)
        {
            this.locale = Marshal.StringToHGlobalAnsi(locale);
            this.case_level = case_level;
            this.case_first = Marshal.StringToHGlobalAnsi(case_first);
            this.strength = strength;
            this.numeric_ordering = numeric_ordering;
            this.alternate = Marshal.StringToHGlobalAnsi(alternate);
            this.max_variable = Marshal.StringToHGlobalAnsi(max_variable);
            this.backwards = backwards;
        }
        public void Dispose()
        {
            Marshal.FreeHGlobal(locale);
            Marshal.FreeHGlobal(case_first);
            Marshal.FreeHGlobal(alternate);
            Marshal.FreeHGlobal(max_variable);
        }
        public IntPtr locale;
        [MarshalAs(UnmanagedType.I1)]
        public bool case_level;
        public IntPtr case_first;
        public int strength;
        [MarshalAs(UnmanagedType.I1)]
        public bool numeric_ordering;
        public IntPtr alternate;
        public IntPtr max_variable;
        [MarshalAs(UnmanagedType.I1)]
        public bool backwards;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct ColTimeseriesWrapper : IDisposable
    {
        public ColTimeseriesWrapper(string time_field, string meta_field, string granularity)
        {
            this.time_field = Marshal.StringToHGlobalAnsi(time_field);
            this.meta_field = Marshal.StringToHGlobalAnsi(meta_field);
            this.granularity = Marshal.StringToHGlobalAnsi(granularity);
        }
        public IntPtr time_field;
        public IntPtr meta_field;
        public IntPtr granularity;

        public void Dispose()
        {
            Marshal.FreeHGlobal(time_field);
            Marshal.FreeHGlobal(meta_field);
            Marshal.FreeHGlobal(granularity);
        }
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct CreateCollectionRequestWrapper : IDisposable
    {
        CreateCollectionRequestWrapper(string collectionname, ColCollationWrapper? collation = null, ColTimeseriesWrapper? timeseries = null, int expire_after_seconds = 0, bool change_stream_pre_and_post_images = false, bool capped = false, int max = 0, int size = 0)
        {
            this.collectionname = Marshal.StringToHGlobalAnsi(collectionname);
            this.collation = collation != null ? Marshal.AllocHGlobal(Marshal.SizeOf(typeof(ColCollationWrapper))) : IntPtr.Zero;
            this.timeseries = timeseries != null ? Marshal.AllocHGlobal(Marshal.SizeOf(typeof(ColTimeseriesWrapper))) : IntPtr.Zero;
            this.expire_after_seconds = expire_after_seconds;
            this.change_stream_pre_and_post_images = change_stream_pre_and_post_images;
            this.capped = capped;
            this.max = max;
            this.size = size;
            this.request_id = 0;
        }
        public void Dispose()
        {
            Marshal.FreeHGlobal(collectionname);
            if (collation != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(collation);
            }
            if (timeseries != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(timeseries);
            }
        }
        public IntPtr collectionname;
        public IntPtr collation;
        public IntPtr timeseries;
        public int expire_after_seconds;
        [MarshalAs(UnmanagedType.I1)]
        public bool change_stream_pre_and_post_images;
        [MarshalAs(UnmanagedType.I1)]
        public bool capped;
        public int max;
        public int size;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct CreateCollectionResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }
    public delegate void CreateCollectionCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DropCollectionResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }
    public delegate void DropCollectionCallback(IntPtr responsePtr);
    [StructLayout(LayoutKind.Sequential)]
    public struct GetIndexesResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int request_id;
    }
    public delegate void GetIndexesCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct CreateIndexRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr index;
        public IntPtr options;
        public IntPtr name;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct CreateIndexResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }
    public delegate void CreateIndexCallback(IntPtr responsePtr);
    [StructLayout(LayoutKind.Sequential)]
    public struct DropIndexResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int request_id;
    }
    public delegate void DropIndexCallback(IntPtr responsePtr);
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
        public int skip;
        public int top;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct QueryResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int request_id;
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
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct AggregateResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int request_id;
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
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct CountResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public int count;
        public IntPtr error;
        public int request_id;
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
        public int request_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct DistinctResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr results;
        public IntPtr error;
        public int results_len;
        public int request_id;
    }
    public delegate void DistinctCallback(IntPtr responsePtr);


    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOneRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr item;
        public int w;
        public bool j;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOneResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr result;
        public IntPtr error;
        public int request_id;
    }
    public delegate void InsertOneCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct InsertManyRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr items;
        public int w;
        public bool j;
        public bool skipresults;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct InsertManyResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr result;
        public IntPtr error;
        public int request_id;
    }
    public delegate void InsertManyCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct UpdateOneRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr item;
        public int w;
        public bool j;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UpdateOneResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr result;
        public IntPtr error;
        public int request_id;
    }
    public delegate void UpdateOneCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOrUpdateOneRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr uniqeness;
        public IntPtr item;
        public int w;
        public bool j;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct InsertOrUpdateOneResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr result;
        public IntPtr error;
        public int request_id;
    }
    public delegate void InsertOrUpdateOneCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteOneRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr id;
        public bool recursive;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteOneResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public int affectedrows;
        public IntPtr error;
        public int request_id;
    }
    public delegate void DeleteOneCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteManyRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr query;
        public bool recursive;
        public IntPtr ids;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteManyResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public int affectedrows;
        public IntPtr error;
        public int request_id;
    }
    public delegate void DeleteManyCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DownloadRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr id;
        public IntPtr folder;
        public IntPtr filename;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct DownloadResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr filename;
        public IntPtr error;
        public int request_id;
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
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UploadResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr id;
        public IntPtr error;
        public int request_id;
    }
    public delegate void UploadCallback(IntPtr responsePtr);
    [StructLayout(LayoutKind.Sequential)]
    public struct WatchRequestWrapper
    {
        public IntPtr collectionname;
        public IntPtr paths;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct WatchResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr watchid;
        public IntPtr error;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct WatchEventWrapper
    {
        public IntPtr id;
        public IntPtr operation;
        public IntPtr document;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UnWatchResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct QueueEventWrapper
    {
        public IntPtr queuename;
        public IntPtr correlation_id;
        public IntPtr replyto;
        public IntPtr routingkey;
        public IntPtr exchangename;
        public IntPtr data;
    }
    public delegate void QueueEventCallback(IntPtr eventStr);
    [StructLayout(LayoutKind.Sequential)]
    public struct RegisterQueueRequestWrapper
    {
        public IntPtr queuename;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct RegisterQueueResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr queuename;
        public IntPtr error;
    }
    public delegate void RegisterQueueCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct RegisterExchangeRequestWrapper
    {
        public IntPtr exchangename;
        public IntPtr algorithm;
        public IntPtr routingkey;
        [MarshalAs(UnmanagedType.I1)]
        public bool addqueue;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct RegisterExchangeResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr queuename;
        public IntPtr error;
    }
    public delegate void RegisterExchangeCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct UnRegisterQueueResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct QueueMessageRequestWrapper
    {
        public IntPtr queuename;
        public IntPtr correlation_id;
        public IntPtr replyto;
        public IntPtr routingkey;
        public IntPtr exchangename;
        public IntPtr data;
        [MarshalAs(UnmanagedType.I1)]
        public bool striptoken;
        public int expiration;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct QueueMessageResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
    }
    public delegate void QueueMessageCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct WorkitemFileWrapper
    {
        public IntPtr filename;
        public IntPtr id;
        public bool compressed;
        // public IntPtr file;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct WorkitemWrapper
    {
        public IntPtr id;
        public IntPtr name;
        public IntPtr payload;
        public int priority;
        public ulong nextrun;
        public ulong lastrun;
        public IntPtr files;
        public int files_len;
        public IntPtr state;
        public IntPtr wiq;
        public IntPtr wiqid;
        public int retries;
        public IntPtr username;
        public IntPtr success_wiqid;
        public IntPtr failed_wiqid;
        public IntPtr success_wiq;
        public IntPtr failed_wiq;
        public IntPtr errormessage;
        public IntPtr errorsource;
        public IntPtr errortype;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct PushWorkitemRequestWrapper
    {
        public IntPtr wiq;
        public IntPtr wiqid;
        public IntPtr name;
        public IntPtr payload;
        public ulong nextrun;
        public IntPtr success_wiqid;
        public IntPtr failed_wiqid;
        public IntPtr success_wiq;
        public IntPtr failed_wiq;
        public int priority;
        public IntPtr files;
        public int files_len;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct PushWorkitemResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public IntPtr workitem;
        public int request_id;
    }
    public delegate void PushWorkitemCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct PopWorkitemRequestWrapper
    {
        public IntPtr wiq;
        public IntPtr wiqid;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct PopWorkitemResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public IntPtr workitem;
        public int request_id;
    }
    public delegate void PopWorkitemCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct UpdateWorkitemRequestWrapper
    {
        public IntPtr workitem;
        public bool ignoremaxretries;
        public IntPtr files;
        public int files_len;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct UpdateWorkitemResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public IntPtr workitem;
        public int request_id;
    }
    public delegate void UpdateWorkitemCallback(IntPtr responsePtr);

    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteWorkitemRequestWrapper
    {
        public string id;
        public int request_id;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct DeleteWorkitemResponseWrapper
    {
        [MarshalAs(UnmanagedType.I1)]
        public bool success;
        public IntPtr error;
        public int request_id;
    }
    public delegate void DeleteWorkitemCallback(IntPtr responsePtr);
    #endregion


    #region dll imports
    private static string GetLibraryPath()
    {
        string libfile;
        var arc = System.Runtime.InteropServices.RuntimeInformation.ProcessArchitecture;
        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                if (Environment.Is64BitProcess)
                {
                    libfile = arc == Architecture.X64 ? "openiap-windows-x64.dll" : "openiap-windows-arm64.dll";
                }
                else
                {
                    libfile = "openiap-windows-i686.dll";
                }
                break;
            case PlatformID.MacOSX:
                if (!Environment.Is64BitProcess) throw new LibraryLoadError("macOS requires a 64-bit process");
                libfile = arc == Architecture.Arm64 ? "libopeniap-macos-arm64.dylib" : "libopeniap-macos-x64.dylib";
                break;
            case PlatformID.Unix:
                if (!Environment.Is64BitProcess) throw new LibraryLoadError("Linux requires a 64-bit process");
                if (System.IO.File.Exists("/etc/alpine-release"))
                {
                    libfile = arc == Architecture.Arm64 ? "libopeniap-linux-musl-arm64.a" : "libopeniap-linux-musl-x64.a";
                }
                else
                {
                    libfile = arc == Architecture.Arm64 ? "libopeniap-linux-arm64.so" : "libopeniap-linux-x64.so";
                }
                break;
            default:
                throw new PlatformNotSupportedException("Unsupported OS platform");
        }

        string libDir = System.IO.Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "runtimes");
        string libPath = System.IO.Path.Combine(libDir, libfile);

        if (System.IO.File.Exists(libPath)) return libPath;

        libDir = System.IO.Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "lib");
        libPath = System.IO.Path.Combine(libDir, libfile);

        if (System.IO.File.Exists(libPath)) return libPath;

        libDir = System.IO.Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "../../../lib");
        libPath = System.IO.Path.Combine(libDir, libfile);

        if (System.IO.File.Exists(libPath)) return libPath;

        libDir = AppDomain.CurrentDomain.BaseDirectory + "../../../../target/debug/";
        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                libfile = "libopeniap_clib.dll";
                break;
            case PlatformID.MacOSX:
                libfile = "libopeniap_clib.dylib";
                break;
            default:
                libfile = "libopeniap_clib.so";
                break;
        }
        libPath = System.IO.Path.Combine(libDir, libfile);
        if (System.IO.File.Exists(libPath)) return libPath;

        throw new LibraryLoadError($"Library {libfile} not found in runtimes directory.");
    }
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr create_client();

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void enable_tracing(string rust_log, string tracing);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void disable_tracing();


    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void ClientEventCallback(IntPtr eventStr);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr on_client_event_async(IntPtr client, ClientEventCallback event_callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_event_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_client_event(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl, EntryPoint = "off_client_event")]
    public static extern IntPtr int_off_client_event(string eventid);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_off_event_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_set_agent_name(IntPtr client, string agentname);
    public delegate void ConnectCallback(IntPtr ConnectResponseWrapperPtr);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void connect_async(IntPtr client, string url, int request_id, ConnectCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_client(IntPtr client);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void signin_async(IntPtr client, ref SigninRequestWrapper request, SigninCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_signin_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void client_disconnect(IntPtr client);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void list_collections_async(IntPtr client, bool includehist, int request_id, ListCollectionsCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_list_collections_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void create_collection_async(IntPtr client, ref CreateCollectionRequestWrapper options, CreateCollectionCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_create_collection_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void drop_collection_async(IntPtr client, string collectionname, int request_id, DropCollectionCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_drop_collection_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void get_indexes_async(IntPtr client, string collectionname, int request_id, GetIndexesCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_get_indexes_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void create_index_async(IntPtr client, ref CreateIndexRequestWrapper request, CreateIndexCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_create_index_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void drop_index_async(IntPtr client, string collectionname, string idnexname, int request_id, DropIndexCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_drop_index_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void query_async(IntPtr client, ref QueryRequestWrapper request, QueryCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_query_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void aggregate_async(IntPtr client, ref AggregateRequestWrapper request, AggregateCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_aggregate_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void count_async(IntPtr client, ref CountRequestWrapper request, CountCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_count_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void distinct_async(IntPtr client, ref DistinctRequestWrapper request, DistinctCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_distinct_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void insert_one_async(IntPtr client, ref InsertOneRequestWrapper request, InsertOneCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_insert_one_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void insert_many_async(IntPtr client, ref InsertManyRequestWrapper request, InsertManyCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_insert_many_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void update_one_async(IntPtr client, ref UpdateOneRequestWrapper request, UpdateOneCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_update_one_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void insert_or_update_one_async(IntPtr client, ref InsertOrUpdateOneRequestWrapper request, InsertOrUpdateOneCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_insert_or_update_one_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void delete_one_async(IntPtr client, ref DeleteOneRequestWrapper request, DeleteOneCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_delete_one_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void delete_many_async(IntPtr client, ref DeleteManyRequestWrapper request, DeleteManyCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_delete_many_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void download_async(IntPtr client, ref DownloadRequestWrapper request, DownloadCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_download_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void upload_async(IntPtr client, ref UploadRequestWrapper request, UploadCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_upload_response(IntPtr response);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void WatchCallback(IntPtr eventStr);
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void WatchEventCallback(IntPtr eventStr);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void watch_async_async(IntPtr client, ref WatchRequestWrapper request, WatchCallback callback, WatchEventCallback event_callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_watch_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_watch_event(IntPtr response);


    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr unwatch(IntPtr client, string watchid);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_unwatch_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr register_queue_async(IntPtr client, ref RegisterQueueRequestWrapper request, QueueEventCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_register_queue_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr register_exchange_async(IntPtr client, ref RegisterExchangeRequestWrapper request, QueueEventCallback callback);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_register_exchange_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr unregister_queue(IntPtr client, string queuename);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_unregister_queue_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr queue_message(IntPtr client, ref QueueMessageRequestWrapper request);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_queue_message_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void push_workitem_async(IntPtr client, ref PushWorkitemRequestWrapper request, PushWorkitemCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_push_workitem_response(IntPtr response);

    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void pop_workitem_async(IntPtr client, ref PopWorkitemRequestWrapper request, string downloadfolder, PopWorkitemCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_pop_workitem_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void update_workitem_async(IntPtr client, ref UpdateWorkitemRequestWrapper request, UpdateWorkitemCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_update_workitem_response(IntPtr response);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void delete_workitem_async(IntPtr client, ref DeleteWorkitemRequestWrapper request, DeleteWorkitemCallback callback);
    [DllImport("libopeniap", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_delete_workitem_response(IntPtr response);
    #endregion
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
        clientPtr = create_client();
        var clientWrapper = Marshal.PtrToStructure<ClientWrapper>(clientPtr);
        if (!clientWrapper.success)
        {
            var errorMsg = Marshal.PtrToStringAnsi(clientWrapper.error) ?? "Unknown error";
            throw new ClientCreationError(errorMsg);
        }
        client = clientWrapper;
        client_set_agent_name(clientPtr, "dotnet");
        _PopWorkitemCallbackDelegate = _PopWorkitemCallback;
        _UpdateWorkitemCallbackDelegate = _UpdateWorkitemCallback;
        _DeleteWorkitemCallbackDelegate = _DeleteWorkitemCallback;
        _PushWorkitemCallbackDelegate = _PushWorkitemCallback;
        _DeleteManyCallbackDelegate = _DeleteManyCallback;
        _DeleteOneCallbackDelegate = _DeleteOneCallback;
        _InsertOrUpdateOneCallbackDelegate = _InsertOrUpdateOneCallback;
        _UploadCallbackDelegate = _UploadCallback;
        _DownloadCallbackDelegate = _DownloadCallback;
        _UpdateOneCallbackDelegate = _UpdateOneCallback;
        _InsertManyCallbackDelegate = _InsertManyCallback;
        _InsertOneCallbackDelegate = _InsertOneCallback;
        _DistinctCallbackDelegate = _DistinctCallback;
        _CountCallbackDelegate = _CountCallback;
        _AggregateCallbackDelegate = _AggregateCallback;
        _QueryCallbackDelegate = _QueryCallback;
        _DropIndexCallbackDelegate = _DropIndexCallback;
        _CreateIndexCallbackDelegate = _CreateIndexCallback;
        _GetIndexesCallbackDelegate = _GetIndexesCallback;
        _SigninCallbackDelegate = _SigninCallback;
        _CreateCollectionCallbackDelegate = _CreateCollectionCallback;
        _ListCollectionsCallbackDelegate = _ListCollectionsCallback;
        _ConnectCallbackDelegate = _ConnectCallback;
    }
    public void enabletracing(string rust_log = "", string tracing = "")
    {
        enable_tracing(rust_log, tracing);
        informing = true;
        if (rust_log.Contains("verbose")) verbosing = true;
        if (rust_log.Contains("trace")) this.tracing = true;
    }
    public void disabletracing()
    {
        disable_tracing();
    }
    public void info(params object[] objs)
    {
        if (informing)
        {
            Console.Write("dotnet: ");
            objs.ToList().ForEach(obj => Console.Write(obj));
            Console.WriteLine();
        }
    }
    public void verbose(params object[] objs)
    {
        if (verbosing)
        {
            Console.Write("dotnet: ");
            objs.ToList().ForEach(obj => Console.Write(obj));
            Console.WriteLine();
        }
    }
    public void trace(params object[] objs)
    {
        if (tracing)
        {
            Console.Write("dotnet: ");
            objs.ToList().ForEach(obj => Console.Write(obj));
            Console.WriteLine();
        }
    }
    void _ConnectCallback(IntPtr clientWrapperPtr)
    {
        try
        {
            var clientWrapper = Marshal.PtrToStructure<ConnectResponseWrapper>(clientWrapperPtr);
            int requestId = clientWrapper.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!clientWrapper.success)
                {
                    string error = Marshal.PtrToStringAnsi(clientWrapper.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, clientWrapper);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_signin_response(clientWrapperPtr);
        }
    }
    public Task connect(string url = "")
    {
        var tcs = new TaskCompletionSource<ConnectResponseWrapper>();
        int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
        CallbackRegistry.TryAddCallback(requestId, tcs);
        connect_async(clientPtr, url, requestId, _ConnectCallbackDelegate);

        return tcs.Task;
    }
    public void disconnect()
    {
        if (clientPtr != IntPtr.Zero)
        {
            client_disconnect(clientPtr);
        }
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
    private void _ListCollectionsCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<ListCollectionsResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, results);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_list_collections_response(responsePtr);
        }
    }
    public Task<T> ListCollections<T>(bool includehist = false)
    {
        var tcs = new TaskCompletionSource<string>();
        int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
        list_collections_async(clientPtr, includehist, requestId, _ListCollectionsCallbackDelegate);
        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            jsonString => jsonString, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    // If the user wants the raw JSON, just return it
                    return (T)(object)responseJson;
                }
                else
                {
                    // Deserialize the JSON string into the specified type T
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }
    private void _CreateCollectionCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<CreateCollectionResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, "");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_create_collection_response(responsePtr);
        }
    }
    public Task CreateCollection(string collectionname, ColCollationWrapper? collation = null, ColTimeseriesWrapper? timeseries = null, int expireAfterSeconds = 0, bool changeStreamPreAndPostImages = false, bool capped = false, int max = 0, int size = 0)
    {
        var tcs = new TaskCompletionSource();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);

        CreateCollectionRequestWrapper request = new CreateCollectionRequestWrapper();
        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            request = new CreateCollectionRequestWrapper
            {
                collectionname = collectionnamePtr,
                collation = collation != null ? Marshal.AllocHGlobal(Marshal.SizeOf(typeof(ColCollationWrapper))) : IntPtr.Zero,
                timeseries = timeseries != null ? Marshal.AllocHGlobal(Marshal.SizeOf(typeof(ColTimeseriesWrapper))) : IntPtr.Zero,
                expire_after_seconds = expireAfterSeconds,
                change_stream_pre_and_post_images = changeStreamPreAndPostImages,
                capped = capped,
                max = max,
                size = size,
                request_id = requestId
            };

            // Copy collation and timeseries if provided
            if (collation != null)
            {
                Marshal.StructureToPtr(collation, request.collation, false);
            }

            if (timeseries != null)
            {
                Marshal.StructureToPtr(timeseries, request.timeseries, false);
            }

            // Invoke the native async function
            create_collection_async(clientPtr, ref request, _CreateCollectionCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            if (request.collation != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(request.collation);
            }
            if (request.timeseries != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(request.timeseries);
            }
        }
        return tcs.Task;
    }
    public Task DropCollection(string collectionname)
    {
        var tcs = new TaskCompletionSource();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);

        DropCollectionCallback callback = new DropCollectionCallback((IntPtr responsePtr) =>
        {
            try
            {
                if (responsePtr == IntPtr.Zero)
                {
                    tcs.SetException(new ClientError("Callback got null response"));
                    return;
                }

                var response = Marshal.PtrToStructure<DropCollectionResponseWrapper>(responsePtr);
                free_drop_collection_response(responsePtr);

                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    tcs.SetException(new ClientError(error));
                }
                else
                {
                    tcs.SetResult();
                }
            }
            catch (Exception ex)
            {
                tcs.SetException(ex);
            }
        });

        drop_collection_async(clientPtr, collectionname, 0, callback);

        return tcs.Task;
    }
    private void _GetIndexesCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<GetIndexesResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {

                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, results);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_get_indexes_response(responsePtr);
        }
    }
    public Task<T> GetIndexes<T>(string collectionname)
    {
        var tcs = new TaskCompletionSource<string>();

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Invoke the native async function
            get_indexes_async(clientPtr, collectionname, requestId, _GetIndexesCallbackDelegate);
        }
        finally
        {
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }
    private void _CreateIndexCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<CreateIndexResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, "");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_create_index_response(responsePtr);
        }
    }
    public Task CreateIndex(string collectionname, string index, string options = "", string name = "")
    {
        var tcs = new TaskCompletionSource();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr indexPtr = Marshal.StringToHGlobalAnsi(index);
        IntPtr optionsPtr = Marshal.StringToHGlobalAnsi(options);
        IntPtr namePtr = Marshal.StringToHGlobalAnsi(name);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            CreateIndexRequestWrapper request = new CreateIndexRequestWrapper
            {
                collectionname = collectionnamePtr,
                index = indexPtr,
                options = optionsPtr,
                name = namePtr,
                request_id = requestId
            };
            // Invoke the native async function
            create_index_async(clientPtr, ref request, _CreateIndexCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(indexPtr);
            Marshal.FreeHGlobal(optionsPtr);
            Marshal.FreeHGlobal(namePtr);
        }

        return tcs.Task;
    }
    private void _DropIndexCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<DropIndexResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, "");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_drop_index_response(responsePtr);
        }
    }
    public Task DropIndex(string collectionname, string indexname)
    {
        var tcs = new TaskCompletionSource();
        int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);

        drop_index_async(clientPtr, collectionname, indexname, requestId, _DropIndexCallbackDelegate);

        return tcs.Task;
    }

    private void _SigninCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<SigninResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string jwt = Marshal.PtrToStringAnsi(response.jwt) ?? string.Empty;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, jwt);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_signin_response(responsePtr);
        }
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
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            SigninRequestWrapper request = new SigninRequestWrapper
            {
                username = usernamePtr,
                password = passwordPtr,
                jwt = jwtPtr,
                agent = agentPtr,
                version = versionPtr,
                longtoken = false,
                validateonly = false,
                ping = false,
                request_id = requestId
            };

            signin_async(clientPtr, ref request, _SigninCallbackDelegate);
        }
        catch (Exception ex)
        {
            tcs.SetException(ex);
        }
        finally
        {
            Marshal.FreeHGlobal(usernamePtr);
            Marshal.FreeHGlobal(passwordPtr);
            Marshal.FreeHGlobal(jwtPtr);
            Marshal.FreeHGlobal(agentPtr);
            Marshal.FreeHGlobal(versionPtr);
        }
        return tcs.Task;
    }
    private void _QueryCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<QueryResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, results);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_query_response(responsePtr);
        }
    }
    public Task<T> Query<T>(string collectionname, string query, string projection = "", string orderby = "", string queryas = "", bool explain = false, int skip = 0, int top = 100)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        IntPtr projectionPtr = Marshal.StringToHGlobalAnsi(projection);
        IntPtr orderbyPtr = Marshal.StringToHGlobalAnsi(orderby);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            QueryRequestWrapper request = new QueryRequestWrapper
            {
                collectionname = collectionnamePtr,
                query = queryPtr,
                projection = projectionPtr,
                orderby = orderbyPtr,
                queryas = queryasPtr,
                explain = explain,
                skip = skip,
                top = top,
                request_id = requestId
            };

            // Invoke the native async function
            query_async(clientPtr, ref request, _QueryCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(projectionPtr);
            Marshal.FreeHGlobal(orderbyPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (responseJson == null || responseJson == "")
                {
                    responseJson = "[]";
                }
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }
    private void _AggregateCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<AggregateResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {

                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string results = Marshal.PtrToStringAnsi(response.results) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, results);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_aggregate_response(responsePtr);
        }
    }
    public Task<T> Aggregate<T>(string collectionname, string aggregates, string queryas = "", string hint = "", bool explain = false)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr aggregatesPtr = Marshal.StringToHGlobalAnsi(aggregates);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);
        IntPtr hintPtr = Marshal.StringToHGlobalAnsi(hint);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            AggregateRequestWrapper request = new AggregateRequestWrapper
            {
                collectionname = collectionnamePtr,
                aggregates = aggregatesPtr,
                queryas = queryasPtr,
                hint = hintPtr,
                explain = explain,
                request_id = requestId
            };

            // Invoke the native async function
            aggregate_async(clientPtr, ref request, _AggregateCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(aggregatesPtr);
            Marshal.FreeHGlobal(queryasPtr);
            Marshal.FreeHGlobal(hintPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }

    private void _CountCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<CountResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                int resCount = (int)response.count;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, resCount);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_count_response(responsePtr);
        }
    }

    public Task<int> Count(string collectionname, string query = "", string queryas = "", bool explain = false)
    {
        var tcs = new TaskCompletionSource<int>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        IntPtr queryasPtr = Marshal.StringToHGlobalAnsi(queryas);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            CountRequestWrapper request = new CountRequestWrapper
            {
                collectionname = collectionnamePtr,
                query = queryPtr,
                queryas = queryasPtr,
                explain = explain,
                request_id = requestId
            };

            count_async(clientPtr, ref request, _CountCallbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(queryasPtr);
        }
        return tcs.Task;
    }
    private void _DistinctCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<DistinctResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                bool success = response.success;

                if (!success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    int resultsCount = (int)response.results_len;
                    string[] results = new string[resultsCount];
                    IntPtr[] resultPtrs = new IntPtr[resultsCount];
                    Marshal.Copy(response.results, resultPtrs, 0, resultsCount);

                    for (int i = 0; i < resultsCount; i++)
                    {
                        results[i] = Marshal.PtrToStringAnsi(resultPtrs[i]) ?? string.Empty;
                    }
                    CallbackRegistry.TrySetResult(requestId, results);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_distinct_response(responsePtr);
        }
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
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            DistinctRequestWrapper request = new DistinctRequestWrapper
            {
                collectionname = collectionnamePtr,
                field = fieldPtr,
                query = queryPtr,
                queryas = queryasPtr,
                explain = explain,
                request_id = requestId
            };

            distinct_async(clientPtr, ref request, _DistinctCallbackDelegate);
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
    private void _InsertOneCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<InsertOneResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string result = Marshal.PtrToStringAnsi(response.result) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, result);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_insert_one_response(responsePtr);
        }
    }
    public Task<T> InsertOne<T>(string collectionname, string item, int w = 1, bool j = false)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr itemPtr = Marshal.StringToHGlobalAnsi(item);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            InsertOneRequestWrapper request = new InsertOneRequestWrapper
            {
                collectionname = collectionnamePtr,
                item = itemPtr,
                w = w,
                j = j,
                request_id = requestId
            };

            // Invoke the native async function
            insert_one_async(clientPtr, ref request, _InsertOneCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(itemPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }

    private void _InsertManyCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<InsertManyResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string result = Marshal.PtrToStringAnsi(response.result) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, result);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_insert_many_response(responsePtr);
        }
    }
    public Task<T> InsertMany<T>(string collectionname, string items, int w = 1, bool j = false, bool skipresults = false)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr itemsPtr = Marshal.StringToHGlobalAnsi(items);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            InsertManyRequestWrapper request = new InsertManyRequestWrapper
            {
                collectionname = collectionnamePtr,
                items = itemsPtr,
                w = w,
                j = j,
                skipresults = skipresults,
                request_id = requestId
            };

            // Invoke the native async function
            insert_many_async(clientPtr, ref request, _InsertManyCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(itemsPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }

    private void _UpdateOneCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<UpdateOneResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string result = Marshal.PtrToStringAnsi(response.result) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, result);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_update_one_response(responsePtr);
        }
    }

    public Task<T> UpdateOne<T>(string collectionname, string item, int w = 1, bool j = false)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr itemPtr = Marshal.StringToHGlobalAnsi(item);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            UpdateOneRequestWrapper request = new UpdateOneRequestWrapper
            {
                collectionname = collectionnamePtr,
                item = itemPtr,
                w = w,
                j = j,
                request_id = requestId
            };

            // Invoke the native async function
            update_one_async(clientPtr, ref request, _UpdateOneCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(itemPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }

    private void _InsertOrUpdateOneCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<InsertOrUpdateOneResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                if (!response.success)
                {
                    string error = Marshal.PtrToStringAnsi(response.error) ?? "Unknown error";
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    string result = Marshal.PtrToStringAnsi(response.result) ?? string.Empty;
                    CallbackRegistry.TrySetResult(requestId, result);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_insert_or_update_one_response(responsePtr);
        }
    }

    public Task<T> InsertOrUpdateOne<T>(string collectionname, string item, string uniqeness = "_id", int w = 1, bool j = false)
    {
        var tcs = new TaskCompletionSource<string>();

        // Allocate unmanaged memory for the strings
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr uniqenessPtr = Marshal.StringToHGlobalAnsi(uniqeness);
        IntPtr itemPtr = Marshal.StringToHGlobalAnsi(item);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            // Create the request wrapper
            InsertOrUpdateOneRequestWrapper request = new InsertOrUpdateOneRequestWrapper
            {
                collectionname = collectionnamePtr,
                uniqeness = uniqenessPtr,
                item = itemPtr,
                w = w,
                j = j,
                request_id = requestId
            };

            // Invoke the native async function
            insert_or_update_one_async(clientPtr, ref request, _InsertOrUpdateOneCallbackDelegate);
        }
        finally
        {
            // Free unmanaged memory
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(uniqenessPtr);
            Marshal.FreeHGlobal(itemPtr);
        }

        // Use the helper to handle continuation
        return AsyncContinuationHelper.ProcessResponseAsync<string, T>(
            tcs.Task,
            responseJson => responseJson, // Simply pass the JSON string as is
            responseJson =>
            {
                if (typeof(T) == typeof(string))
                {
                    return (T)(object)responseJson;
                }
                else
                {
                    return JsonSerializer.Deserialize<T>(responseJson)!;
                }
            }
        );
    }

    private void _DeleteOneCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<DeleteOneResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                int affectedrows = (int)response.affectedrows;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, affectedrows);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            this.trace("Freeing memory");
            free_delete_one_response(responsePtr);
        }
    }

    public Task<int> DeleteOne(string collectionname, string id, bool recursive = false)
    {
        var tcs = new TaskCompletionSource<int>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr idPtr = Marshal.StringToHGlobalAnsi(id);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            DeleteOneRequestWrapper request = new DeleteOneRequestWrapper
            {
                collectionname = collectionnamePtr,
                id = idPtr,
                recursive = recursive,
                request_id = requestId
            };

            CallbackRegistry.TryAddCallback(requestId, tcs);

            delete_one_async(clientPtr, ref request, _DeleteOneCallbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(idPtr);
        }
        return tcs.Task;
    }

    private void _DeleteManyCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<DeleteManyResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                int affectedrows = (int)response.affectedrows;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, affectedrows);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            this.trace("Freeing memory");
            free_delete_many_response(responsePtr);
        }
    }

    public Task<int> DeleteMany(string collectionname, string query = "", string[]? ids = null, bool recursive = false)
    {
        var tcs = new TaskCompletionSource<int>();
        if (ids == null) ids = new string[] { "" };
        if (ids.Length == 0) ids = new string[] { "" };
        // ids = ids.Concat(new string[] { "test" }).ToArray();

        IntPtr idsPtr = Marshal.AllocHGlobal(ids.Length * IntPtr.Size);
        IntPtr[] stringPointers = new IntPtr[ids.Length];
        for (int i = 0; i < ids.Length; i++)
        {
            stringPointers[i] = Marshal.StringToHGlobalAnsi(ids[i]);
            Marshal.WriteIntPtr(idsPtr, i * IntPtr.Size, stringPointers[i]);
        }

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr queryPtr = Marshal.StringToHGlobalAnsi(query);
        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);

            DeleteManyRequestWrapper request = new DeleteManyRequestWrapper
            {
                collectionname = collectionnamePtr,
                query = queryPtr,
                recursive = recursive,
                ids = idsPtr,
                request_id = requestId
            };

            delete_many_async(clientPtr, ref request, _DeleteManyCallbackDelegate);
        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(queryPtr);
            Marshal.FreeHGlobal(idsPtr);
        }
        return tcs.Task;
    }

    private void _DownloadCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<DownloadResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string filename = Marshal.PtrToStringAnsi(response.filename) ?? string.Empty;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, filename);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_query_response(responsePtr);
        }
    }
    public Task<string> download(string collectionname, string id, string folder = "", string filename = "")
    {
        var tcs = new TaskCompletionSource<string>();

        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);
        IntPtr idPtr = Marshal.StringToHGlobalAnsi(id);
        IntPtr folderPtr = Marshal.StringToHGlobalAnsi(folder);
        IntPtr filenamePtr = Marshal.StringToHGlobalAnsi(filename);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            DownloadRequestWrapper request = new DownloadRequestWrapper
            {
                collectionname = collectionnamePtr,
                id = idPtr,
                folder = folderPtr,
                filename = filenamePtr,
                request_id = requestId
            };

            download_async(clientPtr, ref request, _DownloadCallbackDelegate);
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
    private void _UploadCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<UploadResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string id = Marshal.PtrToStringAnsi(response.id) ?? string.Empty;
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, id);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_upload_response(responsePtr);
        }
    }
    public Task<string> upload(string filepath, string filename = "", string mimetype = "", string metadata = "", string collectionname = "")
    {
        var tcs = new TaskCompletionSource<string>();

        IntPtr filepathPtr = Marshal.StringToHGlobalAnsi(filepath);
        IntPtr filenamePtr = Marshal.StringToHGlobalAnsi(filename);
        IntPtr mimetypePtr = Marshal.StringToHGlobalAnsi(mimetype);
        IntPtr metadataPtr = Marshal.StringToHGlobalAnsi(metadata);
        IntPtr collectionnamePtr = Marshal.StringToHGlobalAnsi(collectionname);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            UploadRequestWrapper request = new UploadRequestWrapper
            {
                filepath = filepathPtr,
                filename = filenamePtr,
                mimetype = mimetypePtr,
                metadata = metadataPtr,
                collectionname = collectionnamePtr,
                request_id = requestId
            };

            upload_async(clientPtr, ref request, _UploadCallbackDelegate);
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
    public string on_client_event(Action<ClientEvent> eventHandler)
    {
        string eventid = "";
        try
        {

            var callback = new ClientEventCallback((IntPtr clientEventWrapper) =>
            {
                var eventObj = Marshal.PtrToStructure<ClientEventWrapper>(clientEventWrapper);
                var clientEvent = new ClientEvent
                {
                    evt = Marshal.PtrToStringAnsi(eventObj.evt) ?? string.Empty,
                    reason = Marshal.PtrToStringAnsi(eventObj.reason) ?? string.Empty
                };
                free_client_event(clientEventWrapper);
                eventHandler(clientEvent);
            });

            var reqptr = on_client_event_async(clientPtr, callback);
            ClientEventResponseWrapper response = Marshal.PtrToStructure<ClientEventResponseWrapper>(reqptr);
            string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
            eventid = Marshal.PtrToStringAnsi(response.eventid) ?? string.Empty;
            bool success = response.success;
            free_event_response(reqptr);
            if (!success)
            {
                throw new ClientError(error);
            }
        }
        finally
        {
        }
        return eventid;
    }
    public void off_client_event(string eventid)
    {
        IntPtr eventidPtr = Marshal.StringToHGlobalAnsi(eventid);
        try
        {
            var response = int_off_client_event(eventid);
            var responseWrapper = Marshal.PtrToStructure<OffClientEventResponseWrapper>(response);
            string error = Marshal.PtrToStringAnsi(responseWrapper.error) ?? string.Empty;
            bool success = responseWrapper.success;
            free_off_event_response(response);
            if (!success)
            {
                throw new ClientError(error);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(eventidPtr);
        }

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

            var callback = new WatchEventCallback((IntPtr WatchEventWrapper) =>
            {
                var eventObj = Marshal.PtrToStructure<WatchEventWrapper>(WatchEventWrapper);
                var watchEvent = new WatchEvent
                {
                    id = Marshal.PtrToStringAnsi(eventObj.id) ?? string.Empty,
                    operation = Marshal.PtrToStringAnsi(eventObj.operation) ?? string.Empty,
                    document = Marshal.PtrToStringAnsi(eventObj.document) ?? string.Empty
                };
                free_watch_event(WatchEventWrapper);
                eventHandler(watchEvent);
            });

            void Callback(IntPtr responsePtr)
            {
                this.trace("register watch callback");
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

            watch_async_async(clientPtr, ref request, callbackDelegate, callback);

        }
        finally
        {
            Marshal.FreeHGlobal(collectionnamePtr);
            Marshal.FreeHGlobal(pathsPtr);
        }
        return tcs.Task;
    }
    public void UnWatch(string watchid)
    {
        IntPtr watchidPtr = Marshal.StringToHGlobalAnsi(watchid);
        try
        {
            var response = unwatch(clientPtr, watchid);
            var responseWrapper = Marshal.PtrToStructure<UnWatchResponseWrapper>(response);
            string error = Marshal.PtrToStringAnsi(responseWrapper.error) ?? string.Empty;
            bool success = responseWrapper.success;
            free_unwatch_response(response);
            if (!success)
            {
                throw new ClientError(error);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(watchidPtr);
        }
    }
    public Task<string> RegisterQueue(string queuename, Action<QueueEvent> eventHandler)
    {
        if (eventHandler == null) throw new ArgumentNullException(nameof(eventHandler));
        var tcs = new TaskCompletionSource<string>();
        IntPtr queuenamePtr = Marshal.StringToHGlobalAnsi(queuename);

        try
        {
            RegisterQueueRequestWrapper request = new RegisterQueueRequestWrapper
            {
                queuename = queuenamePtr
            };

            var callback = new QueueEventCallback((IntPtr QueueEventWrapperptr) =>
            {
                var eventObj = Marshal.PtrToStructure<QueueEventWrapper>(QueueEventWrapperptr);
                var watchEvent = new QueueEvent
                {
                    queuename = Marshal.PtrToStringAnsi(eventObj.queuename) ?? string.Empty,
                    correlation_id = Marshal.PtrToStringAnsi(eventObj.correlation_id) ?? string.Empty,
                    replyto = Marshal.PtrToStringAnsi(eventObj.replyto) ?? string.Empty,
                    routingkey = Marshal.PtrToStringAnsi(eventObj.routingkey) ?? string.Empty,
                    exchangename = Marshal.PtrToStringAnsi(eventObj.exchangename) ?? string.Empty,
                    data = Marshal.PtrToStringAnsi(eventObj.data) ?? string.Empty,
                };
                eventHandler(watchEvent);
            });

            void Callback(IntPtr responsePtr)
            {
                this.trace("register watch callback");
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<RegisterQueueResponseWrapper>(responsePtr);
                    string queuename = Marshal.PtrToStringAnsi(response.queuename) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_register_queue_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(queuename);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }
            var callbackDelegate = new RegisterQueueCallback(Callback);
            var queuecallbackkDelegate = new RegisterQueueCallback(Callback);

            var response = register_queue_async(clientPtr, ref request, callback);
            Callback(response);

        }
        finally
        {
            Marshal.FreeHGlobal(queuenamePtr);
        }
        return tcs.Task;
    }
    public Task<string> RegisterExchange(string exchangename, string algorithm = "", string routingkey = "", bool addqueue = true, Action<QueueEvent>? eventHandler = null)
    {
        if (eventHandler == null) throw new ArgumentNullException(nameof(eventHandler));
        var tcs = new TaskCompletionSource<string>();
        IntPtr exchangenamePtr = Marshal.StringToHGlobalAnsi(exchangename);
        IntPtr algorithmPtr = Marshal.StringToHGlobalAnsi(algorithm);
        IntPtr routingkeyPtr = Marshal.StringToHGlobalAnsi(routingkey);

        try
        {
            RegisterExchangeRequestWrapper request = new RegisterExchangeRequestWrapper
            {
                exchangename = exchangenamePtr,
                algorithm = algorithmPtr,
                routingkey = routingkeyPtr,
                addqueue = addqueue
            };

            var callback = new QueueEventCallback((IntPtr QueueEventWrapperptr) =>
            {
                var eventObj = Marshal.PtrToStructure<QueueEventWrapper>(QueueEventWrapperptr);
                var watchEvent = new QueueEvent
                {
                    queuename = Marshal.PtrToStringAnsi(eventObj.queuename) ?? string.Empty,
                    correlation_id = Marshal.PtrToStringAnsi(eventObj.correlation_id) ?? string.Empty,
                    replyto = Marshal.PtrToStringAnsi(eventObj.replyto) ?? string.Empty,
                    routingkey = Marshal.PtrToStringAnsi(eventObj.routingkey) ?? string.Empty,
                    exchangename = Marshal.PtrToStringAnsi(eventObj.exchangename) ?? string.Empty,
                    data = Marshal.PtrToStringAnsi(eventObj.data) ?? string.Empty,
                };
                eventHandler(watchEvent);
            });

            void Callback(IntPtr responsePtr)
            {
                this.trace("register watch callback");
                try
                {
                    if (responsePtr == IntPtr.Zero)
                    {
                        tcs.SetException(new ClientError("Callback got null response"));
                        return;
                    }

                    var response = Marshal.PtrToStructure<RegisterExchangeResponseWrapper>(responsePtr);
                    string queuename = Marshal.PtrToStringAnsi(response.queuename) ?? string.Empty;
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_register_exchange_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult(queuename);
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }
            // var callbackDelegate = new RegisterExchangeCallback(Callback);

            var response = register_exchange_async(clientPtr, ref request, callback);
            Callback(response);

        }
        finally
        {
            Marshal.FreeHGlobal(exchangenamePtr);
            Marshal.FreeHGlobal(algorithmPtr);
            Marshal.FreeHGlobal(routingkeyPtr);
        }
        return tcs.Task;
    }
    public void UnRegisterQueue(string queuename)
    {
        IntPtr queuenamePtr = Marshal.StringToHGlobalAnsi(queuename);
        try
        {
            var response = unregister_queue(clientPtr, queuename);
            var responseWrapper = Marshal.PtrToStructure<UnRegisterQueueResponseWrapper>(response);
            var error = Marshal.PtrToStringAnsi(responseWrapper.error) ?? "Unknown error";
            var success = responseWrapper.success;
            free_unregister_queue_response(response);

            if (!success)
            {
                throw new ClientError(error);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(queuenamePtr);
        }
    }
    public async Task QueueMessage(string data, string queuename = "", string exchangename = "", string replyto = "", string routingkey = "", string correlation_id = "", bool striptoken = false, int expiration = 0)
    {
        var tcs = new TaskCompletionSource<string>();
        IntPtr dataPtr = Marshal.StringToHGlobalAnsi(data);
        IntPtr queuenamePtr = Marshal.StringToHGlobalAnsi(queuename);
        IntPtr exchangenamePtr = Marshal.StringToHGlobalAnsi(exchangename);
        IntPtr replytoPtr = Marshal.StringToHGlobalAnsi(replyto);
        IntPtr routingkeyPtr = Marshal.StringToHGlobalAnsi(routingkey);
        IntPtr correlation_idPtr = Marshal.StringToHGlobalAnsi(correlation_id);

        try
        {
            QueueMessageRequestWrapper request = new QueueMessageRequestWrapper
            {
                data = dataPtr,
                queuename = queuenamePtr,
                exchangename = exchangenamePtr,
                replyto = replytoPtr,
                routingkey = routingkeyPtr,
                correlation_id = correlation_idPtr,
                striptoken = striptoken,
                expiration = expiration
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

                    var response = Marshal.PtrToStructure<QueueMessageResponseWrapper>(responsePtr);
                    string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                    bool success = response.success;
                    free_queue_message_response(responsePtr);

                    if (!success)
                    {
                        tcs.SetException(new ClientError(error));
                    }
                    else
                    {
                        tcs.SetResult("ok");
                    }
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
            }

            var response = queue_message(clientPtr, ref request);
            Callback(response);
        }
        finally
        {
            Marshal.FreeHGlobal(dataPtr);
            Marshal.FreeHGlobal(queuenamePtr);
            Marshal.FreeHGlobal(exchangenamePtr);
            Marshal.FreeHGlobal(replytoPtr);
            Marshal.FreeHGlobal(routingkeyPtr);
            Marshal.FreeHGlobal(correlation_idPtr);
        }
        await tcs.Task;
    }


    private void _PushWorkitemCallback(IntPtr responsePtr)
    {
        this.trace("PushWorkitem callback to dotnet");
        try
        {
            var response = Marshal.PtrToStructure<PushWorkitemResponseWrapper>(responsePtr);

            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;
                var workitem = default(Workitem);
                if (success)
                {
                    var workitem_rsp = Marshal.PtrToStructure<WorkitemWrapper>(response.workitem);
                    var id = Marshal.PtrToStringAnsi(workitem_rsp.id) ?? string.Empty;
                    var name = Marshal.PtrToStringAnsi(workitem_rsp.name) ?? string.Empty;
                    var payload = Marshal.PtrToStringAnsi(workitem_rsp.payload) ?? string.Empty;
                    var wiq = Marshal.PtrToStringAnsi(workitem_rsp.wiq) ?? string.Empty;
                    var state = Marshal.PtrToStringAnsi(workitem_rsp.state) ?? string.Empty;
                    var lastrun = workitem_rsp.lastrun;
                    var nextrun = workitem_rsp.nextrun;
                    var priority = (int)workitem_rsp.priority;
                    var retries = (int)workitem_rsp.retries;
                    var username = Marshal.PtrToStringAnsi(workitem_rsp.username) ?? string.Empty;
                    var wiqid = Marshal.PtrToStringAnsi(workitem_rsp.wiqid) ?? string.Empty;
                    var success_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.success_wiqid) ?? string.Empty;
                    var failed_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiqid) ?? string.Empty;
                    var success_wiq = Marshal.PtrToStringAnsi(workitem_rsp.success_wiq) ?? string.Empty;
                    var failed_wiq = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiq) ?? string.Empty;
                    var errormessage = Marshal.PtrToStringAnsi(workitem_rsp.errormessage) ?? string.Empty;
                    var errorsource = Marshal.PtrToStringAnsi(workitem_rsp.errorsource) ?? string.Empty;
                    var errortype = Marshal.PtrToStringAnsi(workitem_rsp.errortype) ?? string.Empty;
                    workitem = new Workitem
                    {
                        id = id,
                        name = name,
                        payload = payload,
                        wiq = wiq,
                        state = state,
                        lastrun = lastrun,
                        nextrun = nextrun,
                        priority = priority,
                        retries = retries,
                        username = username,
                        wiqid = wiqid,
                        success_wiqid = success_wiqid,
                        failed_wiqid = failed_wiqid,
                        success_wiq = success_wiq,
                        failed_wiq = failed_wiq,
                        errormessage = errormessage,
                        errorsource = errorsource,
                        errortype = errortype,
                    };
                }

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, workitem);
                }
            }
        }
        catch (Exception ex)
        {
            // tcs.SetException(ex);
            Console.WriteLine(ex.Message);

        }
        finally
        {
            this.trace("Freeing memory");
            free_push_workitem_response(responsePtr);
            // Free each WorkitemFileWrapper and its associated strings


        }
    }

    public async Task<Workitem> PushWorkitem(string wiq, Workitem item, string[] files)
    {
        var tcs = new TaskCompletionSource<Workitem>();

        if (files == null) files = new string[] { };
        var _files_coll = new Collection<WorkitemFileWrapper>();

        // Fill the collection with WorkitemFileWrapper objects
        for (int i = 0; i < files.Length; i++)
        {
            _files_coll.Add(new WorkitemFileWrapper
            {
                filename = Marshal.StringToHGlobalAnsi(files[i]),
                id = Marshal.StringToHGlobalAnsi(""),
                compressed = false
            });
        }

        var _files = _files_coll.ToArray();
        var filePointers = new IntPtr[_files.Length];

        // Allocate memory for each WorkitemFileWrapper and store the pointer
        for (int i = 0; i < _files.Length; i++)
        {
            IntPtr structPtr = Marshal.AllocHGlobal(Marshal.SizeOf<WorkitemFileWrapper>());
            Marshal.StructureToPtr(_files[i], structPtr, false);
            filePointers[i] = structPtr; // Store the pointer
        }

        // Allocate memory for the array of pointers and copy the pointers into it
        IntPtr filesPtr = Marshal.AllocHGlobal(filePointers.Length * Marshal.SizeOf<IntPtr>());
        Marshal.Copy(filePointers, 0, filesPtr, filePointers.Length);

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);

            PushWorkitemRequestWrapper request = new PushWorkitemRequestWrapper
            {
                name = Marshal.StringToHGlobalAnsi(item.name),
                payload = Marshal.StringToHGlobalAnsi(item.payload),
                wiq = Marshal.StringToHGlobalAnsi(wiq),
                nextrun = item.nextrun,
                priority = item.priority,
                success_wiq = Marshal.StringToHGlobalAnsi(item.success_wiq),
                failed_wiq = Marshal.StringToHGlobalAnsi(item.failed_wiq),
                success_wiqid = Marshal.StringToHGlobalAnsi(item.success_wiqid),
                failed_wiqid = Marshal.StringToHGlobalAnsi(item.failed_wiqid),
                wiqid = Marshal.StringToHGlobalAnsi(item.wiqid),
                files = filesPtr,
                files_len = files.Length,
                request_id = requestId
            };

            // var callbackDelegate = new PushWorkitemCallback(Callback);
            CallbackRegistry.TryAddCallback(requestId, tcs);

            push_workitem_async(clientPtr, ref request, _PushWorkitemCallbackDelegate);
        }
        finally
        {
            // Marshal.FreeHGlobal(namePtr);
            // Marshal.FreeHGlobal(payloadPtr);
            // Marshal.FreeHGlobal(wiqPtr);
            for (int i = 0; i < _files.Length; i++)
            {
                // Get the pointer to the WorkitemFileWrapper
                IntPtr structPtr = Marshal.ReadIntPtr(filesPtr, i * Marshal.SizeOf<IntPtr>());

                // Retrieve the WorkitemFileWrapper from the unmanaged memory
                WorkitemFileWrapper wrapper = Marshal.PtrToStructure<WorkitemFileWrapper>(structPtr);

                // Free the unmanaged strings
                Marshal.FreeHGlobal(wrapper.filename);
                Marshal.FreeHGlobal(wrapper.id);

                // Free the WorkitemFileWrapper memory block
                Marshal.FreeHGlobal(structPtr);
            }

            // Free the array of pointers
            Marshal.FreeHGlobal(filesPtr);
        }
        return await tcs.Task;
    }
    private void _PopWorkitemCallback(IntPtr responsePtr)
    {
        try
        {
            var response = Marshal.PtrToStructure<PopWorkitemResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;
                var workitem = default(Workitem);
                if (success)
                {
                    if (response.workitem == IntPtr.Zero)
                    {
                        CallbackRegistry.TrySetResult(requestId, workitem);
                        return;
                    }
                    var workitem_rsp = Marshal.PtrToStructure<WorkitemWrapper>(response.workitem);
                    var id = Marshal.PtrToStringAnsi(workitem_rsp.id) ?? string.Empty;
                    var name = Marshal.PtrToStringAnsi(workitem_rsp.name) ?? string.Empty;
                    var payload = Marshal.PtrToStringAnsi(workitem_rsp.payload) ?? string.Empty;
                    var wiq = Marshal.PtrToStringAnsi(workitem_rsp.wiq) ?? string.Empty;
                    var state = Marshal.PtrToStringAnsi(workitem_rsp.state) ?? string.Empty;
                    var lastrun = workitem_rsp.lastrun;
                    var nextrun = workitem_rsp.nextrun;
                    var priority = (int)workitem_rsp.priority;
                    var retries = (int)workitem_rsp.retries;
                    var username = Marshal.PtrToStringAnsi(workitem_rsp.username) ?? string.Empty;
                    var wiqid = Marshal.PtrToStringAnsi(workitem_rsp.wiqid) ?? string.Empty;
                    var success_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.success_wiqid) ?? string.Empty;
                    var failed_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiqid) ?? string.Empty;
                    var success_wiq = Marshal.PtrToStringAnsi(workitem_rsp.success_wiq) ?? string.Empty;
                    var failed_wiq = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiq) ?? string.Empty;
                    var errormessage = Marshal.PtrToStringAnsi(workitem_rsp.errormessage) ?? string.Empty;
                    var errorsource = Marshal.PtrToStringAnsi(workitem_rsp.errorsource) ?? string.Empty;
                    var errortype = Marshal.PtrToStringAnsi(workitem_rsp.errortype) ?? string.Empty;
                    workitem = new Workitem
                    {
                        id = id,
                        name = name,
                        payload = payload,
                        wiq = wiq,
                        state = state,
                        lastrun = lastrun,
                        nextrun = nextrun,
                        priority = priority,
                        retries = retries,
                        username = username,
                        wiqid = wiqid,
                        success_wiqid = success_wiqid,
                        failed_wiqid = failed_wiqid,
                        success_wiq = success_wiq,
                        failed_wiq = failed_wiq,
                        errormessage = errormessage,
                        errorsource = errorsource,
                        errortype = errortype,
                    };
                }

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, workitem);
                }
            }
            else
            {
                Console.WriteLine($"No matching TCS found for request_id: {requestId}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            free_pop_workitem_response(responsePtr);
        }
    }

    public async Task<Workitem?> PopWorkitem(string wiq = "", string wiqid = "", string downloadfolder = ".")
    {
        IntPtr wiqPtr = Marshal.StringToCoTaskMemAnsi(wiq);
        IntPtr wiqidPtr = Marshal.StringToCoTaskMemAnsi(wiqid);

        var tcs = new TaskCompletionSource<Workitem?>();

        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            PopWorkitemRequestWrapper request = new PopWorkitemRequestWrapper
            {
                wiq = wiqPtr,
                wiqid = wiqidPtr,
                request_id = requestId
            };

            CallbackRegistry.TryAddCallback(requestId, tcs);
            pop_workitem_async(clientPtr, ref request, downloadfolder, _PopWorkitemCallbackDelegate);
        }
        finally
        {
            Marshal.FreeCoTaskMem(wiqPtr);
            Marshal.FreeCoTaskMem(wiqidPtr);
        }

        return await tcs.Task;
    }

    private void _UpdateWorkitemCallback(IntPtr responsePtr)
    {
        this.trace("UpdateWorkitem callback to dotnet");
        try
        {
            var response = Marshal.PtrToStructure<PopWorkitemResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<Workitem?>(requestId, out var tcs))
            {
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;
                var workitem = default(Workitem);
                if (success)
                {
                    var workitem_rsp = Marshal.PtrToStructure<WorkitemWrapper>(response.workitem);
                    var id = Marshal.PtrToStringAnsi(workitem_rsp.id) ?? string.Empty;
                    var name = Marshal.PtrToStringAnsi(workitem_rsp.name) ?? string.Empty;
                    var payload = Marshal.PtrToStringAnsi(workitem_rsp.payload) ?? string.Empty;
                    var wiq = Marshal.PtrToStringAnsi(workitem_rsp.wiq) ?? string.Empty;
                    var state = Marshal.PtrToStringAnsi(workitem_rsp.state) ?? string.Empty;
                    var lastrun = workitem_rsp.lastrun;
                    var nextrun = workitem_rsp.nextrun;
                    var priority = (int)workitem_rsp.priority;
                    var retries = (int)workitem_rsp.retries;
                    var username = Marshal.PtrToStringAnsi(workitem_rsp.username) ?? string.Empty;
                    var wiqid = Marshal.PtrToStringAnsi(workitem_rsp.wiqid) ?? string.Empty;
                    var success_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.success_wiqid) ?? string.Empty;
                    var failed_wiqid = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiqid) ?? string.Empty;
                    var success_wiq = Marshal.PtrToStringAnsi(workitem_rsp.success_wiq) ?? string.Empty;
                    var failed_wiq = Marshal.PtrToStringAnsi(workitem_rsp.failed_wiq) ?? string.Empty;
                    var errormessage = Marshal.PtrToStringAnsi(workitem_rsp.errormessage) ?? string.Empty;
                    var errorsource = Marshal.PtrToStringAnsi(workitem_rsp.errorsource) ?? string.Empty;
                    var errortype = Marshal.PtrToStringAnsi(workitem_rsp.errortype) ?? string.Empty;
                    workitem = new Workitem
                    {
                        id = id,
                        name = name,
                        payload = payload,
                        wiq = wiq,
                        state = state,
                        lastrun = lastrun,
                        nextrun = nextrun,
                        priority = priority,
                        retries = retries,
                        username = username,
                        wiqid = wiqid,
                        success_wiqid = success_wiqid,
                        failed_wiqid = failed_wiqid,
                        success_wiq = success_wiq,
                        failed_wiq = failed_wiq,
                        errormessage = errormessage,
                        errorsource = errorsource,
                        errortype = errortype,
                    };

                }

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, workitem);
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            this.trace("Freeing memory");
            free_update_workitem_response(responsePtr);
        }
    }
    public async Task<Workitem?> UpdateWorkitem(Workitem workitem, string[] files, bool ignoremaxretries = false)
    {
        var tcs = new TaskCompletionSource<Workitem?>();

        if (files == null) files = new string[] { };
        var _files_coll = new Collection<WorkitemFileWrapper>();

        // Fill the collection with WorkitemFileWrapper objects
        for (int i = 0; i < files.Length; i++)
        {
            _files_coll.Add(new WorkitemFileWrapper
            {
                filename = Marshal.StringToHGlobalAnsi(files[i]),
                id = Marshal.StringToHGlobalAnsi(""),
                compressed = false
            });
        }

        var _files = _files_coll.ToArray();
        var filePointers = new IntPtr[_files.Length];

        // Allocate memory for each WorkitemFileWrapper and store the pointer
        for (int i = 0; i < _files.Length; i++)
        {
            IntPtr structPtr = Marshal.AllocHGlobal(Marshal.SizeOf<WorkitemFileWrapper>());
            Marshal.StructureToPtr(_files[i], structPtr, false);
            filePointers[i] = structPtr; // Store the pointer
        }

        // Allocate memory for the array of pointers and copy the pointers into it
        IntPtr filesPtr = Marshal.AllocHGlobal(filePointers.Length * Marshal.SizeOf<IntPtr>());
        Marshal.Copy(filePointers, 0, filesPtr, filePointers.Length);
        GCHandle handle = default(GCHandle);
        try
        {
            var workitemwrapper = new WorkitemWrapper
            {
                id = Marshal.StringToHGlobalAnsi(workitem.id),
                name = Marshal.StringToHGlobalAnsi(workitem.name),
                payload = Marshal.StringToHGlobalAnsi(workitem.payload),
                wiq = Marshal.StringToHGlobalAnsi(workitem.wiq),
                state = Marshal.StringToHGlobalAnsi(workitem.state),
                lastrun = workitem.lastrun,
                nextrun = workitem.nextrun,
                priority = workitem.priority,
                retries = workitem.retries,
                username = Marshal.StringToHGlobalAnsi(workitem.username),
                wiqid = Marshal.StringToHGlobalAnsi(workitem.wiqid),
                success_wiqid = Marshal.StringToHGlobalAnsi(workitem.success_wiqid),
                failed_wiqid = Marshal.StringToHGlobalAnsi(workitem.failed_wiqid),
                success_wiq = Marshal.StringToHGlobalAnsi(workitem.success_wiq),
                failed_wiq = Marshal.StringToHGlobalAnsi(workitem.failed_wiq),
                errormessage = Marshal.StringToHGlobalAnsi(workitem.errormessage),
                errorsource = Marshal.StringToHGlobalAnsi(workitem.errorsource),
                errortype = Marshal.StringToHGlobalAnsi(workitem.errortype)
            };

            // Create a GCHandle to the workitemptr object
            handle = GCHandle.Alloc(workitemwrapper, GCHandleType.Pinned);

            // Get the IntPtr that points to the WorkitemWrapper object
            IntPtr workitemPtr = handle.AddrOfPinnedObject();

            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);

            UpdateWorkitemRequestWrapper request = new UpdateWorkitemRequestWrapper
            {
                workitem = workitemPtr,
                files = filesPtr,
                files_len = files.Length,
                ignoremaxretries = ignoremaxretries,
                request_id = requestId
            };

            CallbackRegistry.TryAddCallback(requestId, tcs);
            update_workitem_async(clientPtr, ref request, _UpdateWorkitemCallbackDelegate);
        }
        finally
        {
            for (int i = 0; i < _files.Length; i++)
            {
                // Get the pointer to the WorkitemFileWrapper
                IntPtr structPtr = Marshal.ReadIntPtr(filesPtr, i * Marshal.SizeOf<IntPtr>());

                // Retrieve the WorkitemFileWrapper from the unmanaged memory
                WorkitemFileWrapper wrapper = Marshal.PtrToStructure<WorkitemFileWrapper>(structPtr);

                // Free the unmanaged strings
                Marshal.FreeHGlobal(wrapper.filename);
                Marshal.FreeHGlobal(wrapper.id);

                // Free the WorkitemFileWrapper memory block
                Marshal.FreeHGlobal(structPtr);
            }

            // Free the array of pointers
            Marshal.FreeHGlobal(filesPtr);
            handle.Free();

        }
        return await tcs.Task;
    }


    private void _DeleteWorkitemCallback(IntPtr responsePtr)
    {
        this.trace("DeleteWorkitem callback to dotnet");
        try
        {

            var response = Marshal.PtrToStructure<DeleteWorkitemResponseWrapper>(responsePtr);
            int requestId = response.request_id;
            var count = CallbackRegistry.Count;
            if (count == 0)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
                return;
            }
            else if (count > 1)
            {
                Console.WriteLine($"Callback request_id: {requestId} and we have: {CallbackRegistry.Count} items in the registry");
            }
            if (CallbackRegistry.TryGetCallback<string>(requestId, out var tcs))
            {
                string error = Marshal.PtrToStringAnsi(response.error) ?? string.Empty;
                bool success = response.success;

                if (!success)
                {
                    CallbackRegistry.TrySetException<Workitem?>(requestId, new ClientError(error));
                }
                else
                {
                    CallbackRegistry.TrySetResult(requestId, "ok");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
        finally
        {
            this.trace("Freeing memory");
            free_delete_workitem_response(responsePtr);
        }
    }
    public async Task DeleteWorkitem(string id)
    {
        var tcs = new TaskCompletionSource<string>();
        try
        {
            int requestId = Interlocked.Increment(ref CallbackRegistryNextRequestId);
            DeleteWorkitemRequestWrapper request = new DeleteWorkitemRequestWrapper { id = id, request_id = requestId };
            CallbackRegistry.TryAddCallback(requestId, tcs);

            delete_workitem_async(clientPtr, ref request, _DeleteWorkitemCallbackDelegate);
        }
        finally
        {
        }
        await tcs.Task;
    }
    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }
    protected virtual void Dispose(bool disposing)
    {
        if (disposing)
        {
        }
        if (clientPtr != IntPtr.Zero)
        {
            free_client(clientPtr);
            clientPtr = IntPtr.Zero;
        }
    }
    ~Client()
    {
        Dispose(false);
    }
}

#region helper classes
public class WatchEvent
{
    public WatchEvent()
    {
        id = "";
        operation = "";
        document = "";
    }
    public string id { get; set; }
    public string operation { get; set; }
    public string document { get; set; }
}
public class ClientEvent
{
    public ClientEvent()
    {
        evt = "";
        reason = "";
    }
    public string evt { get; set; }
    public string reason { get; set; }
}
public class QueueEvent
{
    public QueueEvent()
    {
        queuename = "";
        correlation_id = "";
        replyto = "";
        routingkey = "";
        exchangename = "";
        data = "";
    }
    public string queuename;
    public string correlation_id;
    public string replyto;
    public string routingkey;
    public string exchangename;
    public string data;
}
public class WorkitemFile
{
    public WorkitemFile()
    {
        filename = "";
        id = "";
        compressed = false;
    }
    public string filename;
    public string id;
    public bool compressed;
}
public class Workitem
{
    public Workitem()
    {
        id = "";
        name = "";
        payload = "";
        priority = 0;
        nextrun = 0;
        lastrun = 0;
        files = new WorkitemFile[0];
        state = "";
        wiq = "";
        wiqid = "";
        retries = 0;
        username = "";
        success_wiqid = "";
        failed_wiqid = "";
        success_wiq = "";
        failed_wiq = "";
        errormessage = "";
        errorsource = "";
        errortype = "";
    }
    public string id;
    public string name;
    public string payload;
    public int priority;
    public ulong nextrun;
    public ulong lastrun;
    public WorkitemFile[] files;
    public string state;
    public string wiq;
    public string wiqid;
    public int retries;
    public string username;
    public string success_wiqid;
    public string failed_wiqid;
    public string success_wiq;
    public string failed_wiq;
    public string errormessage;
    public string errorsource;
    public string errortype;
}
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
#endregion
