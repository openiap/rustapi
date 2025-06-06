const koffi = require('koffi');
const path = require('path');
const fs = require('fs');
const { log } = require('console');

const CString = 'char*';
const voidPtr = koffi.pointer('void');
const bool = koffi.types.bool;
const int = 'int';
const uint64 = 'uint64_t';

function encodeStruct(value, type) {
    const buf = Buffer.allocUnsafe(koffi.sizeof(type));
    koffi.encode(buf, type, value);
    return buf;
}
function encode_files(req) {
    for (let i = 0; i < req.files.length; i++) {
        if (req.files[i] == null) continue;
        if (typeof req.files[i] === 'string') {
            req.files[i] = {
                filename: req.files[i],
                id: "",
            }
        }
        req.files[i] = encodeStruct(req.files[i], WorkitemFileWrapper);
    }
    if (req.files.length == 0 || req.files.at(-1) != null) {
        req.files.push(null); // terminate array
    }
    req.files_len = req.files.length - 1;
}
function decode_files(res) {
    let _files = [];
    if (res.files_len > 0) {
        var files = koffi.decode(res.files, 'WorkitemFileWrapper ***', res.files_len);
        for (let i = 0; i < res.files_len; i++) {
            let file = files[i];
            if (file != null) {
                var _file = koffi.decode(file, WorkitemFileWrapper);
                delete _file.compressed;
                delete _file.file;
                _files.push(_file);
            }
        }
    }
    res.files = _files;

}

const ClientWrapper = koffi.struct('ClientWrapper', {
    success: 'bool',
    error: 'char*'
});
const ClientWrapperPtr = koffi.pointer(ClientWrapper);
const ConnectResponseWrapper = koffi.struct('ConnectResponseWrapper', {
    success: 'bool',
    error: 'char*'
});
const ConnectResponseWrapperPtr = koffi.pointer(ConnectResponseWrapper);


const ClientEventResponseWrapper = koffi.struct('ClientEventResponseWrapper', {
    success: 'bool',
    eventid: 'char*',
    error: 'char*'
});
const ClientEventResponseWrapperPtr = koffi.pointer(ClientEventResponseWrapper);
const ClientEventWrapper = koffi.struct('ClientEventWrapper', {
    event: 'char*',
    reason: 'char*'
});
const ClientEventWrapperPtr = koffi.pointer(ClientEventWrapper);
const OffClientEventResponseWrapper = koffi.struct('OffClientEventResponseWrapper', {
    success: 'bool',
    error: 'char*'
});
const OffClientEventResponseWrapperPtr = koffi.pointer(OffClientEventResponseWrapper);

const SigninRequestWrapper = koffi.struct('SigninRequestWrapper', {
    username: CString,
    password: CString,
    jwt: CString,
    agent: CString,
    version: CString,
    longtoken: bool,
    validateonly: bool,
    ping: bool,
});
const SigninRequestWrapperPtr = koffi.pointer(SigninRequestWrapper);
const SigninResponseWrapper = koffi.struct('SigninResponseWrapper', {
    success: bool,
    jwt: CString,
    error: CString
});
const SigninResponseWrapperPtr = koffi.pointer(SigninResponseWrapper);

const ListCollectionsResponseWrapper = koffi.struct('ListCollectionsResponseWrapper', {
    success: bool,
    collections: CString,
    error: CString
});
const ListCollectionsResponseWrapperPtr = koffi.pointer(ListCollectionsResponseWrapper);

const ColCollationWrapper = koffi.struct('ColCollationWrapper', {
    locale: CString,
    case_level: bool,
    case_first: CString,
    strength: int,
    numeric_ordering: bool,
    alternate: CString,
    max_variable: CString,
    backwards: bool,
});
const ColCollationWrapperPtr = koffi.pointer(ColCollationWrapper);
const ColTimeseriesWrapper = koffi.struct('ColTimeseriesWrapper', {
    time_field: CString,
    meta_field: CString,
    granularity: CString
});
const ColTimeseriesWrapperPtr = koffi.pointer(ColTimeseriesWrapper);
const CreateCollectionRequestWrapper = koffi.struct('CreateCollectionRequestWrapper', {
    collectionname: CString,
    collation: ColCollationWrapperPtr,
    timeseries: ColTimeseriesWrapperPtr,
    expire_after_seconds: int,
    change_stream_pre_and_post_images: bool,
    capped: bool,
    max: int,
    size: int,
});
const CreateCollectionRequestWrapperPtr = koffi.pointer(CreateCollectionRequestWrapper);
const CreateCollectionResponseWrapper = koffi.struct('CreateCollectionResponseWrapper', {
    success: bool,
    error: CString
});
const CreateCollectionResponseWrapperPtr = koffi.pointer(CreateCollectionResponseWrapper);

const DropCollectionResponseWrapper = koffi.struct('DropCollectionResponseWrapper', {
    success: bool,
    error: CString
});
const DropCollectionResponseWrapperPtr = koffi.pointer(DropCollectionResponseWrapper);

const GetIndexesResponseWrapper = koffi.struct('GetIndexesResponseWrapper', {
    success: bool,
    indexes: CString,
    error: CString
});
const GetIndexesResponseWrapperPtr = koffi.pointer(GetIndexesResponseWrapper);

const CreateIndexRequestWrapper = koffi.struct('CreateIndexRequestWrapper', {
    collectionname: CString,
    index: CString,
    options: CString,
    name: CString,
    request_id: int,
});
const CreateIndexRequestWrapperPtr = koffi.pointer(CreateIndexRequestWrapper);
const CreateIndexResponseWrapper = koffi.struct('CreateIndexResponseWrapper', {
    success: bool,
    error: CString,
    request_id: int,
});
const CreateIndexResponseWrapperPtr = koffi.pointer(CreateIndexResponseWrapper);

const DropIndexResponseWrapper = koffi.struct('DropIndexResponseWrapper', {
    success: bool,
    error: CString
});
const DropIndexResponseWrapperPtr = koffi.pointer(DropIndexResponseWrapper);

const QueryRequestWrapper = koffi.struct('QueryRequestWrapper', {
    collectionname: CString,
    query: CString,
    projection: CString,
    orderby: CString,
    queryas: CString,
    explain: bool,
    skip: int,
    top: int,
});
const QueryRequestWrapperPtr = koffi.pointer(QueryRequestWrapper);
const QueryResponseWrapper = koffi.struct('QueryResponseWrapper', {
    success: bool,
    results: CString,
    error: CString
});
const QueryResponseWrapperPtr = koffi.pointer(QueryResponseWrapper);

const CustomCommandRequestWrapper = koffi.struct('CustomCommandRequestWrapper', {
    command: CString,
    id: CString,
    name: CString,
    data: CString,
    request_id: int,
});
const CustomCommandRequestWrapperPtr = koffi.pointer(CustomCommandRequestWrapper);
const CustomCommandResponseWrapper = koffi.struct('CustomCommandResponseWrapper', {
    success: bool,
    result: CString,
    error: CString,
    request_id: int,
});
const CustomCommandResponseWrapperPtr = koffi.pointer(CustomCommandResponseWrapper);

const AggregateRequestWrapper = koffi.struct('AggregateRequestWrapper', {
    collectionname: CString,
    aggregates: CString,
    queryas: CString,
    hint: CString,
    explain: bool,
});
const AggregateRequestWrapperPtr = koffi.pointer(AggregateRequestWrapper);
const AggregateResponseWrapper = koffi.struct('AggregateResponseWrapper', {
    success: bool,
    results: CString,
    error: CString
});
const AggregateResponseWrapperPtr = koffi.pointer(AggregateResponseWrapper);

const CountRequestWrapper = koffi.struct('CountRequestWrapper', {
    collectionname: CString,
    query: CString,
    queryas: CString,
    explain: bool,
});
const CountRequestWrapperPtr = koffi.pointer(CountRequestWrapper);
const CountResponseWrapper = koffi.struct('CountResponseWrapper', {
    success: bool,
    result: int,
    error: CString
});
const CountResponseWrapperPtr = koffi.pointer(CountResponseWrapper);

const DistinctRequestWrapper = koffi.struct('DistinctRequestWrapper', {
    collectionname: CString,
    field: CString,
    query: CString,
    queryas: CString,
    explain: bool,
});
const DistinctRequestWrapperPtr = koffi.pointer(DistinctRequestWrapper);
const DistinctResponseWrapper = koffi.struct('DistinctResponseWrapper', {
    success: bool,
    results: 'char **',
    error: CString,
    results_len: int,
});
const DistinctResponseWrapperPtr = koffi.pointer(DistinctResponseWrapper);

const InsertOneRequestWrapper = koffi.struct('InsertOneRequestWrapper', {
    collectionname: CString,
    item: CString,
    w: int,
    j: bool,
});
const InsertOneRequestWrapperPtr = koffi.pointer(InsertOneRequestWrapper);
const InsertOneResponseWrapper = koffi.struct('InsertOneResponseWrapper', {
    success: bool,
    result: CString,
    error: CString
});
const InsertOneResponseWrapperPtr = koffi.pointer(InsertOneResponseWrapper);

const InsertManyRequestWrapper = koffi.struct('InsertManyRequestWrapper', {
    collectionname: CString,
    items: CString,
    w: int,
    j: bool,
    skipresults: bool,
});
const InsertManyRequestWrapperPtr = koffi.pointer(InsertManyRequestWrapper);
const InsertManyResponseWrapper = koffi.struct('InsertManyResponseWrapper', {
    success: bool,
    results: CString,
    error: CString
});
const InsertManyResponseWrapperPtr = koffi.pointer(InsertManyResponseWrapper);

const UpdateOneRequestWrapper = koffi.struct('UpdateOneRequestWrapper', {
    collectionname: CString,
    item: CString,
    w: int,
    j: bool,
});
const UpdateOneRequestWrapperPtr = koffi.pointer(UpdateOneRequestWrapper);
const UpdateOneResponseWrapper = koffi.struct('UpdateOneResponseWrapper', {
    success: bool,
    result: CString,
    error: CString
});
const UpdateOneResponseWrapperPtr = koffi.pointer(UpdateOneResponseWrapper);

const InsertOrUpdateOneRequestWrapper = koffi.struct('InsertOrUpdateOneRequestWrapper', {
    collectionname: CString,
    uniqeness: CString,
    item: CString,
    w: int,
    j: bool,
});
const InsertOrUpdateOneRequestWrapperPtr = koffi.pointer(InsertOrUpdateOneRequestWrapper);
const InsertOrUpdateOneResponseWrapper = koffi.struct('InsertOrUpdateOneResponseWrapper', {
    success: bool,
    result: CString,
    error: CString
});
const InsertOrUpdateOneResponseWrapperPtr = koffi.pointer(InsertOrUpdateOneResponseWrapper);

const DeleteOneRequestWrapper = koffi.struct('DeleteOneRequestWrapper', {
    collectionname: CString,
    id: CString,
    recursive: bool,
});
const DeleteOneRequestWrapperPtr = koffi.pointer(DeleteOneRequestWrapper);
const DeleteOneResponseWrapper = koffi.struct('DeleteOneResponseWrapper', {
    success: bool,
    affectedrows: int,
    error: CString
});
const DeleteOneResponseWrapperPtr = koffi.pointer(DeleteOneResponseWrapper);

const DeleteManyRequestWrapper = koffi.struct('DeleteManyRequestWrapper', {
    collectionname: CString,
    query: CString,
    recursive: bool,
    ids: 'char **',
});
const DeleteManyRequestWrapperPtr = koffi.pointer(DeleteManyRequestWrapper);
const DeleteManyResponseWrapper = koffi.struct('DeleteManyResponseWrapper', {
    success: bool,
    affectedrows: int,
    error: CString
});
const DeleteManyResponseWrapperPtr = koffi.pointer(DeleteManyResponseWrapper);

const DownloadRequestWrapper = koffi.struct('DownloadRequestWrapper', {
    collectionname: CString,
    id: CString,
    folder: CString,
    filename: CString
});
const DownloadRequestWrapperPtr = koffi.pointer(DownloadRequestWrapper);
const DownloadResponseWrapper = koffi.struct('DownloadResponseWrapper', {
    success: bool,
    filename: CString,
    error: CString
});
const DownloadResponseWrapperPtr = koffi.pointer(DownloadResponseWrapper);

const UploadRequestWrapper = koffi.struct('UploadRequestWrapper', {
    filepath: CString,
    filename: CString,
    mimetype: CString,
    metadata: CString,
    collectionname: CString
});
const UploadRequestWrapperPtr = koffi.pointer(UploadRequestWrapper);
const UploadResponseWrapper = koffi.struct('UploadResponseWrapper', {
    success: bool,
    id: CString,
    error: CString
});
const UploadResponseWrapperPtr = koffi.pointer(UploadResponseWrapper);

const WorkitemFileWrapper = koffi.struct('WorkitemFileWrapper', {
    filename: CString,
    id: CString,
    compressed: bool,
    file: 'uint8_t *',
});

const WorkitemWrapper = koffi.struct('WorkitemWrapper', {
    id: CString,
    name: CString,
    payload: CString,
    priority: int,
    nextrun: uint64,
    lastrun: uint64,
    files: 'WorkitemFileWrapper ***',
    files_len: int,
    state: CString,
    wiq: CString,
    wiqid: CString,
    retries: int,
    username: CString,
    success_wiqid: CString,
    failed_wiqid: CString,
    success_wiq: CString,
    failed_wiq: CString,
    errormessage: CString,
    errorsource: CString,
    errortype: CString
});
const WorkitemWrapperPtr = koffi.pointer(WorkitemWrapper);
const PushWorkitemRequestWrapper = koffi.struct('PushWorkitemRequestWrapper', {
    wiq: CString,
    wiqid: CString,
    name: CString,
    payload: CString,
    nextrun: uint64,
    success_wiqid: CString,
    failed_wiqid: CString,
    success_wiq: CString,
    failed_wiq: CString,
    priority: int,
    files: 'WorkitemFileWrapper ***',
    files_len: int,
});
const PushWorkitemRequestWrapperPtr = koffi.pointer(PushWorkitemRequestWrapper);
const PushWorkitemResponseWrapper = koffi.struct('PushWorkitemResponseWrapper', {
    success: bool,
    error: CString,
    workitem: WorkitemWrapperPtr
});
const PushWorkitemResponseWrapperPtr = koffi.pointer(PushWorkitemResponseWrapper);

const PopWorkitemRequestWrapper = koffi.struct('PopWorkitemRequestWrapper', {
    wiq: CString,
    wiqid: CString,
    request_id: int,
});
const PopWorkitemRequestWrapperPtr = koffi.pointer(PopWorkitemRequestWrapper);
const PopWorkitemResponseWrapper = koffi.struct('PopWorkitemResponseWrapper', {
    success: bool,
    error: CString,
    workitem: WorkitemWrapperPtr,
    request_id: int,
});
const PopWorkitemResponseWrapperPtr = koffi.pointer(PopWorkitemResponseWrapper);


const UpdateWorkitemRequestWrapper = koffi.struct('UpdateWorkitemRequestWrapper', {
    workitem: WorkitemWrapperPtr,
    ignoremaxretries: bool,
    files: 'WorkitemFileWrapper ***',
    files_len: int,
});
const UpdateWorkitemRequestWrapperPtr = koffi.pointer(UpdateWorkitemRequestWrapper);
const UpdateWorkitemResponseWrapper = koffi.struct('UpdateWorkitemResponseWrapper', {
    success: bool,
    error: CString,
    workitem: WorkitemWrapperPtr
});
const UpdateWorkitemResponseWrapperPtr = koffi.pointer(UpdateWorkitemResponseWrapper);

const DeleteWorkitemRequestWrapper = koffi.struct('DeleteWorkitemRequestWrapper', {
    id: CString,
});
const DeleteWorkitemRequestWrapperPtr = koffi.pointer(DeleteWorkitemRequestWrapper);
const DeleteWorkitemResponseWrapper = koffi.struct('DeleteWorkitemResponseWrapper', {
    success: bool,
    error: CString
});
const DeleteWorkitemResponseWrapperPtr = koffi.pointer(DeleteWorkitemResponseWrapper);

const WatchRequestWrapper = koffi.struct('WatchRequestWrapper', {
    collectionname: CString,
    paths: CString,
});
const WatchRequestWrapperPtr = koffi.pointer(WatchRequestWrapper);
const WatchResponseWrapper = koffi.struct('WatchResponseWrapper', {
    success: bool,
    watchid: CString,
    error: CString,
});
const WatchResponseWrapperPtr = koffi.pointer(WatchResponseWrapper);
const WatchEventWrapper = koffi.struct('WatchEventWrapper', {
    id: CString,
    operation: CString,
    document: CString,
});
const WatchEventWrapperPtr = koffi.pointer(WatchEventWrapper);

const UnwatchResponseWrapper = koffi.struct('UnwatchResponseWrapper', {
    success: bool,
    error: CString
});
const UnwatchResponseWrapperPtr = koffi.pointer(UnwatchResponseWrapper);

const QueueEventWrapper = koffi.struct('QueueEventWrapper', {
    queuename: CString,
    correlation_id: CString,
    replyto: CString,
    routingkey: CString,
    exchangename: CString,
    data: CString,
});
const RegisterQueueRequestWrapper = koffi.struct('RegisterQueueRequestWrapper', {
    queuename: CString
});
const RegisterQueueRequestWrapperPtr = koffi.pointer(RegisterQueueRequestWrapper);
const RegisterQueueResponseWrapper = koffi.struct('RegisterQueueResponseWrapper', {
    success: bool,
    queuename: CString,
    error: CString
});
const RegisterQueueResponseWrapperPtr = koffi.pointer(RegisterQueueResponseWrapper);

const RegisterExchangeRequestWrapper = koffi.struct('RegisterExchangeRequestWrapper', {
    exchangename: CString,
    algorithm: CString,
    routingkey: CString,
    addqueue: bool,
});
const RegisterExchangeRequestWrapperPtr = koffi.pointer(RegisterExchangeRequestWrapper);
const RegisterExchangeResponseWrapper = koffi.struct('RegisterExchangeResponseWrapper', {
    success: bool,
    queuename: CString,
    error: CString
});
const RegisterExchangeResponseWrapperPtr = koffi.pointer(RegisterExchangeResponseWrapper);

const UnRegisterQueueResponseWrapper = koffi.struct('UnRegisterQueueResponseWrapper', {
    success: bool,
    error: CString
});
const UnRegisterQueueResponseWrapperPtr = koffi.pointer(UnRegisterQueueResponseWrapper);

const QueueEvent = koffi.struct({
    queuename: CString,
    correlation_id: CString,
    replyto: CString,
    routingkey: CString,
    exchangename: CString,
    data: CString,
});
const QueueEventPtr = koffi.pointer(QueueEvent);

const QueueMessageRequestWrapper = koffi.struct('QueueMessageRequestWrapper', {
    queuename: CString,
    correlation_id: CString,
    replyto: CString,
    routingkey: CString,
    exchangename: CString,
    data: CString,
    striptoken: bool,
    expiration: int,
});
const QueueMessageRequestWrapperPtr = koffi.pointer(QueueMessageRequestWrapper);
const QueueMessageResponseWrapper = koffi.struct('QueueMessageResponseWrapper', {
    success: bool,
    error: CString
});
const QueueMessageResponseWrapperPtr = koffi.pointer(QueueMessageResponseWrapper);

const RpcResponseWrapper = koffi.struct('RpcResponseWrapper', {
    success: bool,
    result: CString,
    error: CString
});
const RpcResponseWrapperPtr  = koffi.pointer(RpcResponseWrapper);


const InvokeOpenRPARequestWrapper = koffi.struct('InvokeOpenRPARequestWrapper', {
    robotid: CString,
    workflowid: CString,
    payload: CString,
    rpc: bool,
    request_id: int,
});
const InvokeOpenRPARequestWrapperPtr = koffi.pointer(InvokeOpenRPARequestWrapper);
const InvokeOpenRPAResponseWrapper = koffi.struct('InvokeOpenRPAResponseWrapper', {
    success: bool,
    result: CString,
    error: CString,
    request_id: int,
});
const InvokeOpenRPAResponseWrapperPtr = koffi.pointer(InvokeOpenRPAResponseWrapper);

function isMusl() {
    // For Node 10
    if (!process.report || typeof process.report.getReport !== 'function') {
        try {
            const lddPath = require('child_process').execSync('which ldd').toString().trim()
            return readFileSync(lddPath, 'utf8').includes('musl')
        } catch (e) {
            return true
        }
    } else {
        const { glibcVersionRuntime } = process.report.getReport().header
        return !glibcVersionRuntime
    }
}

// Function to load the correct library file based on the operating system
function loadLibrary() {
    const { platform, arch } = process
    let libDir = path.join(__dirname, 'lib');
    let libPath;
    // console.log(`Platform: ${platform}, Arch: ${arch}`);
    switch (platform) {
        case 'android':
            switch (arch) {
                case 'arm64':
                    libPath = path.join(libDir, 'libopeniap-android-arm64.so'); break;
                case 'arm':
                    libPath = path.join(libDir, 'libopeniap-android-arm-eabi.so'); break;
                default:
                    throw new Error(`Unsupported architecture on Android ${arch}`)
            }
            break;
        case 'win32':
            switch (arch) {
                case 'x64':
                    libPath = path.join(libDir, 'openiap-windows-x64.dll'); break;
                case 'x86':
                    libPath = path.join(libDir, 'openiap-windows-i686.dll'); break;
                case 'arm64':
                    libPath = path.join(libDir, 'openiap-windows-arm64.dll'); break;
                default:
                    throw new Error(`Unsupported architecture on win32 ${arch}`)
            }
            break;
        case 'darwin':
            switch (arch) {
                case 'x64':
                    libPath = path.join(libDir, 'libopeniap-macos-x64.dylib'); break;
                case 'arm64':
                    libPath = path.join(libDir, 'libopeniap-macos-arm64.dylib'); break;
                default:
                    throw new Error(`Unsupported architecture on darwin ${arch}`)
            }
            break;
        case 'freebsd':
            switch (arch) {
                case 'x64':
                    libPath = path.join(libDir, 'libopeniap-freebsd-x64.so'); break;
                default:
                    throw new Error(`Unsupported architecture on freebsd ${arch}`)
            }
            break;
        case 'linux':
            switch (arch) {
                case 'x64':
                    if (isMusl()) {
                        libPath = path.join(libDir, 'libopeniap-linux-musl-x64.a'); break;
                    } else {
                        libPath = path.join(libDir, 'libopeniap-linux-x64.so'); break;
                    }
                case 'arm64':
                    if (isMusl()) {
                        libPath = path.join(libDir, 'libopeniap-linux-musl-arm64.a'); break;
                    } else {
                        libPath = path.join(libDir, 'libopeniap-linux-arm64.so'); break;
                    }
                default:
                    throw new Error(`Unsupported architecture on linux ${arch}`)
            }
            break;
        default:
            throw new Error(`Unsupported platform ${platform}`)
    }
    if (!fs.existsSync(libPath)) {
        libDir = path.join(__dirname, '../target/debug/');
        switch (process.platform) {
            case 'win32':
                libPath = path.join(libDir, 'openiap_clib.dll');
                break;
            case 'darwin':
                libPath = path.join(libDir, 'libopeniap_clib.dylib');
                break;
            default:
                libPath = path.join(libDir, 'libopeniap_clib.so');
                break;
        }
    }
    // console.log(`Using library: ${libPath}`);

    try {
        const lib = koffi.load(libPath);

        lib.enable_tracing = lib.func('void enable_tracing(const char* rust_log, const char* tracing)');

        lib.error = lib.func('void error(const char* message)');
        lib.info = lib.func('void info(const char* message)');
        lib.warn = lib.func('void warn(const char* message)');
        lib.debug = lib.func('void debug(const char* message)');
        lib.trace = lib.func('void trace(const char* message)');

        lib.set_f64_observable_gauge = lib.func('void set_f64_observable_gauge(const char* name, double value, const char* description);');
        lib.set_u64_observable_gauge = lib.func('void set_u64_observable_gauge(const char* name, uint64_t value, const char* description);');
        lib.set_i64_observable_gauge = lib.func('void set_i64_observable_gauge(const char* name, int64_t value, const char* description);');
        lib.disable_observable_gauge = lib.func('void disable_observable_gauge(const char* name);');

        lib.create_client = lib.func('create_client', ClientWrapperPtr, []);
        lib.on_client_event = lib.func('on_client_event', ClientEventResponseWrapperPtr, [ClientWrapperPtr]);
        lib.next_client_event = lib.func('next_client_event', ClientEventWrapperPtr, [CString]);
        lib.off_client_event = lib.func('off_client_event', OffClientEventResponseWrapperPtr, [CString]);
        lib.free_off_event_response = lib.func('free_off_event_response', 'void', [OffClientEventResponseWrapperPtr]);
        lib.free_event_response = lib.func('free_event_response', 'void', [ClientEventResponseWrapperPtr]);
        lib.free_client_event = lib.func('free_client_event', 'void', [ClientEventWrapperPtr]);

        lib.set_agent_name = lib.func('client_set_agent_name', 'void', [ClientWrapperPtr, 'str']);
        lib.client_set_default_timeout = lib.func('client_set_default_timeout', 'void', [ClientWrapperPtr, 'int']);
        lib.client_get_default_timeout = lib.func('client_get_default_timeout', 'int', [ClientWrapperPtr]);
        lib.client_get_state = lib.func('client_get_state', 'str', [ClientWrapperPtr]);
        lib.set_agent_version = lib.func('client_set_agent_version', 'void', [ClientWrapperPtr, 'str']);

        lib.disable_tracing = lib.func('void disable_tracing()');
        lib.connect = lib.func('client_connect', ConnectResponseWrapperPtr, [ClientWrapperPtr, 'str']);
        lib.ConnectCallback = koffi.proto('void ConnectCallback(ConnectResponseWrapper*)');
        lib.connect_async = lib.func('connect_async', 'void', [ClientWrapperPtr, 'str', 'int', koffi.pointer(lib.ConnectCallback)]);
        lib.free_client = lib.func('void free_client(ClientWrapper*)');
        lib.signin = lib.func('signin', SigninResponseWrapperPtr, [ClientWrapperPtr, SigninRequestWrapperPtr]);
        lib.signinCallback = koffi.proto('void signinCallback(SigninResponseWrapper*)');
        lib.signin_async = lib.func('signin_async', 'void', [ClientWrapperPtr, SigninRequestWrapperPtr, koffi.pointer(lib.signinCallback)]);
        lib.free_signin_response = lib.func('free_signin_response', 'void', [SigninResponseWrapperPtr]);

        lib.disconnect = lib.func('client_disconnect', 'void', [ClientWrapperPtr]);

        lib.list_collections = lib.func('list_collections', ListCollectionsResponseWrapperPtr, [ClientWrapperPtr, 'bool']);
        lib.listCollectionsCallback = koffi.proto('void ListCollectionsCallback(ListCollectionsResponseWrapper*)');
        lib.list_collections_async = lib.func('list_collections_async', 'void', [ClientWrapperPtr, 'bool', 'int', koffi.pointer(lib.listCollectionsCallback)]);
        lib.free_list_collections_response = lib.func('free_list_collections_response', 'void', [ListCollectionsResponseWrapperPtr]);

        lib.create_collection = lib.func('create_collection', CreateCollectionResponseWrapperPtr, [ClientWrapperPtr, CreateCollectionRequestWrapperPtr]);
        lib.create_collectionCallback = koffi.proto('void create_collectionCallback(CreateCollectionResponseWrapper*)');
        lib.create_collection_async = lib.func('create_collection_async', 'void', [ClientWrapperPtr, CreateCollectionRequestWrapperPtr, koffi.pointer(lib.create_collectionCallback)]);
        lib.free_create_collection_response = lib.func('free_create_collection_response', 'void', [CreateCollectionResponseWrapperPtr]);

        lib.drop_collection = lib.func('drop_collection', DropCollectionResponseWrapperPtr, [ClientWrapperPtr, CString]);
        lib.dropCollectionCallback = koffi.proto('void dropCollectionCallback(DropCollectionResponseWrapper*)');
        lib.drop_collection_async = lib.func('drop_collection_async', 'void', [ClientWrapperPtr, CString, 'int', koffi.pointer(lib.dropCollectionCallback)]);
        lib.free_drop_collection_response = lib.func('free_drop_collection_response', 'void', [DropCollectionResponseWrapperPtr]);

        lib.get_indexes = lib.func('get_indexes', GetIndexesResponseWrapperPtr, [ClientWrapperPtr, CString]);
        lib.get_indexesCallback = koffi.proto('void get_indexesCallback(GetIndexesResponseWrapper*)');
        lib.get_indexes_async = lib.func('get_indexes_async', 'void', [ClientWrapperPtr, CString, 'int', koffi.pointer(lib.get_indexesCallback)]);
        lib.free_get_indexes_response = lib.func('free_get_indexes_response', 'void', [GetIndexesResponseWrapperPtr]);

        lib.create_index = lib.func('create_index', CreateIndexResponseWrapperPtr, [ClientWrapperPtr, CreateIndexRequestWrapperPtr]);
        lib.create_indexCallback = koffi.proto('void create_indexCallback(CreateIndexResponseWrapper*)');
        lib.create_index_async = lib.func('create_index_async', 'void', [ClientWrapperPtr, CreateIndexRequestWrapperPtr, koffi.pointer(lib.create_indexCallback)]);
        lib.free_create_index_response = lib.func('free_create_index_response', 'void', [CreateIndexResponseWrapperPtr]);

        lib.drop_index = lib.func('drop_index', DropIndexResponseWrapperPtr, [ClientWrapperPtr, CString, CString]);
        lib.drop_indexCallback = koffi.proto('void drop_indexCallback(DropIndexResponseWrapper*)');
        lib.drop_index_async = lib.func('drop_index_async', 'void', [ClientWrapperPtr, CString, CString, 'int', koffi.pointer(lib.drop_indexCallback)]);
        lib.free_drop_index_response = lib.func('free_drop_index_response', 'void', [DropIndexResponseWrapperPtr]);

        lib.custom_command = lib.func('custom_command', CustomCommandResponseWrapperPtr, [ClientWrapperPtr, CustomCommandRequestWrapperPtr, 'int']);
        lib.custom_commandCallback = koffi.proto('void CustomCommandCallback(CustomCommandResponseWrapper*)');
        lib.custom_command_async = lib.func('custom_command_async', 'void', [ClientWrapperPtr, CustomCommandRequestWrapperPtr, koffi.pointer(lib.custom_commandCallback), 'int']);
        lib.free_custom_command_response = lib.func('free_custom_command_response', 'void', [CustomCommandResponseWrapperPtr]);

        lib.query = lib.func('query', QueryResponseWrapperPtr, [ClientWrapperPtr, QueryRequestWrapperPtr]);
        lib.queryCallback = koffi.proto('void queryCallback(QueryResponseWrapper*)');
        lib.query_async = lib.func('query_async', 'void', [ClientWrapperPtr, QueryRequestWrapperPtr, koffi.pointer(lib.queryCallback)]);
        lib.free_query_response = lib.func('free_query_response', 'void', [QueryResponseWrapperPtr]);
        lib.count = lib.func('count', CountResponseWrapperPtr, [ClientWrapperPtr, CountRequestWrapperPtr]);
        lib.countCallback = koffi.proto('void countCallback(CountResponseWrapper*)');
        lib.count_async = lib.func('count_async', 'void', [ClientWrapperPtr, CountRequestWrapperPtr, koffi.pointer(lib.countCallback)]);
        lib.free_count_response = lib.func('free_count_response', 'void', [CountResponseWrapperPtr]);
        lib.distinct = lib.func('distinct', DistinctResponseWrapperPtr, [ClientWrapperPtr, DistinctRequestWrapperPtr]);
        lib.distinctCallback = koffi.proto('void distinctCallback(DistinctResponseWrapper*)');
        lib.distinct_async = lib.func('distinct_async', 'void', [ClientWrapperPtr, DistinctRequestWrapperPtr, koffi.pointer(lib.distinctCallback)]);
        lib.free_distinct_response = lib.func('free_distinct_response', 'void', [DistinctResponseWrapperPtr]);
        lib.aggregate = lib.func('aggregate', AggregateResponseWrapperPtr, [ClientWrapperPtr, AggregateRequestWrapperPtr]);
        lib.aggregateCallback = koffi.proto('void aggregateCallback(AggregateResponseWrapper*)');
        lib.aggregate_async = lib.func('aggregate_async', 'void', [ClientWrapperPtr, AggregateRequestWrapperPtr, koffi.pointer(lib.aggregateCallback)]);
        lib.free_aggregate_response = lib.func('free_aggregate_response', 'void', [AggregateResponseWrapperPtr]);
        lib.insert_one = lib.func('insert_one', InsertOneResponseWrapperPtr, [ClientWrapperPtr, InsertOneRequestWrapperPtr]);
        lib.insert_oneCallback = koffi.proto('void insert_oneCallback(InsertOneResponseWrapper*)');
        lib.insert_one_async = lib.func('insert_one_async', 'void', [ClientWrapperPtr, InsertOneRequestWrapperPtr, koffi.pointer(lib.insert_oneCallback)]);
        lib.free_insert_one_response = lib.func('free_insert_one_response', 'void', [InsertOneResponseWrapperPtr]);
        lib.insert_many = lib.func('insert_many', InsertManyResponseWrapperPtr, [ClientWrapperPtr, InsertManyRequestWrapperPtr]);
        lib.insert_manyCallback = koffi.proto('void insert_manyCallback(InsertManyResponseWrapper*)');
        lib.insert_many_async = lib.func('insert_many_async', 'void', [ClientWrapperPtr, InsertManyRequestWrapperPtr, koffi.pointer(lib.insert_manyCallback)]);
        lib.free_insert_many_response = lib.func('free_insert_many_response', 'void', [InsertManyResponseWrapperPtr]);
        lib.update_one = lib.func('update_one', UpdateOneResponseWrapperPtr, [ClientWrapperPtr, UpdateOneRequestWrapperPtr]);
        lib.update_oneCallback = koffi.proto('void update_oneCallback(UpdateOneResponseWrapper*)');
        lib.update_one_async = lib.func('update_one_async', 'void', [ClientWrapperPtr, UpdateOneRequestWrapperPtr, koffi.pointer(lib.update_oneCallback)]);
        lib.free_update_one_response = lib.func('free_update_one_response', 'void', [UpdateOneResponseWrapperPtr]);
        lib.insert_or_update_one = lib.func('insert_or_update_one', InsertOrUpdateOneResponseWrapperPtr, [ClientWrapperPtr, InsertOrUpdateOneRequestWrapperPtr]);
        lib.insert_or_update_oneCallback = koffi.proto('void insert_or_update_oneCallback(InsertOrUpdateOneResponseWrapper*)');
        lib.insert_or_update_one_async = lib.func('insert_or_update_one_async', 'void', [ClientWrapperPtr, InsertOrUpdateOneRequestWrapperPtr, koffi.pointer(lib.insert_or_update_oneCallback)]);
        lib.free_insert_or_update_one_response = lib.func('free_insert_or_update_one_response', 'void', [InsertOrUpdateOneResponseWrapperPtr]);
        lib.delete_one = lib.func('delete_one', DeleteOneResponseWrapperPtr, [ClientWrapperPtr, DeleteOneRequestWrapperPtr]);
        lib.delete_oneCallback = koffi.proto('void delete_oneCallback(DeleteOneResponseWrapper*)');
        lib.delete_one_async = lib.func('delete_one_async', 'void', [ClientWrapperPtr, DeleteOneRequestWrapperPtr, koffi.pointer(lib.delete_oneCallback)]);
        lib.free_delete_one_response = lib.func('free_delete_one_response', 'void', [DeleteOneResponseWrapperPtr]);
        lib.delete_many = lib.func('delete_many', DeleteManyResponseWrapperPtr, [ClientWrapperPtr, DeleteManyRequestWrapperPtr]);
        lib.delete_manyCallback = koffi.proto('void delete_manyCallback(DeleteManyResponseWrapper*)');
        lib.delete_many_async = lib.func('delete_many_async', 'void', [ClientWrapperPtr, DeleteManyRequestWrapperPtr, koffi.pointer(lib.delete_manyCallback)]);
        lib.free_delete_many_response = lib.func('free_delete_many_response', 'void', [DeleteManyResponseWrapperPtr]);
        lib.download = lib.func('download', DownloadResponseWrapperPtr, [ClientWrapperPtr, DownloadRequestWrapperPtr]);
        lib.downloadCallback = koffi.proto('void downloadCallback(DownloadResponseWrapper*)');
        lib.download_async = lib.func('download_async', 'void', [ClientWrapperPtr, DownloadRequestWrapperPtr, koffi.pointer(lib.downloadCallback)]);
        lib.free_download_response = lib.func('free_download_response', 'void', [DownloadResponseWrapperPtr]);
        lib.upload = lib.func('upload', UploadResponseWrapperPtr, [ClientWrapperPtr, UploadRequestWrapperPtr]);
        lib.uploadCallback = koffi.proto('void uploadCallback(UploadResponseWrapper*)');
        lib.upload_async = lib.func('upload_async', 'void', [ClientWrapperPtr, UploadRequestWrapperPtr, koffi.pointer(lib.uploadCallback)]);
        lib.free_upload_response = lib.func('free_upload_response', 'void', [UploadResponseWrapperPtr]);
        lib.push_workitem = lib.func('push_workitem', PushWorkitemResponseWrapperPtr, [ClientWrapperPtr, PushWorkitemRequestWrapperPtr]);
        lib.push_workitemCallback = koffi.proto('void push_workitemCallback(PushWorkitemResponseWrapper*)');
        lib.push_workitem_async = lib.func('push_workitem_async', 'void', [ClientWrapperPtr, PushWorkitemRequestWrapperPtr, koffi.pointer(lib.push_workitemCallback)]);
        lib.free_push_workitem_response = lib.func('free_push_workitem_response', 'void', [PushWorkitemResponseWrapperPtr]);
        lib.pop_workitem = lib.func('pop_workitem', PopWorkitemResponseWrapperPtr, [ClientWrapperPtr, PopWorkitemRequestWrapperPtr, CString]);
        lib.pop_workitemCallback = koffi.proto('void pop_workitemCallback(PopWorkitemResponseWrapper*)');
        lib.pop_workitem_async = lib.func('pop_workitem_async', 'void', [ClientWrapperPtr, PopWorkitemRequestWrapperPtr, CString, koffi.pointer(lib.pop_workitemCallback)]);
        lib.pop_workitem2_async = lib.func('pop_workitem2_async', 'void', [ClientWrapperPtr, PopWorkitemRequestWrapperPtr, CString, 'int', koffi.pointer(lib.pop_workitemCallback)]);

        lib.free_pop_workitem_response = lib.func('free_pop_workitem_response', 'void', [PopWorkitemResponseWrapperPtr]);
        lib.update_workitem = lib.func('update_workitem', UpdateWorkitemResponseWrapperPtr, [ClientWrapperPtr, UpdateWorkitemRequestWrapperPtr]);
        lib.update_workitemCallback = koffi.proto('void update_workitemCallback(UpdateWorkitemResponseWrapper*)');
        lib.update_workitem_async = lib.func('update_workitem_async', 'void', [ClientWrapperPtr, UpdateWorkitemRequestWrapperPtr, koffi.pointer(lib.update_workitemCallback)]);
        lib.free_update_workitem_response = lib.func('free_update_workitem_response', 'void', [UpdateWorkitemResponseWrapperPtr]);
        lib.delete_workitem = lib.func('delete_workitem', DeleteWorkitemResponseWrapperPtr, [ClientWrapperPtr, DeleteWorkitemRequestWrapperPtr]);
        lib.delete_workitemCallback = koffi.proto('void delete_workitemCallback(DeleteWorkitemResponseWrapper*)');
        lib.delete_workitem_async = lib.func('delete_workitem_async', 'void', [ClientWrapperPtr, DeleteWorkitemRequestWrapperPtr, koffi.pointer(lib.delete_workitemCallback)]);
        lib.free_delete_workitem_response = lib.func('free_delete_workitem_response', 'void', [DeleteWorkitemResponseWrapperPtr]);
        lib.watch = lib.func('watch', WatchResponseWrapperPtr, [ClientWrapperPtr, WatchRequestWrapperPtr]);
        lib.next_watch_event = lib.func('next_watch_event', WatchEventWrapperPtr, [CString]);
        lib.WatchEventCallback = koffi.proto('void WatchEventCallback(WatchEventWrapper*)');
        lib.watchCallback = koffi.proto('void watchCallback(WatchResponseWrapper*)');
        lib.watch_async_async = lib.func('watch_async_async', 'void', [ClientWrapperPtr, WatchRequestWrapperPtr, koffi.pointer(lib.watchCallback), koffi.pointer(lib.WatchEventCallback)]);
        lib.free_watch_event = lib.func('free_watch_event', 'void', [WatchEventWrapperPtr]);
        lib.free_watch_response = lib.func('free_watch_response', 'void', [WatchResponseWrapperPtr]);
        lib.unwatch = lib.func('unwatch', UnwatchResponseWrapperPtr, [ClientWrapperPtr, CString]);
        lib.unwatchCallback = koffi.proto('void unwatchCallback(UnwatchResponseWrapper*)');
        lib.unwatch_async = lib.func('unwatch_async', 'void', [ClientWrapperPtr, CString, 'int', koffi.pointer(lib.unwatchCallback)]);
        lib.free_unwatch_response = lib.func('free_unwatch_response', 'void', [UnwatchResponseWrapperPtr]);

        lib.register_queue = lib.func('register_queue', RegisterQueueResponseWrapperPtr, [ClientWrapperPtr, RegisterQueueRequestWrapperPtr]);
        lib.QueueEventCallback = koffi.proto('const char * QueueEventCallback(QueueEventWrapper*)');
        lib.register_queue_async = lib.func('register_queue_async', RegisterQueueResponseWrapperPtr, [ClientWrapperPtr, RegisterQueueRequestWrapperPtr, koffi.pointer(lib.QueueEventCallback)]);
        lib.free_register_queue_response = lib.func('free_register_queue_response', 'void', [RegisterQueueResponseWrapperPtr]);
        lib.register_exchange = lib.func('register_exchange', RegisterExchangeResponseWrapperPtr, [ClientWrapperPtr, RegisterExchangeRequestWrapperPtr]);
        lib.free_register_exchange_response = lib.func('free_register_exchange_response', 'void', [RegisterExchangeResponseWrapperPtr]);
        lib.rpc = lib.func('rpc', RpcResponseWrapperPtr, [ClientWrapperPtr, QueueMessageRequestWrapperPtr, 'int']);
        lib.rpcCallback = koffi.proto('void rpcCallback(RpcResponseWrapper*)');
        lib.rpc_async = lib.func('rpc_async', 'void', [ClientWrapperPtr, QueueMessageRequestWrapperPtr, koffi.pointer(lib.rpcCallback), 'int']);
        lib.free_rpc_response = lib.func('free_rpc_response', 'void', [RpcResponseWrapperPtr]);
        lib.next_queue_event = lib.func('next_queue_event', QueueEventPtr, [CString]);
        lib.free_queue_event = lib.func('free_queue_event', 'void', [QueueEventPtr]);
        lib.unregister_queue = lib.func('unregister_queue', UnRegisterQueueResponseWrapperPtr, [ClientWrapperPtr, CString]);
        lib.free_unregister_queue_response = lib.func('free_unregister_queue_response', 'void', [UnRegisterQueueResponseWrapperPtr]);
        lib.queue_message = lib.func('queue_message', QueueMessageResponseWrapperPtr, [ClientWrapperPtr, QueueMessageRequestWrapperPtr]);
        lib.free_queue_message_response = lib.func('free_queue_message_response', 'void', [QueueMessageResponseWrapperPtr]);

        lib.invoke_openrpa = lib.func('invoke_openrpa', InvokeOpenRPAResponseWrapperPtr, [ClientWrapperPtr, InvokeOpenRPARequestWrapperPtr, 'int']);
        lib.free_invoke_openrpa_response = lib.func('free_invoke_openrpa_response', 'void', [InvokeOpenRPAResponseWrapperPtr]);

        return lib;
    } catch (e) {
        throw new LibraryLoadError(`Failed to load library: ${e.message}`);
    }
}

// Custom error classes
class ClientError extends Error {
    constructor(message) {
        super(message);
        this.name = "ClientError";
    }
}

class LibraryLoadError extends ClientError {
    constructor(message) {
        super(message);
        this.name = "LibraryLoadError";
    }
}

class ClientCreationError extends ClientError {
    constructor(message) {
        super(message);
        this.name = "ClientCreationError";
    }
}

// Client class
class Client {
    constructor() {
        this.lib = loadLibrary();
        this.connected = false;
        const _clientWrapperPtr = this.lib.create_client();
        if (_clientWrapperPtr === 0) {
            this.trace('Received a null pointer from Rust function');
            throw new Error('Received a null pointer from Rust function');
        }
        const clientWrapper = koffi.decode(_clientWrapperPtr, ClientWrapper);
        this.client = _clientWrapperPtr;
        this.lib.set_agent_name(_clientWrapperPtr, 'node');
    }
    formatBytes(bytes, decimals = 2) {
        if (bytes === 0) return "0 Bytes";
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if (bytes < 0) {
            bytes = bytes * -1;
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return "-" + parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
        } else {
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
        }
    }

    free() {
        if (this.client) {
            this.trace('this.client not null, call free_client');
            this.lib.free_client(this.client);
            this.client = null;
        }
        this.connected = false;
        this.trace('free::end');
    }

    enable_tracing(rust_log = '', tracing = '') {
        this.trace('enable_tracing invoked', rust_log, tracing);
        this.lib.enable_tracing(rust_log, tracing);
        this.informing = true;
        this.trace('enable_tracing called');
    }
    disable_tracing() {
        this.lib.disable_tracing();
    }
    error(...args) {
        this.lib.error(args.join(' '));
    }
    info(...args) {
        this.lib.info(args.join(' '));
    }
    warn(...args) {
        this.lib.warn(args.join(' '));
    }
    debug(...args) {
        this.lib.debug(args.join(' '));
    }
    trace(...args) {
        this.lib.trace(args.join(' '));
    }
    set_f64_observable_gauge(name, value, description) {
        this.lib.set_f64_observable_gauge(name, value, description);
    }
    set_u64_observable_gauge(name, value, description) {
        this.lib.set_u64_observable_gauge(name, value, description);
    }
    set_i64_observable_gauge(name, value, description) {
        this.lib.set_i64_observable_gauge(name, value, description);
    }
    disable_observable_gauge(name) {
        this.lib.disable_observable_gauge(name);
    }
    set_agent_name(name) {
        this.trace('set_agent_name invoked', name);
        this.lib.set_agent_name(this.client, name);
    }
    set_default_timeout(timeout) {
        this.trace('set_default_timeout invoked', timeout);
        this.lib.client_set_default_timeout(this.client, timeout);
    }
    get_default_timeout() {
        this.trace('get_default_timeout invoked');
        let timeout = this.lib.client_get_default_timeout(this.client);
        return timeout;
    }
    get_state() {
        this.trace('get_state invoked');
        let state = this.lib.client_get_state(this.client);
        return state;
    }
    set_agent_version(version) {
        this.trace('set_agent_version invoked', version);
        this.lib.set_agent_version(this.client, version);
    }
    stringify(obj) {
        if(obj === null) {
            return '';
        }
        if (typeof data === 'object') {
            return JSON.stringify(data);
        }
        if(typeof obj === 'string') {
            return obj;
        }
        return JSON.stringify(obj);
    }
    async connect(url = "") {
        this.trace('connect invoked', url);
        this.connected = false;
        const ResponsePtr = this.lib.connect(this.client, url);
        if (ResponsePtr === 0) {
            this.trace('Received a null pointer from Rust function');
            throw new Error('Received a null pointer from Rust function');
        }
        this.trace('Callback invoked');
        const Response = koffi.decode(ResponsePtr, ConnectResponseWrapper);
        if (!Response.success) {
            throw new ClientCreationError(Response.error);
        }
        this.connected = true;
        return Response;
    }

    connect_async(url = "") {
        this.trace('connect_async invoked', url);
        this.connected = false;
        return new Promise((resolve, reject) => {
            try {
                const cb = koffi.register((responsePtr) => {
                    this.trace('Callback invoked');
                    try {
                        if (responsePtr === 0) {
                            throw new Error('Received a null pointer from Rust function');
                        }
                        const Response = koffi.decode(responsePtr, ConnectResponseWrapper);
                        if (!Response.success) {
                            reject(new ClientCreationError(Response.error));
                        } else {
                            this.connected = true;
                            resolve(Response);
                        }
                    } catch (error) {
                        reject(new ClientCreationError(error.message));
                    }
                }, koffi.pointer(this.lib.ConnectCallback));
                this.trace('call connect_async');
                this.lib.connect_async(this.client, url, 0, cb);
            } catch (error) {
                reject(new ClientCreationError(error.message));
            }
        });
    }

    disconnect() {
        this.trace('disconnect invoked');
        this.lib.disconnect(this.client);
        this.connected = false;
    }

    signin({ username = '', password = '', jwt = '', agent = '', version = '', longtoken = false, validateonly = false, ping = false } = {}) {
        this.trace('signin invoked');
        const req = {
            username: username,
            password: password,
            jwt: jwt,
            agent: agent,
            version: version,
            longtoken: longtoken,
            validateonly: validateonly,
            ping: ping
        };
        this.trace('call signin');
        const response = this.lib.signin(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, SigninResponseWrapper);
        this.trace('free_signin_response');
        this.lib.free_signin_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return {
            success: result.success,
            jwt: result.jwt,
            error: null
        };
    }
    signin_async({ username = '', password = '', jwt = '', agent = '', version = '', longtoken = false, validateonly = false, ping = false } = {}) {
        this.trace('signin invoked');
        return new Promise((resolve, reject) => {
            const req = {
                username: username,
                password: password,
                jwt: jwt,
                agent: agent,
                version: version,
                longtoken: longtoken,
                validateonly: validateonly,
                ping: ping
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('signin_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, SigninResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response);
                }
                this.trace('free_signin_response')
                this.lib.free_signin_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.signinCallback));
            this.trace('call signin_async');
            this.lib.signin_async(this.client, req, cb, (err, res) => {
                this.error('signin_async error', err, res);
                if (err) {
                    reject(new ClientError('Signin failed'));
                }
            });
        });
    }

    list_collections(includehist = false) {
        this.trace('list_collections invoked');
        const responsePtr = this.lib.list_collections(this.client, includehist);
        const response = koffi.decode(responsePtr, ListCollectionsResponseWrapper);
        this.lib.free_list_collections_response(responsePtr);
        if (!response.success) {
            const errorMsg = response.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(response.collections);
    }
    list_collections_async(includehist = false) {
        this.trace('list_collections invoked');
        return new Promise((resolve, reject) => {
            const callback = (responsePtr) => {
                this.trace('list_collections_async callback');
                const response = koffi.decode(responsePtr, ListCollectionsResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.collections));
                }
                this.trace('free_list_collections_response');
                this.lib.free_list_collections_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.listCollectionsCallback));
            this.trace('call list collections async');
            this.lib.list_collections_async(this.client, includehist, 0, cb, (err) => {
                if (err) {
                    reject(new ClientError('List collections failed'));
                }
            });
        });
    }
    // "seconds" | "minutes" | "hours"
    create_collection({ collectionname, collation = undefined, timeseries = undefined, expire_after_seconds = 0, change_stream_pre_and_post_images = false, capped = false, max = 0, size = 0 }) {
        this.trace('create_collection invoked');
        const req = {
            collectionname: collectionname,
            collation: collation,
            timeseries: timeseries,
            expire_after_seconds: expire_after_seconds,
            change_stream_pre_and_post_images: change_stream_pre_and_post_images,
            capped: capped,
            max: max,
            size: size
        };
        this.trace('call create_collection');
        const response = this.lib.create_collection(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, CreateCollectionResponseWrapper);
        this.trace('free_create_collection_response');
        this.lib.free_create_collection_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.success;
    }
    create_collection_async({ collectionname, collation = undefined, timeseries = undefined, expire_after_seconds = 0, change_stream_pre_and_post_images = false, capped = false, max = 0, size = 0 }) {
        this.trace('create_collection invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                collation: collation,
                timeseries: timeseries,
                expire_after_seconds: expire_after_seconds,
                change_stream_pre_and_post_images: change_stream_pre_and_post_images,
                capped: capped,
                max: max,
                size: size
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('create_collection_async callback');
                const response = koffi.decode(responsePtr, CreateCollectionResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.success);
                }
                this.trace('free_create_collection_response');
                this.lib.free_create_collection_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.create_collectionCallback));
            this.trace('call create_collection_async');
            this.lib.create_collection_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Create collection failed'));
                }
            });
        });
    }

    drop_collection(collectionname) {
        this.trace('drop_collection invoked');
        const response = this.lib.drop_collection(this.client, collectionname);
        const result = koffi.decode(response, DropCollectionResponseWrapper);
        this.lib.free_drop_collection_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
    }
    drop_collection_async(collectionname) {
        this.trace('drop_collection invoked');
        return new Promise((resolve, reject) => {
            const response = this.lib.drop_collection(this.client, collectionname);
            const result = koffi.decode(response, DropCollectionResponseWrapper);
            this.lib.free_drop_collection_response(response);
            if (!result.success) {
                const errorMsg = result.error;
                reject(new ClientError(errorMsg));
            } else {
                resolve();
            }
        });
    }

    get_indexes(collectionname) {
        this.trace('get_indexes invoked');
        const response = this.lib.get_indexes(this.client, collectionname);
        const result = koffi.decode(response, GetIndexesResponseWrapper);
        this.lib.free_get_indexes_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.indexes);
    }
    get_indexes_async(collectionname) {
        this.trace('get_indexes invoked');
        return new Promise((resolve, reject) => {
            const response = this.lib.get_indexes(this.client, collectionname);
            const result = koffi.decode(response, GetIndexesResponseWrapper);
            this.lib.free_get_indexes_response(response);
            if (!result.success) {
                const errorMsg = result.error;
                reject(new ClientError(errorMsg));
            } else {
                resolve(JSON.parse(result.indexes));
            }
        });
    }

    create_index({ collectionname, index, options = {}, name = "" }) {
        this.trace('create_index invoked');
        const req = {
            collectionname: collectionname,
            index: this.stringify(index),
            options: this.stringify(options),
            name: name
        };
        this.trace('call create_index');
        const response = this.lib.create_index(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, CreateIndexResponseWrapper);
        this.trace('free_create_index_response');
        this.lib.free_create_index_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.success;
    }
    create_index_async({ collectionname, index, options = {}, name = "" }) {
        this.trace('create_index invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                index: this.stringify(index),
                options: this.stringify(options),
                name: name
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('create_index_async callback');
                const response = koffi.decode(responsePtr, CreateIndexResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.success);
                }
                this.trace('free_create_index_response');
                this.lib.free_create_index_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.create_indexCallback));
            this.trace('call create_index_async');
            this.lib.create_index_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Create index failed'));
                }
            });
        });
    }

    drop_index(collectionname, indexname) {
        this.trace('drop_index invoked');
        const response = this.lib.drop_index(this.client, collectionname, indexname);
        const result = koffi.decode(response, DropIndexResponseWrapper);
        this.lib.free_drop_index_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
    }
    drop_index_async(collectionname, indexname) {
        this.trace('drop_index invoked');
        return new Promise((resolve, reject) => {
            const response = this.lib.drop_index(this.client, collectionname, indexname);
            const result = koffi.decode(response, DropIndexResponseWrapper);
            this.lib.free_drop_index_response(response);
            if (!result.success) {
                const errorMsg = result.error;
                reject(new ClientError(errorMsg));
            } else {
                resolve();
            }
        });
    }

    custom_command({ command, id = "", name = "", data = {}, timeout = -1}) {
        this.trace('custom_command invoked');
        const req = {
            command: command,
            id: id,
            name: name,
            data: this.stringify(data)
        };
        this.trace('call custom_command');
        const response = this.lib.custom_command(this.client, req, timeout);
        this.trace('decode response');
        const result = koffi.decode(response, CustomCommandResponseWrapper);
        this.trace('free_custom_command_response');
        this.lib.free_custom_command_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        try {
            return JSON.parse(result.result);
        } catch (error) {            
        }
        return result.result;
    }
    custom_command_async({ command, id = "", name = "", data = {}, timeout = -1}) {
        this.trace('custom_command invoked');
        return new Promise((resolve, reject) => {
            const req = {
                command: command,
                id: id,
                name: name,
                data: this.stringify(data)
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('custom_command_async callback');
                const response = koffi.decode(responsePtr, CustomCommandResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    try {
                        resolve(JSON.parse(response.result));
                    } catch (error) {
                        resolve(response.result);
                    }
                }
                this.trace('free_custom_command_response');
                this.lib.free_custom_command_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.custom_commandCallback));
            this.trace('call custom_command_async');
            this.lib.custom_command_async(this.client, req, cb, timeout, (err) => {
                if (err) {
                    reject(new ClientError('Custom command failed'));
                }
            });
        });
    }

    query({ collectionname, query, projection = {}, orderby = "", skip = 0, top = 100, queryas = "", explain = false }) {
        this.trace('query invoked');
        const req = {
            collectionname: collectionname,
            query: this.stringify(query),
            projection: this.stringify(projection),
            orderby: this.stringify(orderby),
            queryas: queryas,
            explain: explain,
            skip: skip,
            top: top
        };
        this.trace('call query');
        const responsePtr = this.lib.query(this.client, req);
        this.trace('decode response');
        const response = koffi.decode(responsePtr, QueryResponseWrapper);
        this.trace('free_query_response');
        this.lib.free_query_response(responsePtr);
        if (!response.success) {
            const errorMsg = response.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(response.results);
    }

    query_async({ collectionname, query, projection = {}, orderby = "", skip = 0, top = 100, queryas = "", explain = false }) {
        this.trace('query_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                query: this.stringify(query),
                projection: this.stringify(projection),
                orderby: this.stringify(orderby),
                queryas: queryas,
                explain: explain,
                skip: skip,
                top: top
            };
            const callback = (responsePtr) => {
                this.trace('query_async callback');
                const response = koffi.decode(responsePtr, QueryResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.trace('free_query_response::begin');
                this.lib.free_query_response(responsePtr);
                this.trace('free_query_response::end');
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.queryCallback));
            this.trace('call query_async');
            this.lib.query_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Query failed'));
                }
            });
        });
    }
    aggregate({ collectionname, aggregates = [], queryas = "", hint = "", explain = false }) {
        this.trace('aggregate invoked');
        const req = {
            collectionname: collectionname,
            aggregates: this.stringify(aggregates),
            queryas: queryas,
            hint: hint,
            explain: explain
        };
        this.trace('call aggregate');
        const response = this.lib.aggregate(this.client, req);
        const result = koffi.decode(response, AggregateResponseWrapper);
        this.trace('free_aggregate_response');
        this.lib.free_aggregate_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.results);
    }
    aggregate_async({ collectionname, aggregates = [], queryas = "", hint = "", explain = false }) {
        this.trace('aggregate invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                aggregates: this.stringify(aggregates),
                queryas: queryas,
                hint: hint,
                explain: explain
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('aggregate_async callback');
                const response = koffi.decode(responsePtr, AggregateResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.trace('free_aggregate_response');
                this.lib.free_aggregate_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.aggregateCallback));

            this.trace('call aggregate_async');
            this.lib.aggregate_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Aggregate failed'));
                }
            });
        });
    }
    count({ collectionname, query = {}, queryas = "", explain = false }) {
        this.trace('count invoked');
        const req = {
            collectionname: collectionname,
            query: this.stringify(query),
            queryas: queryas,
            explain: explain
        };
        this.trace('call count');
        const response = this.lib.count(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, CountResponseWrapper);
        this.trace('free_count_response');
        this.lib.free_count_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.result;
    }
    count_async({ collectionname, query = {}, queryas = "", explain = false }) {
        this.trace('count async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                query: this.stringify(query),
                queryas: queryas,
                explain: explain
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('count_async callback');
                const response = koffi.decode(responsePtr, CountResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.result);
                }
                this.trace('free_count_response');
                this.lib.free_count_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.countCallback));

            this.trace('call count_async');
            this.lib.count_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Count failed'));
                }
            });
        });
    }
    distinct({ collectionname, field, query = {}, queryas = "", explain = false }) {
        this.trace('distinct invoked');
        const req = {
            collectionname: collectionname,
            field: field,
            query: this.stringify(query),
            queryas: queryas,
            explain: explain
        };
        this.trace('call distinct');
        const responsePtr = this.lib.distinct(this.client, req);
        this.trace('decode response');
        let results = [];
        const response = koffi.decode(responsePtr, DistinctResponseWrapper);
        if (response.success) {
            this.trace('decode response results');
            let strings = koffi.decode(response.results, 'void *', -1);
            for (let i = 0; i < response.results_len; i++) {
                this.trace('decode response results #', i);
                let ptr = strings[i];
                let value = koffi.decode(ptr, 'char', -1);
                results.push(value.toString());
            };
        }
        this.trace('free_distinct_response');
        this.lib.free_distinct_response(responsePtr);
        if (!response.success) {
            const errorMsg = response.error;
            throw new ClientError(errorMsg);
        }
        return results;
    }
    distinct_async({ collectionname, field, query = {}, queryas = "", explain = false }) {
        this.trace('distinct invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                field: field,
                query: this.stringify(query),
                queryas: queryas,
                explain: explain
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('distinct_async callback');
                const response = koffi.decode(responsePtr, DistinctResponseWrapper);
                let results = [];
                if (response.success) {
                    let strings = koffi.decode(response.results, 'void *', -1);
                    for (let i = 0; i < response.results_len; i++) {
                        let ptr = strings[i];
                        let value = koffi.decode(ptr, 'char', -1);
                        results.push(value.toString());
                    };
                }
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(results);
                }
                this.trace('free_distinct_response');
                this.lib.free_distinct_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.distinctCallback));

            this.trace('call distinct_async');
            this.lib.distinct_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Distinct failed'));
                }
            });
        });
    }
    insert_one({ collectionname, item, w = 1, j = false }) {
        this.trace('insert_one invoked');
        const req = {
            collectionname: collectionname,
            item: this.stringify(item),
            w: w,
            j: j
        };
        this.trace('call insert_one');
        const response = this.lib.insert_one(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, InsertOneResponseWrapper);
        this.trace('free_insert_one_response');
        this.lib.free_insert_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_one_async({ collectionname, item, w = 1, j = false }) {
        this.trace('insert_one async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: this.stringify(item),
                w: w,
                j: j
            };
            const callback = (responsePtr) => {
                this.trace('insert_one_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, InsertOneResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.trace('free_insert_one_response');
                this.lib.free_insert_one_response(responsePtr);
            }
            const cb = koffi.register(callback, koffi.pointer(this.lib.insert_oneCallback));
            this.trace('call insert_one_async');
            this.lib.insert_one_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertOne failed'));
                }
            });
        });
    };
    insert_many({ collectionname, items = [], w = 1, j = false, skipresults = false }) {
        this.trace('insert_many invoked');
        if(items == null || items.length == 0 ){
            throw new ClientError('Items cannot be null or empty');
        }
        const req = {
            collectionname: collectionname,
            items: this.stringify(items),
            w: w,
            j: j,
            skipresults: skipresults
        };
        this.trace('call insert_many');
        const response = this.lib.insert_many(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, InsertManyResponseWrapper);
        this.trace('free_insert_many_response');
        this.lib.free_insert_many_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.results);
    }
    insert_many_async({ collectionname, items = [], w = 1, j = false, skipresults = false }) {
        this.trace('insert_many invoked');
        if(items == null || items.length == 0 ){
            throw new ClientError('Items cannot be null or empty');
        }
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                items: this.stringify(items),
                w: w,
                j: j,
                skipresults: skipresults
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('insert_many_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, InsertManyResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.trace('free_insert_many_response');
                this.lib.free_insert_many_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.insert_manyCallback));
            this.trace('call insert_many_async');
            this.lib.insert_many_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertMany failed'));
                }
            });
        });
    }
    update_one({ collectionname, item, w = 1, j = false }) {
        this.trace('update_one invoked');
        const req = {
            collectionname: collectionname,
            item: this.stringify(item),
            w: w,
            j: j
        };
        this.trace('call update_one');
        const response = this.lib.update_one(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, UpdateOneResponseWrapper);
        this.trace('free_update_one_response');
        this.lib.free_update_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    update_one_async({ collectionname, item, w = 1, j = false }) {
        this.trace('update_one invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: this.stringify(item),
                w: w,
                j: j
            };
            const callback = (responsePtr) => {
                this.trace('update_one_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, UpdateOneResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.trace('free_update_one_response');
                this.lib.free_update_one_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.update_oneCallback));
            this.trace('call update_one_async');
            this.lib.update_one_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('UpdateOne failed'));
                }
            });
        });
    }
    insert_or_update_one({ collectionname, item, uniqeness = "_id", w = 1, j = false }) {
        this.trace('insert_or_update_one invoked');
        const req = {
            collectionname: collectionname,
            item: this.stringify(item),
            uniqeness: uniqeness,
            w: w,
            j: j
        };
        this.trace('call insert_or_update_one');
        const response = this.lib.insert_or_update_one(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, InsertOrUpdateOneResponseWrapper);
        this.trace('free_insert_or_update_one_response');
        this.lib.free_insert_or_update_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_or_update_one_async({ collectionname, item, uniqeness = "_id", w = 1, j = false }) {
        this.trace('insert_or_update_one invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: this.stringify(item),
                uniqeness: uniqeness,
                w: w,
                j: j
            };
            const callback = (responsePtr) => {
                this.trace('insert_or_update_one_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, InsertOrUpdateOneResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.trace('free_insert_or_update_one_response');
                this.lib.free_insert_or_update_one_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.insert_or_update_oneCallback));
            this.trace('call insert_or_update_one_async');
            this.lib.insert_or_update_one_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertOrUpdateOne failed'));
                }
            });
        });
    }
    delete_one({ collectionname, id, recursive = false }) {
        this.trace('delete_one invoked');
        const req = {
            collectionname: collectionname,
            id: id,
            recursive: recursive
        };
        this.trace('call delete_one');
        const response = this.lib.delete_one(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, DeleteOneResponseWrapper);
        this.trace('free_delete_one_response');
        this.lib.free_delete_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.affectedrows;
    }
    delete_one_async({ collectionname, id, recursive = false }) {
        this.trace('delete_one_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                id: id,
                recursive: recursive
            };
            const callback = (responsePtr) => {
                this.trace('delete_one_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, DeleteOneResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.affectedrows);
                }
                this.trace('free_delete_one_response');
                this.lib.free_delete_one_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.delete_oneCallback));
            this.trace('call delete_one_async');
            this.lib.delete_one_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('DeleteOne failed'));
                }
            });
        });
    }
    delete_many({ collectionname, query, recursive = false }) {
        this.trace('delete_many invoked');
        const req = {
            collectionname: collectionname,
            query: JSON.stringify(query),
            ids: null,
            recursive: recursive
        };
        ids.push(null); // terminate array
        req.ids = ids;
        this.trace('call delete_many');
        const response = this.lib.delete_many(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, DeleteManyResponseWrapper);
        this.trace('free_delete_many_response');
        this.lib.free_delete_many_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.affectedrows;
    }
    delete_many_async({ collectionname, query, recursive = false }) {
        this.trace('delete_many_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                query: query,
                ids: null,
                recursive: recursive
            };
            ids.push(null); // terminate array
            req.ids = ids;
            const callback = (responsePtr) => {
                this.trace('delete_many_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, DeleteManyResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.affectedrows);
                }
                this.trace('free_delete_many_response');
                this.lib.free_delete_many_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.delete_manyCallback));
            this.trace('call delete_many_async');
            this.lib.delete_many_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('DeleteMany failed'));
                }
            });
        });
    }
    download({ collectionname, id, folder = "", filename = "" }) {
        this.trace('download invoked');
        const req = {
            collectionname: collectionname,
            id: id,
            folder: folder,
            filename: filename
        };
        this.trace('call download');
        const response = this.lib.download(this.client, req);
        const result = koffi.decode(response, DownloadResponseWrapper);
        this.trace('free_download_response');
        this.lib.free_download_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.filename;
    }
    download_async({ collectionname, id, folder = "", filename = "" }) {
        this.trace('download async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                id: id,
                folder: folder,
                filename: filename
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('download_async callback');
                const response = koffi.decode(responsePtr, DownloadResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.filename);
                }
                this.trace('free_download_response');
                this.lib.free_download_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.downloadCallback));
            this.trace('call download_async');
            this.lib.download_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Download failed'));
                }
            });
        });
    }
    upload({ filepath, filename, mimetype = "application/octet-stream", metadata = {}, collectionname = "fs.files" }) {
        this.trace('upload invoked');
        const req = {
            filepath: filepath,
            filename: filename,
            mimetype: mimetype,
            metadata: JSON.stringify(metadata),
            collectionname: collectionname
        };
        this.trace('call upload');
        const response = this.lib.upload(this.client, req);
        const result = koffi.decode(response, UploadResponseWrapper);
        this.trace('free_upload_response');
        this.lib.free_upload_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.id
    }
    upload_async({ filepath, filename, mimetype = "application/octet-stream", metadata = {}, collectionname = "fs.files" }) {
        this.trace('upload async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                filepath: filepath,
                filename: filename,
                mimetype: mimetype,
                metadata: metadata,
                collectionname: collectionname
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('upload_async callback');
                const response = koffi.decode(responsePtr, UploadResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.id);
                }
                this.trace('free_upload_response');
                this.lib.free_upload_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.uploadCallback));
            this.trace('call upload_async');
            this.lib.upload_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('Upload failed'));
                }
            });
        });
    }
    push_workitem({ wiq = "", wiqid = "", name, payload = {}, nextrun = 0, success_wiqid = "", failed_wiqid = "", success_wiq = "", failed_wiq = "", priority = 2,
        files = []
    }) {
        this.trace('push_workitem invoked');
        // if nextrun is not null and nextrun is a date
        if (nextrun != null && nextrun instanceof Date) {
            this.trace('Node.js: nextrun before', nextrun);
            // then convert nextrun to a number ( POSIX time )
            nextrun = Math.floor(nextrun.getTime() / 1000); // Convert to seconds
        } else {
            nextrun = 0;
        }
        const req = {
            wiq: wiq,
            wiqid: wiqid,
            name: name,
            payload: this.stringify(payload),
            nextrun: nextrun,
            success_wiqid: success_wiqid,
            failed_wiqid: failed_wiqid,
            success_wiq: success_wiq,
            failed_wiq: failed_wiq,
            priority: priority,
            files: files,
            files_len: files.length
        };
        encode_files(req);
        this.trace('call push_workitem');
        const response = this.lib.push_workitem(this.client, req);
        try {
            this.trace('decode response');
            const result = koffi.decode(response, PushWorkitemResponseWrapper);
            if (!result.success) {
                const errorMsg = result.error;
                throw new ClientError(errorMsg);
            }
            if (result.workitem != null) {
                var workitem = koffi.decode(result.workitem, WorkitemWrapper);
                decode_files(workitem);
                if (workitem.nextrun > 0) {
                    workitem.nextrun = new Date(workitem.nextrun * 1000);
                } else {
                    delete workitem.nextrun;
                }
                if (workitem.lastrun > 0) {
                    workitem.lastrun = new Date(workitem.lastrun * 1000);
                } else {
                    delete workitem.lastrun;
                }
                try {
                    if (workitem.payload != null && workitem.payload != "") {
                        workitem.payload = JSON.parse(workitem.payload);
                    }
                } catch (error) {
                }
                return workitem;
            }
            return null;
        } finally {
            this.trace('free_push_workitem_response');
            this.lib.free_push_workitem_response(response);
        }
    }
    push_workitem_async({ wiq = "", wiqid = "", name, payload = {}, nextrun = 0, success_wiqid = "", failed_wiqid = "", success_wiq = "", failed_wiq = "", priority = 2,
        files = []
    }) {
        this.trace('push_workitem invoked');
        return new Promise((resolve, reject) => {
            // if nextrun is not null and nextrun is a date
            if (nextrun != null && nextrun instanceof Date) {
                this.trace('Node.js: nextrun before', nextrun);
                // then convert nextrun to a number ( POSIX time )
                nextrun = Math.floor(nextrun.getTime() / 1000); // Convert to seconds
            } else {
                nextrun = 0;
            }
            const req = {
                wiq: wiq,
                wiqid: wiqid,
                name: name,
                payload: this.stringify(payload),
                nextrun: nextrun,
                success_wiqid: success_wiqid,
                failed_wiqid: failed_wiqid,
                success_wiq: success_wiq,
                failed_wiq: failed_wiq,
                priority: priority,
                files: files,
                files_len: files.length
            };
            encode_files(req);
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('push_workitem_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, PushWorkitemResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    if (response.workitem != null) {
                        var workitem = koffi.decode(response.workitem, WorkitemWrapper);
                        decode_files(workitem);
                        if (workitem.nextrun > 0) {
                            workitem.nextrun = new Date(workitem.nextrun * 1000);
                        } else {
                            delete workitem.nextrun;
                        }
                        if (workitem.lastrun > 0) {
                            workitem.lastrun = new Date(workitem.lastrun * 1000);
                        } else {
                            delete workitem.lastrun;
                        }
                        try {
                            if (workitem.payload != null && workitem.payload != "") {
                                workitem.payload = JSON.parse(workitem.payload);
                            }
                        } catch (error) {
                        }
                        return resolve(workitem);
                    }
                    resolve(null);
                }
                this.trace('free_push_workitem_response');
                this.lib.free_push_workitem_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.push_workitemCallback));
            this.trace('call push_workitem_async');
            this.lib.push_workitem_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('PushWorkitem async failed'));
                }
            }
            );
        });
    }
    pop_workitem({ wiq = "", wiqid = "", downloadfolder = "." }) {
        this.trace('pop_workitem invoked');
        if (downloadfolder == null || downloadfolder == "") downloadfolder = ".";
        const req = {
            wiq: wiq,
            wiqid: wiqid
        };
        this.trace('call pop_workitem');
        const response = this.lib.pop_workitem(this.client, req, downloadfolder);
        try {
            this.trace('decode response');
            const result = koffi.decode(response, PopWorkitemResponseWrapper);
            if (!result.success) {
                const errorMsg = result.error;
                throw new ClientError(errorMsg);
            }
            if (result.workitem != null) {
                var workitem = koffi.decode(result.workitem, WorkitemWrapper);
                decode_files(workitem);
                if (workitem.nextrun > 0) {
                    workitem.nextrun = new Date(workitem.nextrun * 1000);
                } else {
                    delete workitem.nextrun;
                }
                if (workitem.lastrun > 0) {
                    workitem.lastrun = new Date(workitem.lastrun * 1000);
                } else {
                    delete workitem.lastrun;
                }
                try {
                    if (workitem.payload != null && workitem.payload != "") {
                        workitem.payload = JSON.parse(workitem.payload);
                    }
                } catch (error) {
                }
                return workitem;
            }
            return null;            
        } finally {
            this.trace('free_pop_workitem_response');
            this.lib.free_pop_workitem_response(response);            
        }
    }
    callbackid = 0;
    callbacks = {};
    pop_workitem_async_callback(responsePtr) {
        this.trace('pop_workitem_async callback');
        this.trace('decode response');
        const response = koffi.decode(responsePtr, PopWorkitemResponseWrapper);
        const request_id = response.request_id;
        if (this.callbacks[request_id] == null) {
            this.error(`Callback for request_id ${request_id} not found!`);
            return;
        }
        const { resolve, reject, callback } = this.callbacks[request_id];
        delete this.callbacks[request_id];
        koffi.unregister(callback);
        let keys = Object.keys(this.callbacks);
        if(keys.length > 0) {
            this.trace(`Deleted callback for request_id ${request_id}, i now have ${keys.length} remaining callbacks`);
        }
        try {
            if (!response.success) {
                const errorMsg = response.error;
                reject(new ClientError(errorMsg));
            } else {
                if (response.workitem != null) {
                    var workitem = koffi.decode(response.workitem, WorkitemWrapper);
                    decode_files(workitem);
                    if (workitem.nextrun > 0) {
                        workitem.nextrun = new Date(workitem.nextrun * 1000);
                    } else {
                        delete workitem.nextrun;
                    }
                    if (workitem.lastrun > 0) {
                        workitem.lastrun = new Date(workitem.lastrun * 1000);
                    } else {
                        delete workitem.lastrun;
                    }
                    try {
                        if (workitem.payload != null && workitem.payload != "") {
                            workitem.payload = JSON.parse(workitem.payload);
                        }
                    } catch (error) {
                    }
                    resolve(workitem);
                } else {
                    resolve(null);
                }
            }
        } catch (error) {
            this.error(error);
        } finally {
            this.trace('free_pop_workitem_response');
            this.lib.free_pop_workitem_response(responsePtr);
            responsePtr = null;
        }
    }
    pop_workitem_async({ wiq = "", wiqid = "", downloadfolder = "." }) {
        this.trace('pop_workitem async invoked');
        return new Promise((resolve, reject) => {
            if (downloadfolder == null || downloadfolder == "") downloadfolder = ".";

            this.callbackid++;
            let request_id = this.callbackid;
            this.callbacks[request_id] = {
                resolve: resolve,
                reject: reject,
                callback: koffi.register(this, this.pop_workitem_async_callback, koffi.pointer(this.lib.pop_workitemCallback))
            }
            const req = {
                wiq: wiq,
                wiqid: wiqid,
                request_id: request_id
            };
            this.trace('call pop_workitem_async');
            this.lib.pop_workitem_async(this.client, req, downloadfolder, this.callbacks[request_id].callback);
        });
    }
    update_workitem({ workitem, ignoremaxretries = false, files = [] }) {
        this.trace('update_workitem invoked');
        workitem = Object.assign({
            id: "",
            name: "",
            payload: "",
            priority: 2,
            nextrun: 0,
            lastrun: 0,
            files: [null],
            files_len: 0,
            state: "",
            wiq: "",
            wiqid: "",
            retries: 3,
            username: "",
            success_wiqid: "",
            failed_wiqid: "",
            success_wiq: "",
            failed_wiq: "",
            errormessage: "",
            errorsource: "",
            errortype: ""
        }, workitem);
        if (workitem.payload == null) workitem.payload = "{}";
        if (workitem.payload != null && typeof workitem.payload === 'object') {
            workitem.payload = JSON.stringify(workitem.payload);
        }
        const req = {
            workitem: workitem,
            ignoremaxretries: ignoremaxretries,
            files: files,
            files_len: files.length
        };
        encode_files(req);
        if (workitem.nextrun != null && workitem.nextrun instanceof Date) {
            this.trace('Node.js: nextrun before', workitem.nextrun);
            // then convert nextrun to a number ( POSIX time )
            workitem.nextrun = Math.floor(workitem.nextrun.getTime() / 1000); // Convert to seconds
        } else {
            workitem.nextrun = 0;
        }
        if (workitem.lastrun != null && workitem.lastrun instanceof Date) {
            this.trace('Node.js: lastrun before', workitem.lastrun);
            // then convert lastrun to a number ( POSIX time )
            workitem.lastrun = Math.floor(workitem.lastrun.getTime() / 1000); // Convert to seconds
        } else {
            workitem.lastrun = 0;
        }
        encode_files(workitem);
        this.trace('encode workitem');
        req.workitem = workitem;
        this.trace('encode request');
        this.trace('call update_workitem with ', files.length, ' files');
        const response = this.lib.update_workitem(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, UpdateWorkitemResponseWrapper);
        try {
            if (!result.success) {
                const errorMsg = result.error;
                throw new ClientError(errorMsg);
            }
            if (result.workitem != null) {
                var workitem = koffi.decode(result.workitem, WorkitemWrapper);
                decode_files(workitem);
                if (workitem.nextrun > 0) {
                    workitem.nextrun = new Date(workitem.nextrun * 1000);
                } else {
                    delete workitem.nextrun;
                }
                if (workitem.lastrun > 0) {
                    workitem.lastrun = new Date(workitem.lastrun * 1000);
                } else {
                    delete workitem.lastrun;
                }
                try {
                    if (workitem.payload != null && workitem.payload != "") {
                        workitem.payload = JSON.parse(workitem.payload);
                    }
                } catch (error) {
                }
                return workitem;
            }
            return null;
        } finally {
            this.trace('free_update_workitem_response');
            this.lib.free_update_workitem_response(response);
        }
    }
    update_workitem_async({ workitem, ignoremaxretries = false, files = [] }) {
        this.trace('update_workitem async invoked');
        return new Promise((resolve, reject) => {
            workitem = Object.assign({
                id: "",
                name: "",
                payload: "",
                priority: 2,
                nextrun: 0,
                lastrun: 0,
                files: [null],
                files_len: 0,
                state: "",
                wiq: "",
                wiqid: "",
                retries: 3,
                username: "",
                success_wiqid: "",
                failed_wiqid: "",
                success_wiq: "",
                failed_wiq: "",
                errormessage: "",
                errorsource: "",
                errortype: ""
            }, workitem);
            if (workitem.payload == null) workitem.payload = "{}";
            if (workitem.payload != null && typeof workitem.payload === 'object') {
                workitem.payload = JSON.stringify(workitem.payload);
            }
            const req = {
                workitem: workitem,
                ignoremaxretries: ignoremaxretries,
                files: files,
                files_len: files.length
            };
            encode_files(req);
            if (workitem.nextrun != null && workitem.nextrun instanceof Date) {
                this.trace('Node.js: nextrun before', workitem.nextrun);
                // then convert nextrun to a number ( POSIX time )
                workitem.nextrun = Math.floor(workitem.nextrun.getTime() / 1000); // Convert to seconds
            } else {
                workitem.nextrun = 0;
            }
            if (workitem.lastrun != null && workitem.lastrun instanceof Date) {
                this.trace('Node.js: lastrun before', workitem.lastrun);
                // then convert lastrun to a number ( POSIX time )
                workitem.lastrun = Math.floor(workitem.lastrun.getTime() / 1000); // Convert to seconds
            } else {
                workitem.lastrun = 0;
            }
            encode_files(workitem);
            this.trace('encode workitem');
            req.workitem = workitem;
            this.trace('encode request');
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('update_workitem_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, UpdateWorkitemResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    if (response.workitem != null) {
                        var workitem = koffi.decode(response.workitem, WorkitemWrapper);
                        decode_files(workitem);
                        if (workitem.nextrun > 0) {
                            workitem.nextrun = new Date(workitem.nextrun * 1000);
                        } else {
                            delete workitem.nextrun;
                        }
                        if (workitem.lastrun > 0) {
                            workitem.lastrun = new Date(workitem.lastrun * 1000);
                        } else {
                            delete workitem.lastrun;
                        }
                        try {
                            if (workitem.payload != null && workitem.payload != "") {
                                workitem.payload = JSON.parse(workitem.payload);
                            }
                        } catch (error) {
                        }
                        return resolve(workitem);
                    }
                    resolve(null);
                }
                this.trace('free_update_workitem_response');
                this.lib.free_update_workitem_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.update_workitemCallback));
            this.trace('call update_workitem_async');
            this.lib.update_workitem_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('UpdateWorkitem async failed'));
                }
            });

        });

    }

    delete_workitem(id) {
        this.trace('delete_workitem invoked');
        const req = {
            id: id
        };
        this.trace('call delete_workitem');
        const response = this.lib.delete_workitem(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, DeleteWorkitemResponseWrapper);
        this.trace('free_delete_workitem_response');
        this.lib.free_delete_workitem_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
    }
    delete_workitem_async(id) {
        this.trace('delete_workitem async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                id: id
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('delete_workitem_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, DeleteWorkitemResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve();
                }
                this.trace('free_delete_workitem_response');
                this.lib.free_delete_workitem_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.delete_workitemCallback));
            this.trace('call delete_workitem_async');
            this.lib.delete_workitem_async(this.client, req, cb, (err) => {
                if (err) {
                    reject(new ClientError('DeleteWorkitem async failed'));
                }
            });
        });
    }
    watches = {}
    next_watch_interval = 200;
/**
 * @typedef {object} WatchOptions
 * @property {string}   collectionname  The name of the collection to watch.
 * @property {string[]} [paths=[]]      Specific document paths to filter on.
 */

/**
 * @typedef {object} WatchEvent
 * @property {string} id          The document’s unique ID.
 * @property {string} operation   The type of change ("insert", "update", "delete", etc.).
 * @property {any}    document    The JSON‑parsed document payload.
 */

/**
 * @callback WatchCallback
 * @param {WatchEvent} event         The watch event.
 * @param {number}     event_counter How many events have fired so far.
 * @returns {void}   Throw or log inside if you need to handle errors.
 */

/**
 * Start watching a collection.
 *
 * @param {WatchOptions} options
 * @param {WatchCallback} callback
 * @returns {string}  The watch ID you can later cancel.
 */
    watch({ collectionname, paths = null }, callback) {
        this.trace('watch invoked');
        const req = {
            collectionname: collectionname,
            paths: paths == null ? null : this.stringify(paths)
        }
        this.trace('call watch');
        const response = this.lib.watch(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, WatchResponseWrapper);
        this.trace('free_watch_response');
        this.lib.free_watch_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let watchid = result.watchid;
        let event_counter = 0;
        this.watches[watchid] = setInterval(() => {
            if (this.connected == false) {
                this.trace('No longer connected, so clearInterval for watchid', watchid);
                clearInterval(this.watches[watchid]);
                delete this.watches[watchid];
                return;
            }
            let hadone = false;
            do {
                this.trace('call next');
                const responsePtr = this.lib.next_watch_event(watchid);
                this.trace('decode response');
                const result = koffi.decode(responsePtr, WatchEventWrapper);
                if (result.id != null && result.id != "") {
                    hadone = true;
                    event_counter++;
                    let event = {
                        id: result.id,
                        operation: result.operation,
                        document: JSON.parse(result.document),
                    }
                    this.trace('call next had result', event_counter, event);
                    try {
                        callback(event, event_counter);
                    } catch (error) {
                        this.error('Error in watch event callback', error);
                    }
                } else {
                    hadone = false;
                }
                this.trace('free_watch_event');
                this.lib.free_watch_event(responsePtr);
            } while (hadone);
        }, this.next_watch_interval);

        return watchid;
    }
    clientevents = {}
    next_watch_interval = 1000;
    on_client_event(callback) {
        this.trace('on_client_event invoked');
        this.trace('call on_client_event');
        const response = this.lib.on_client_event(this.client);
        this.trace('decode response');
        const result = koffi.decode(response, ClientEventResponseWrapper);
        this.trace('free_event_response');
        this.lib.free_event_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let eventid = result.eventid;
        let event_counter = 0;
        this.info('on_client_event eventid', eventid);
        this.clientevents[eventid] = setInterval(() => {
            if (this.connected == false) {
                this.trace('No longer connected, so clearInterval for eventid', eventid);
                clearInterval(this.clientevents[eventid]);
                delete this.clientevents[eventid];
                return;
            }
            let hadone = false;
            do {
                this.trace('call next');
                const responsePtr = this.lib.next_client_event(eventid);
                this.trace('decode response');
                const result = koffi.decode(responsePtr, ClientEventWrapper);
                if (result.event != null && result.event != "") {
                    hadone = true;
                    event_counter++;
                    let event = {
                        event: result.event,
                        reason: result.reason
                    }
                    this.trace('call next had result', event_counter, event);
                    try {
                        callback(event, event_counter);
                    } catch (error) {
                        this.error('Error in client event callback', error);
                    }
                } else {
                    hadone = false;
                }
                this.trace('free_client_event');
                this.lib.free_client_event(responsePtr);
            } while (hadone);
        }, this.next_watch_interval);

        return eventid;
    }
    off_client_event(eventid) {
        this.trace('off_client_event invoked');
        const response = this.lib.off_client_event(eventid);
        this.trace('decode response');
        const result = koffi.decode(response, OffClientEventResponseWrapper);
        this.trace('free_off_event_response');
        this.lib.free_off_event_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        if (this.clientevents[eventid] != null) {
            this.trace('clearInterval for eventid', eventid);
            clearInterval(this.clientevents[eventid]);
            delete this.clientevents[eventid];
        }
    }
    event_refs = {};
    uniqeid() {
        return Math.random().toString(36).substr(2, 9);
    }
    watch_async({ collectionname, paths = [] }, callback) {
        throw new Error('Not implemented');
        this.trace('watch invoked');
        const req = {
            collectionname: collectionname,
            paths: this.stringify(paths)
        }
        let event_counter = 0;
        const event_callback = (responsePtr) => {
            event_counter++;
            this.trace('watch_async event callback');
            const response = koffi.decode(responsePtr, WatchEventWrapper);
            let event = {
                id: response.id,
                operation: response.operation,
                document: JSON.parse(response.document),
            }
            this.trace('free_watch_event');
            this.lib.free_watch_event(responsePtr);
            try {
                callback(event, event_counter);
                this.trace('event #', event_counter, ' callback done');
            } catch (error) {
                this.error('Error in watch event callback', error);
            }
        }
        const event_cb = koffi.register(event_callback, koffi.pointer(this.lib.WatchEventCallback));

        this.trace('call watch');
        const response = this.lib.watch(this.client, req, event_cb);
        this.trace('decode response');
        const result = koffi.decode(response, WatchResponseWrapper);
        this.trace('free_watch_response');
        this.lib.free_watch_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let watchid = result.watchid;

        return watchid;
    }
    watch_async_async({ collectionname, paths = [] }, callback) {
        throw new Error('Not implemented');
        this.trace('watch async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                paths: this.stringify(paths)
            };
            const callback = (responsePtr) => {
                this.trace('watch_async callback');
                const response = koffi.decode(responsePtr, WatchResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.watchid);
                }
                this.trace('free_watch_response');
                this.lib.free_watch_response(responsePtr);
            };
            let event_counter = 0;
            const event_callback = (responsePtr) => {
                event_counter++;
                this.trace('watch_async event callback');
                const response = koffi.decode(responsePtr, WatchEventWrapper);
                let event = {
                    id: response.id,
                    operation: response.operation,
                    document: JSON.parse(response.document),
                }
                this.trace('free_watch_event');
                this.lib.free_watch_event(responsePtr);
                try {
                    callback(event, event_counter);
                } catch (error) {
                    this.error('Error in watch event callback', error);
                }
            }
            const cb = koffi.register(callback, koffi.pointer(this.lib.watchCallback));
            const event_cb = koffi.register(event_callback, koffi.pointer(this.lib.WatchEventCallback));

            // this.event_refs[this.uniqeid()] = { event_callback, event_cb };

            this.trace('call watch_async');
            this.lib.watch_async(this.client, req, cb, event_cb, (err) => {
                if (err) {
                    reject(new ClientError('Watch failed'));
                }
            });
        });
    }
    unwatch(watchid) {
        this.trace('unwatch invoked');
        const response = this.lib.unwatch(this.client, watchid);
        this.trace('decode response');
        const result = koffi.decode(response, UnwatchResponseWrapper);
        this.trace('free_unwatch_response');
        this.lib.free_unwatch_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        if (this.watches[watchid] != null) {
            this.trace('clearInterval for watchid', watchid);
            clearInterval(this.watches[watchid]);
            delete this.watches[watchid];
        }
    }
    queues = {}
    next_queue_interval = 200;
/**
 * @typedef {object} QueueEvent
 * @property {string} queuename       The name of the queue that produced this event.
 * @property {string} correlation_id   The ID you can use to correlate replies.
 * @property {string} replyto          The queue name to reply to (if any).
 * @property {string} routingkey       The routing key from the broker.
 * @property {string} exchangename     The exchange this event came from.
 * @property {any}    data             The payload (already JSON‑parsed if valid).
 * @property {string} [jwt]            A stripped‑out JWT, if one was present.
 * @property {any}    [user]           A stripped‑out user object, if one was present.
 */
/**
 * @callback QueueCallback
 * @param {QueueEvent} event   The fully‑typed event object.
 * @returns {Promise<any>|any}  If you return something and `event.replyto` is set, it'll be sent.
 */
/**
 * Register a queue and start pumping events into your callback.
 *
 * @param {{ queuename: string }} options            Options bag.
 * @param {QueueCallback}       callback            Called for each event.
 * @returns {string}            The (possibly‑rewritten) queue name.
 */
    register_queue({ queuename }, callback ) {
        this.trace('register queue invoked');
        const req = {
            queuename: queuename
        };
        this.trace('call register_queue');
        const response = this.lib.register_queue(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, RegisterQueueResponseWrapper);
        this.trace('free_register_queue_response');
        this.lib.free_register_queue_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        queuename = result.queuename;
        const id = this.uniqeid();
        this.queues[id] = {
            interval:
                setInterval(async () => {
                    if (this.connected == false) {
                        clearInterval(this.queues[id].interval);
                        delete this.queues[id];
                        return;
                    }
                    let hadone = false;
                    do {
                        hadone = false;
                        this.trace('call next queue event');
                        const responsePtr = this.lib.next_queue_event(queuename);
                        this.trace('decode response');
                        const result = koffi.decode(responsePtr, QueueEventWrapper);
                        if (result.queuename != null && result.queuename != "") {
                            hadone = true;
                            let data = result.data;
                            let jwt = undefined;
                            let user = undefined;
                            if (data != null && data != "") {
                                try {
                                    data = JSON.parse(data);
                                } catch (error) {
                                }
                            }
                            if (data.__jwt != null) {
                                jwt = data.__jwt;
                                delete data.__jwt;
                            }
                            if (data.__user != null) {
                                user = data.__user;
                                delete data.__user;
                            }
                            let event = {
                                queuename: result.queuename,
                                correlation_id: result.correlation_id,
                                replyto: result.replyto,
                                routingkey: result.routingkey,
                                exchangename: result.exchangename,
                                jwt: jwt,
                                user: user,
                                data: data,
                            }
                            this.trace('call next had result', event);
                            let cbresult = await callback(event);
                            if(cbresult != null && cbresult != "" && event.replyto != null && event.replyto != "") {
                                if(typeof cbresult === 'object') {
                                    cbresult = JSON.stringify(cbresult);
                                }
                                await this.queue_message({
                                    queuename: event.replyto,
                                    correlation_id: event.correlation_id,
                                    striptoken: true,
                                    data: cbresult
                                });
                            }
                        }

                        this.trace('free_queue_event');
                        this.lib.free_queue_event(responsePtr);
                    } while (hadone);
                }, this.next_queue_interval),
            queuename
        };
        return result.queuename;
    }

    /**
     * Register a queue asynchronously and receive events via callback.
     *
     * @param {{ queuename: string }} options
     * @param {function(event: object): (Promise<any>|any)} callback
     * @returns {Promise<string>} Resolves to the queue name.
     */
    register_queue_async({ queuename }, callback) {
        this.trace('register_queue_async invoked');
        return new Promise((resolve, reject) => {
            const req = { queuename: queuename };
            const id = this.uniqeid();
            const event_callback = koffi.register((eventPtr) => {
                console.log("event_callback!")
                try {
                    const result = koffi.decode(eventPtr, QueueEventWrapper);
                    let data = result.data;
                    if (data != null && data !== "") {
                        try { data = JSON.parse(data); } catch (e) {}
                    }
                    let event = {
                        queuename: result.queuename,
                        correlation_id: result.correlation_id,
                        replyto: result.replyto,
                        routingkey: result.routingkey,
                        exchangename: result.exchangename,
                        data: data,
                    };
                    let cbresult = callback(event);
                    console.log('register_queue_async callback', event, cbresult);
                    if (cbresult != null && cbresult !== "" && event.replyto) {
                        if (typeof cbresult === 'object') cbresult = JSON.stringify(cbresult);
                        const buf = Buffer.from(cbresult + '\0', 'utf8');
                        return buf;
                        // return cbresult;
                    }
                    // Return null if no reply
                    return null;
                } catch (err) {
                    this.error('register_queue_async event_callback error', err);
                    return null;
                }
            }, koffi.pointer(this.lib.QueueEventCallback));
            try {
                const response = this.lib.register_queue_async(this.client, req, event_callback);
                const result = koffi.decode(response, RegisterQueueResponseWrapper);
                this.lib.free_register_queue_response(response);
                if (!result.success) {
                    reject(new ClientError(result.error));
                    return;
                }
                queuename = result.queuename;
                this.queues[id] = { queuename, event_callback };
                resolve(queuename);
            } catch (err) {
                reject(err);
            }
        });
    }
    // register_queue_async({ queuename }, callback) {
    //     this.trace('register_queue_async invoked');
    //     return new Promise((resolve, reject) => {
    //         const req = { queuename: queuename };
    //         const id = this.uniqeid();
    //         const event_callback = koffi.register((eventPtr) => {
    //             try {
    //                 const result = koffi.decode(eventPtr, QueueEventWrapper);
    //                 let data = result.data;
    //                 if (data != null && data !== "") {
    //                     try { data = JSON.parse(data); } catch (e) {}
    //                 }
    //                 let event = {
    //                     queuename: result.queuename,
    //                     correlation_id: result.correlation_id,
    //                     replyto: result.replyto,
    //                     routingkey: result.routingkey,
    //                     exchangename: result.exchangename,
    //                     data: data,
    //                 };
    //                 let cbresult;
    //                 try {
    //                     cbresult = callback(event);
    //                 } catch (err) {
    //                     this.error('register_queue_async user callback error', err);
    //                     return null;
    //                 }
    //                 // If callback returns a Promise (async), handle it
    //                 if (cbresult && typeof cbresult.then === 'function') {
    //                     cbresult.then(resolved => {
    //                         if (resolved != null && resolved !== "" && event.replyto) {
    //                             let msg = resolved;
    //                             if (typeof msg === 'object') msg = JSON.stringify(msg);
    //                             // Call queue_message asynchronously
    //                             this.queue_message({
    //                                 queuename: event.replyto,
    //                                 correlation_id: event.correlation_id,
    //                                 striptoken: true,
    //                                 data: msg
    //                             });
    //                         }
    //                     }).catch(err => {
    //                         this.error('register_queue_async async callback error', err);
    //                     });
    //                     return null; // Return null to native immediately
    //                 } else {
    //                     if (cbresult != null && cbresult !== "" && event.replyto) {
    //                         if (typeof cbresult === 'object') cbresult = JSON.stringify(cbresult);
    //                         return cbresult;
    //                     }
    //                     return null;
    //                 }
    //             } catch (err) {
    //                 this.error('register_queue_async event_callback error', err);
    //                 return null;
    //             }
    //         }, koffi.pointer(this.lib.QueueEventCallback));
    //         try {
    //             const response = this.lib.register_queue_async(this.client, req, event_callback);
    //             const result = koffi.decode(response, RegisterQueueResponseWrapper);
    //             this.lib.free_register_queue_response(response);
    //             if (!result.success) {
    //                 reject(new ClientError(result.error));
    //                 return;
    //             }
    //             queuename = result.queuename;
    //             this.queues[id] = { queuename, event_callback };
    //             resolve(queuename);
    //         } catch (err) {
    //             reject(err);
    //         }
    //     });
    // }

/**
 * @typedef {object} RegisterExchangeOptions
 * @property {string} exchangename    The exchange to bind.
 * @property {string} [algorithm="fanout"]  Exchange algorithm (e.g. "fanout", "direct", "topic").
 * @property {string} [routingkey=""]      Default routing key to use.
 * @property {boolean} [addqueue=true]     Whether to auto‑create a queue for this exchange.
 */

/**
 * @typedef {object} ExchangeEvent
 * @property {string} queuename       The name of the queue that produced this event.
 * @property {string} correlation_id   Correlation ID for replies.
 * @property {string} replyto          Queue name to send replies to.
 * @property {string} routingkey       The routing key of this message.
 * @property {string} exchangename     The exchange name that emitted this event.
 * @property {any}    data             The raw payload.
 */

/**
 * @callback ExchangeCallback
 * @param {ExchangeEvent} event  The decoded exchange event.
 * @returns {void|Promise<void>}  If the handler is async, return a Promise.
 */

/**
 * Register an exchange (and auto‑create a queue if requested).
 *
 * @param {RegisterExchangeOptions} options
 * @param {ExchangeCallback}       callback
 * @returns {string}  The (possibly‑rewritten) queue name for this exchange.
 */
    register_exchange({ exchangename, algorithm = "fanout", routingkey = "", addqueue = true }, callback) {
        this.trace('register exchange invoked');
        if (exchangename == null || exchangename == "") throw new ClientError('exchangename is required');
        if (algorithm == null) algorithm = "";
        if (routingkey == null) routingkey = "";
        if (addqueue == null) addqueue = true;
        const req = {
            exchangename: exchangename,
            algorithm: algorithm,
            routingkey: routingkey,
            addqueue: addqueue
        };
        this.trace('call register_exchange');
        const response = this.lib.register_exchange(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, RegisterExchangeResponseWrapper);
        this.trace('free_register_exchange_response');
        this.lib.free_register_exchange_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let queuename = result.queuename;
        if (queuename != null && queuename != "") {
            const id = this.uniqeid();
            this.queues[id] = {
                interval:
                    setInterval(() => {
                        if (this.connected == false) {
                            clearInterval(this.queues[id].interval);
                            delete this.queues[id];
                            return;
                        }
                        let hadone = false;
                        do {
                            hadone = false;
                            this.trace('call next queue event');
                            const responsePtr = this.lib.next_queue_event(queuename);
                            this.trace('decode response');
                            const result = koffi.decode(responsePtr, QueueEventWrapper);
                            if (result.queuename != null && result.queuename != "") {
                                hadone = true;
                                let event = {
                                    queuename: result.queuename,
                                    correlation_id: result.correlation_id,
                                    replyto: result.replyto,
                                    routingkey: result.routingkey,
                                    exchangename: result.exchangename,
                                    data: result.data,
                                }
                                this.trace('call next had result', event);
                                callback(event);
                            }
                            this.trace('free_queue_event');
                            this.lib.free_queue_event(responsePtr);
                        } while (hadone);
                    }, this.next_queue_interval),
                queuename
            };
        }
        return result.queuename;
    }
    rpc({ queuename, data = {}, striptoken = false, timeout = -1 }) {
        this.trace('rpc invoked');
        const req = {
            queuename: queuename,
            striptoken: striptoken,
            data: this.stringify(data)
        };
        this.trace('call rpc');
        const response = this.lib.rpc(this.client, req, timeout);
        this.trace('decode response');
        const result = koffi.decode(response, RpcResponseWrapper);
        this.trace('free_rpc_response');
        this.lib.free_rpc_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let payload = result.result;
        try {
            payload = JSON.parse(payload);
        } catch (error) {            
        }
        return payload;
    }
    rpc_async({ queuename, data = {}, striptoken = true, timeout = -1 }) {
        this.trace('rpc_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                queuename: queuename,
                striptoken: striptoken,
                data: this.stringify(data)
            };
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.trace('rpc_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, RpcResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    let payload = response.result;
                    try {
                        payload = JSON.parse(payload);
                    } catch (error) {
                    }
                    resolve(payload);
                }
                this.trace('free_rpc_response');
                this.lib.free_rpc_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.rpcCallback));
            this.trace('call rpc_async');
            this.lib.rpc_async(this.client, req, cb, timeout, (err) => {
                if (err) {
                    reject(new ClientError('Rpc async failed'));
                }
            });
        });
    }
    unregister_queue(queuename) {
        const reqptr = this.lib.unregister_queue(this.client, queuename);
        this.trace('decode response');
        const result = koffi.decode(reqptr, UnRegisterQueueResponseWrapper);
        this.trace('free_unregister_queue_response');
        this.lib.free_unregister_queue_response(reqptr);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let keys = Object.keys(this.queues);
        for (let i = 0; i < keys.length; i++) {
            if (this.queues[keys[i]].queuename == queuename) {
                clearInterval(this.queues[keys[i]].interval);
                delete this.queues[keys[i]];
            }
        }
    }
    queue_message({ queuename, data = {}, replyto = "", exchangename = "", correlation_id = "", routingkey = "", striptoken = false, expiration = 0 }) {
        this.trace('queue message invoked');
        if (queuename == null || queuename == "") {
            if (exchangename == null || exchangename == "") {
                throw new ClientError('queuename or exchangename is required');
            }
        }
        if (replyto == null) replyto = "";
        if (exchangename == null) exchangename = "";
        if (correlation_id == null) correlation_id = "";
        if (routingkey == null) routingkey = "";
        if (striptoken == null) striptoken = false;
        if (expiration == null) expiration = 0;
        const req = {
            queuename: queuename,
            data: this.stringify(data),
            replyto: replyto,
            exchangename: exchangename,
            correlation_id: correlation_id,
            routingkey: routingkey,
            striptoken: striptoken,
            expiration: expiration
        };
        this.trace('call queue_message');
        const response = this.lib.queue_message(this.client, req);
        this.trace('decode response');
        const result = koffi.decode(response, QueueMessageResponseWrapper);
        this.trace('free_queue_message_response');
        this.lib.free_queue_message_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
    }

    invoke_openrpa({ robotid, workflowid, payload, rpc = true, timeout = -1 }) {
        this.trace('invoke_openrpa invoked');
        if (robotid == null || robotid == "") {
            throw new ClientError('robotid is required');
        }
        if (workflowid == null || workflowid == "") {
            throw new ClientError('workflowid is required');
        }
        if (payload == null) payload = {};
        if (rpc == null) rpc = true;
        const req = {
            robotid: robotid,
            workflowid: workflowid,
            payload: this.stringify(payload),
            rpc: rpc
        };
        this.trace('call invoke_openrpa');
        const response = this.lib.invoke_openrpa(this.client, req, timeout);
        this.trace('decode response');
        const result = koffi.decode(response, InvokeOpenRPAResponseWrapper);
        this.trace('free_invoke_openrpa_response');
        this.lib.free_invoke_openrpa_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let rparesult = result.result || "{}";
        try {
            rparesult = JSON.parse(rparesult);
        } catch (error) {            
        }
        return rparesult;
    }
}

module.exports = {
    Client,
    ClientError,
    LibraryLoadError,
    ClientCreationError
};
