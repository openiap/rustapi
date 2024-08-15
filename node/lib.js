const koffi = require('koffi');
const path = require('path');
const fs = require('fs');
const { log } = require('console');

const CString = 'char*';
const voidPtr = koffi.pointer('void');
const bool = koffi.types.bool;
const int = 'int';
const uint64 = 'uint64_t';
const size_t = 'size_t';

function encodeStruct(value, type) {
    const buf = Buffer.allocUnsafe(koffi.sizeof(type));
    koffi.encode(buf, type, value);
    return buf;
}
// const CStringArray = ArrayType(CString);

const ClientWrapper = koffi.struct('ClientWrapper', {
    success: 'bool',
    error: 'char*'
});
const ClientWrapperPtr = koffi.pointer(ClientWrapper);

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

const AggregateRequestWrapper = koffi.struct('AggregateRequestWrapper',{
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
    results_count: size_t,
    error: CString
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
    result: CString,
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
    // Uint8Array
    file: 'uint8_t *',
    // file: 'uint8_t *', 
    // file: koffi.array(koffi.types.uint8_t, koffi.sizeof(koffi.types.uintptr_t))
    // file: 'uint8 ***',
    // file: ArrayType(ref.types.uint8) // This represents Vec<u8> in Rust
});
const WorkitemFileWrapperPtr = koffi.pointer(WorkitemFileWrapper);
// const WorkitemFileWrapperPtrArray = ArrayType(WorkitemFileWrapperPtr);

const WorkitemWrapper = koffi.struct('WorkitemWrapper', {
    id: CString,
    name: CString,
    payload: CString,
    priority: int,
    nextrun: uint64,
    lastrun: uint64,
    /// files: koffi.pointer(WorkitemFileWrapperPtrArray),
    // files: WorkitemFileWrapperPtrArray,
    files: 'WorkitemFileWrapper **',
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
    error: CString
});
const PushWorkitemResponseWrapperPtr = koffi.pointer(PushWorkitemResponseWrapper);

const PopWorkitemRequestWrapper = koffi.struct('PopWorkitemRequestWrapper', {
    wiq: CString,
    wiqid: CString,
});
const PopWorkitemRequestWrapperPtr = koffi.pointer(PopWorkitemRequestWrapper);
const PopWorkitemResponseWrapper = koffi.struct('PopWorkitemResponseWrapper', {
    success: bool,
    error: CString,
    workitem: WorkitemWrapperPtr
});
const PopWorkitemResponseWrapperPtr = koffi.pointer(PopWorkitemResponseWrapper);


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
    console.log(`Platform: ${platform}, Arch: ${arch}`);
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
                    libPath = path.join(libDir, 'openiap-windows-x86.dll'); break;
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
    console.log(`Using library: ${libPath}`);

    try {
        const lib = koffi.load(libPath);

        lib.enable_tracing = lib.func('void enable_tracing(const char* rust_log, const char* tracing)');
        lib.disable_tracing = lib.func('void disable_tracing()');
        lib.connect = lib.func('client_connect', ClientWrapperPtr, ['str']);
        lib.ConnectCallback = koffi.proto('void ConnectCallback(ClientWrapper*)');
        lib.connect_async = lib.func('connect_async', 'void', ['str', koffi.pointer(lib.ConnectCallback)]);
        lib.free_client = lib.func('void free_client(ClientWrapper*)');
        lib.signin = lib.func('signin', SigninResponseWrapperPtr, [ClientWrapperPtr, SigninRequestWrapperPtr]);
        lib.signinCallback = koffi.proto('void signinCallback(SigninResponseWrapper*)');
        lib.signin_async = lib.func('signin_async', 'void', [ClientWrapperPtr, SigninRequestWrapperPtr, koffi.pointer(lib.signinCallback)]);
        lib.free_signin_response = lib.func('free_signin_response', 'void', [SigninResponseWrapperPtr]);
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
        // lib.pop_workitem_async = lib.func('pop_workitem_async', 'void', [ClientWrapperPtr, PopWorkitemRequestWrapperPtr, koffi.pointer(lib.pop_workitemCallback)]);
        lib.free_pop_workitem_response = lib.func('free_pop_workitem_response', 'void', [PopWorkitemResponseWrapperPtr]);

        
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

var ref;

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
    }
    tracing = false;
    informing = true;
    verbosing = false;
    connected = false;
    free() {
        if (this.client) {
            this.verbose('this.client not null, call free_client');
            this.lib.free_client(this.client);
            this.client = null;
        }
        this.connected = false;
        this.trace('free::end');
    }

    enable_tracing(rust_log = '', tracing = '') {
        this.verbose('enable_tracing invoked', rust_log, tracing);
        this.lib.enable_tracing(rust_log, tracing);
        this.informing = true;
        if(rust_log.indexOf('verbose') > -1) this.verbosing = true;
        if(rust_log.indexOf('trace') > -1) this.tracing = true;
        this.trace('enable_tracing called');
    }
    disable_tracing() {
        this.lib.disable_tracing();
    }
    info(...args) {
        if(this.informing == true) console.log('Node.js:', ...args);
    }
    verbose(...args) {
        if(this.verbosing == true) console.log('Node.js:', ...args);
    }
    trace(...args) {
        if(this.tracing == true) console.log('Node.js:', ...args);
    }
    async connect(url) {
        this.verbose('connect invoked', url);
        this.connected = false;
        const _clientWrapperPtr = this.lib.connect(url);
        if (_clientWrapperPtr === 0) {
            this.trace('Received a null pointer from Rust function');
            throw new Error('Received a null pointer from Rust function');
        }
        this.trace('Callback invoked');
        const clientWrapper = koffi.decode(_clientWrapperPtr,ClientWrapper);

        this.connected = true;
        this.client = _clientWrapperPtr;
        return clientWrapper;
    }
    test_add() {
        return new Promise(async (resolve, reject) => {
            console.log("test_add::begin");
            const TransferCallback = koffi.proto('TransferCallback', 'void', ['int']);
            const addAsync = this.lib.func('add_async', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
            function callback(value) {
                console.log("callback with value", value);
                resolve();
            }
            const cb = koffi.register(callback, koffi.pointer(TransferCallback));
            addAsync.async(2, 2, cb, (err, res) => {
                console.log(err, res); // why is this not called?
            });
        });
    }
    test_add2() {
        return new Promise(async (resolve, reject) => {
            console.log("test_add::begin");
            const TransferCallback = koffi.proto('TransferCallback2', 'void', ['int']);
            const addAsync = this.lib.func('add_async2', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
            function callback(value) {
                console.log("callback with value", value);
                resolve();
            }
            const cb = koffi.register(callback, koffi.pointer(TransferCallback));
            addAsync.async(2, 2, cb, (err, res) => {
                console.log(err, res); // why is this not called?
            });
        });
    }

    connect_async(url) {
        this.verbose('connect_async invoked', url);
        this.connected = false;
        return new Promise((resolve, reject) => {
            try {
                const cb = koffi.register((wrapper) => {
                    this.trace('Callback invoked');
                    try {
                        if (wrapper === 0) {
                            throw new Error('Received a null pointer from Rust function');
                        }
                        const clientWrapper = koffi.decode(wrapper, ClientWrapper);
                        this.client = wrapper;
                        if (!clientWrapper.success) {
                            reject(new ClientCreationError(clientWrapper.error));
                        } else {
                            this.connected = true;
                            resolve(clientWrapper);
                        }
                    } catch (error) {
                        reject(new ClientCreationError(error.message));
                    } 
                }, koffi.pointer(this.lib.ConnectCallback));
                this.verbose('call connect_async');
                this.lib.connect_async(url, cb);                
            } catch (error) {
                reject(new ClientCreationError(error.message));
            }
        });
    }

    signin({ username = '', password = '', jwt = '', agent = '', version = '', longtoken = false, validateonly = false, ping = false } = {}) {
        this.verbose('signin invoked');
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
        const reqptr = encodeStruct(req, SigninRequestWrapper);

        this.trace('call signin');
        const response = this.lib.signin(this.client, reqptr);
        this.trace('decode response');
        const result = koffi.decode(response,SigninResponseWrapper);
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
        this.verbose('signin invoked');
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
            const reqptr = encodeStruct(req, SigninRequestWrapper);
    
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.verbose('signin_async callback');
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
            this.lib.signin_async(this.client, reqptr, cb, (err, res) => {
                console.log('signin_async error', err, res);
                if (err) {
                    reject(new ClientError('Signin failed'));
                }
            });
        });
    }

    query({ collectionname, query, projection = "", orderby = "", skip = 0, top = 100, queryas = "", explain = false }) {
        this.verbose('query invoked');
        const req = {
            collectionname: collectionname,
            query: query,
            projection: projection,
            orderby: orderby,
            queryas: queryas,
            explain: explain,
            skip: skip,
            top: top
        };
        const reqptr = encodeStruct(req, QueryRequestWrapper);
        this.trace('call query');
        const responsePtr = this.lib.query(this.client, reqptr);
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

    query_async({ collectionname, query, projection = "", orderby = "", skip = 0, top = 100, queryas = "", explain = false }) {
        this.trace('query_async invoked');
        return new Promise((resolve, reject) => {
            const req = {            
                collectionname: collectionname,
                query: query,
                projection: projection,
                orderby: orderby,
                queryas: queryas,
                explain: explain,
                skip: skip,
                top: top
            };
            const reqptr = encodeStruct(req, QueryRequestWrapper);
            const callback = (responsePtr) => {
                this.trace('query_async callback');
                const response = koffi.decode(responsePtr, QueryResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.verbose('free_query_response');
                this.lib.free_query_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.queryCallback));
            this.trace('call query_async');
            this.lib.query_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Query failed'));
                }
            });
        });
    }
    aggregate({ collectionname, aggregates = "[]", queryas = "", hint = "", explain = false }) {
        this.verbose('aggregate invoked');
        const req = {
            collectionname: collectionname,
            aggregates: aggregates,
            queryas: queryas,
            hint: hint,
            explain: explain
        };
        const reqptr = encodeStruct(req, AggregateRequestWrapper);
        this.verbose('call aggregate');
        const response = this.lib.aggregate(this.client, reqptr);
        const result = koffi.decode(response, AggregateResponseWrapper);
        this.verbose('free_aggregate_response');
        this.lib.free_aggregate_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.results);
    }
    aggregate_async({ collectionname, aggregates, queryas = "", hint = "", explain = false }) {
        this.verbose('aggregate invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                aggregates: aggregates,
                queryas: queryas,
                hint: hint,
                explain: explain
            };
            const reqptr = encodeStruct(req, AggregateRequestWrapper);
            this.verbose('create callback');
            const callback = (responsePtr) => {
                this.verbose('aggregate_async callback');
                const response = koffi.decode(responsePtr, AggregateResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.verbose('free_aggregate_response');
                this.lib.free_aggregate_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.aggregateCallback));

            this.verbose('call aggregate_async');
            this.lib.aggregate_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Aggregate failed'));
                }
            });
        });
    }
    count({ collectionname, query = "", queryas = "", explain = false}) {
        this.verbose('count invoked');
        const req = {
            collectionname: collectionname,
            query: query,
            queryas: queryas,
            explain: explain
        };
        const reqptr = encodeStruct(req, CountRequestWrapper);
        this.trace('call count');
        const response = this.lib.count(this.client, reqptr);
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
    count_async({ collectionname, query = "", queryas = "", explain = false}) {
        this.verbose('count async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                query: query,
                queryas: queryas,
                explain: explain
            };
            const reqptr = encodeStruct(req, CountRequestWrapper);
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.verbose('count_async callback');
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
            this.lib.count_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Count failed'));
                }
            });
        });
    }
    distinct({ collectionname, field, query = "", queryas = "", explain = false }) {
        this.verbose('distinct invoked');
        const req = {
            collectionname: collectionname,
            field: field,
            query: query,
            queryas: queryas,
            explain: explain
        };
        const reqptr = encodeStruct(req, DistinctRequestWrapper);
        this.verbose('call distinct');
        const responsePtr = this.lib.distinct(this.client, reqptr);
        this.trace('decode response');
        const response = koffi.decode(responsePtr, DistinctResponseWrapper);
        let results = [];
        this.trace('decode response results');
        let strings = koffi.decode(response.results, 'void *', -1);
        for(let i = 0; i < response.results_count; i++) {
            this.trace('decode response results #', i);
            let ptr = strings[i];
            let value = koffi.decode(ptr, 'char', -1);
            results.push(value.toString());
        };
        this.verbose('free_distinct_response');
        this.lib.free_distinct_response(responsePtr);
        if (!response.success) {
            const errorMsg = response.error;
            throw new ClientError(errorMsg);
        }
        return results;
    }
    distinct_async({ collectionname, field, query = "", queryas = "", explain = false }) {
        this.verbose('distinct invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                field: field,
                query: query,
                queryas: queryas,
                explain: explain
            };
            const reqptr = encodeStruct(req, DistinctRequestWrapper);
            this.verbose('create callback');
            const callback = (responsePtr) => {
                this.verbose('distinct_async callback');
                const response = koffi.decode(responsePtr, DistinctResponseWrapper);
                let results = [];
                let strings = koffi.decode(response.results, 'void *', -1);
                for(let i = 0; i < response.results_count; i++) {
                    let ptr = strings[i];
                    let value = koffi.decode(ptr, 'char', -1);
                    results.push(value.toString());
                };
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(results);
                }
                this.verbose('free_distinct_response');
                this.lib.free_distinct_response(responsePtr);
            };
            const cb = koffi.register(callback, koffi.pointer(this.lib.distinctCallback));

            this.verbose('call distinct_async');
            this.lib.distinct_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Distinct failed'));
                }
            });
        });
    }
    insert_one({ collectionname, document, w = 1, j = false }) {
        this.verbose('insert_one invoked');
        const req = {
            collectionname: collectionname,
            item: document,
            w: w,
            j: j
        };
        const reqptr = encodeStruct(req, InsertOneRequestWrapper);
        this.trace('call insert_one');
        const response = this.lib.insert_one(this.client, reqptr);
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
    insert_one_async({ collectionname, document, w = 1, j = false }) {
        this.verbose('insert_one async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: document,
                w: w,
                j: j
            };
            const reqptr = encodeStruct(req, InsertOneRequestWrapper);
            const callback = (responsePtr) => {
                this.verbose('insert_one_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, InsertOneResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.verbose('free_insert_one_response');
                this.lib.free_insert_one_response(responsePtr);
            }
            const cb = koffi.register(callback, koffi.pointer(this.lib.insert_oneCallback));
            this.verbose('call insert_one_async');
            this.lib.insert_one_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertOne failed'));
                }
            });
        });
    };
    insert_many({ collectionname, documents, w = 1, j = false, skipresults = false }) {
        this.verbose('insert_many invoked');
        const req = {
            collectionname: collectionname,
            items: documents,
            w: w,
            j: j,
            skipresults: skipresults
        };
        const reqptr = encodeStruct(req, InsertManyRequestWrapper);
        this.trace('call insert_many');
        const response = this.lib.insert_many(this.client, reqptr);
        this.trace('decode response');
        const result = koffi.decode(response, InsertManyResponseWrapper);
        this.trace('free_insert_many_response');
        this.lib.free_insert_many_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_many_async({ collectionname, documents, w = 1, j = false, skipresults = false }) {
        this.verbose('insert_many invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                items: documents,
                w: w,
                j: j,
                skipresults: skipresults
            };
            const reqptr = encodeStruct(req, InsertManyRequestWrapper);
            this.verbose('create callback');
            const callback = (responsePtr) => {
                this.verbose('insert_many_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, InsertManyResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.verbose('free_insert_many_response');
                this.lib.free_insert_many_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.insert_manyCallback));
            this.verbose('call insert_many_async');
            this.lib.insert_many_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertMany failed'));
                }
            });
        });
    }
    update_one({ collectionname, item, w = 1, j = false }) {
        this.info('update_one invoked');
        const req = {
            collectionname: collectionname,
            item: item,
            w: w,
            j: j            
        };
        const reqptr = encodeStruct(req, UpdateOneRequestWrapper);
        this.info('call update_one');
        const response = this.lib.update_one(this.client, reqptr);
        this.info('decode response');
        const result = koffi.decode(response, UpdateOneResponseWrapper);
        this.info('free_update_one_response');
        this.lib.free_update_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    update_one_async({ collectionname, item, w = 1, j = false }) {
        this.verbose('update_one invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: item,
                w: w,
                j: j
            };
            const reqptr = encodeStruct(req, UpdateOneRequestWrapper);
            const callback = (responsePtr) => {
                this.verbose('update_one_async callback');
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
            this.lib.update_one_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('UpdateOne failed'));
                }
            });
        });
    }
    insert_or_update_one({ collectionname, item, uniqeness = "_id", w = 1, j = false }) {
        this.verbose('insert_or_update_one invoked');
        const req = {
            collectionname: collectionname,
            item: item,
            uniqeness: uniqeness,
            w: w,
            j: j
        };
        const reqptr = encodeStruct(req, InsertOrUpdateOneRequestWrapper);
        this.trace('call insert_or_update_one');
        const response = this.lib.insert_or_update_one(this.client, reqptr);
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
        this.verbose('insert_or_update_one invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                item: item,
                uniqeness: uniqeness,
                w: w,
                j: j
            };
            const reqptr = encodeStruct(req, InsertOrUpdateOneRequestWrapper);
            const callback = (responsePtr) => {
                this.verbose('insert_or_update_one_async callback');
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
            this.lib.insert_or_update_one_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('InsertOrUpdateOne failed'));
                }
            });        
        });
    }
    delete_one({ collectionname, id, recursive }) {
        this.verbose('delete_one invoked');
        const req = {
            collectionname: collectionname,
            id: id,
            recursive: recursive
        };
        const reqptr = encodeStruct(req, DeleteOneRequestWrapper);
        this.trace('call delete_one');
        const response = this.lib.delete_one(this.client, reqptr);
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
    delete_one_async({ collectionname, id, recursive }) {
        this.verbose('delete_one_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                id: id,
                recursive: recursive
            };
            const reqptr = encodeStruct(req, DeleteOneRequestWrapper);
            const callback = (responsePtr) => {
                this.verbose('delete_one_async callback');
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
            this.lib.delete_one_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('DeleteOne failed'));
                }
            });
        });
    }
    delete_many({ collectionname, query = "", ids = [], recursive = false }) {
        this.verbose('delete_many invoked');
        const req = {
            collectionname: collectionname,
            query: query,
            ids: null,
            recursive: recursive
        };
        ids.push(null); // terminate array
        req.ids = ids;
        const reqptr = encodeStruct(req, DeleteManyRequestWrapper);
        this.trace('call delete_many');
        const response = this.lib.delete_many(this.client, reqptr);
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
    delete_many_async({ collectionname, query = "", ids = [], recursive = false }) {
        this.verbose('delete_many_async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                query: query,
                ids: ids,
                recursive: recursive
            };
            const reqptr = encodeStruct(req, DeleteManyRequestWrapper);
            const callback = (responsePtr) => {
                this.verbose('delete_many_async callback');
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
            this.lib.delete_many_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('DeleteMany failed'));
                }
            });
        });
    }
    download({ collectionname, id, folder, filename }) {
        this.verbose('download invoked');
        const req = {
            collectionname: collectionname,
            id: id,
            folder: folder,
            filename: filename
        };
        const reqptr = encodeStruct(req, DownloadRequestWrapper);
        this.trace('call download');
        const response = this.lib.download(this.client, reqptr);
        const result = koffi.decode(response, DownloadResponseWrapper);
        this.trace('free_download_response');
        this.lib.free_download_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.filename;
    }
    download_async({ collectionname, id, folder, filename }) {
        this.verbose('download async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                collectionname: collectionname,
                id: id,
                folder: folder,
                filename: filename
            };
            const reqptr = encodeStruct(req, DownloadRequestWrapper);
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.verbose('download_async callback');
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
            this.lib.download_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Download failed'));
                }
            });
        });
    }
    upload({ filepath, filename, mimetype, metadata, collectionname }) {
        this.verbose('upload invoked');
        const req = {
            filepath: filepath,
            filename: filename,
            mimetype: mimetype,
            metadata: metadata,
            collectionname: collectionname
        };
        const reqptr = encodeStruct(req, UploadRequestWrapper);
        this.trace('call upload');
        const response = this.lib.upload(this.client, reqptr);
        const result = koffi.decode(response, UploadResponseWrapper);
        this.trace('free_upload_response');
        this.lib.free_upload_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.id
    }
    upload_async({ filepath, filename, mimetype, metadata, collectionname }) {
        this.verbose('upload async invoked');
        return new Promise((resolve, reject) => {
            const req = {
                filepath: filepath,
                filename: filename,
                mimetype: mimetype,
                metadata: metadata,
                collectionname: collectionname
            };
            const reqptr = encodeStruct(req, UploadRequestWrapper);
            this.trace('create callback');
            const callback = (responsePtr) => {
                this.verbose('upload_async callback');
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
            this.lib.upload_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('Upload failed'));
                }
            });
        });
    }
    watches = {}
    watch({ collectionname, paths }, callback) {
        this.info('watch invoked');
        const req = new WatchRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            paths: ref.allocCString(paths)
        });
        this.info('call watch');
        const response = this.lib.watch(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_watch_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let watchid = result.watchid;
        this.watches[watchid] = setInterval(() => {
            if (this.connected == false) {
                clearInterval(this.watches[watchid]);
                delete this.watches[watchid];
                return;
            }
            let hadone = false;
            do {
                // this.log('call next');
                const responsePtr = this.lib.next_watch_event(ref.allocCString(watchid));
                const result = responsePtr.deref();
                if (result.id != null && result.id != "") {
                    hadone = true;
                    let event = {
                        id: result.id,
                        operation: result.operation,
                        document: JSON.parse(result.document),
                    }
                    // this.log('call next had result', event);
                    callback(event);
                    // callback(JSON.parse(result));
                } else {
                    hadone = false;
                }
                this.lib.free_watch_event(responsePtr);
            } while (hadone);
        }, 1000);
        return result.watchid;
    }
    watch_async({ collectionname, paths }, callback) {
        this.info('watch invoked');
        return new Promise((resolve, reject) => {
            this.info('create event_callbackPtr');
            const event_callbackPtr = ffi.Callback('void', ['string'], (data) => {
                this.info('watch_async event callback');
                try {
                    const event = JSON.parse(data);
                    event.document = JSON.parse(event.document);
                    callback(event);
                } catch (error) {
                    console.error(`watch callback error: ${error}`);
                }
            });
            const req = new WatchRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                paths: ref.allocCString(paths)
            });

            this.info('create callback');
            const callbackPtr = ffi.Callback('void', [koffi.pointer(WatchResponseWrapper)], (responsePtr) => {
                this.info('watch_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.watchid);
                }
                this.lib.free_watch_response(responsePtr);
            });

            this.info('call watch_async');
            this.lib.watch_async.async(this.client, req.ref(), callbackPtr, event_callbackPtr, (err) => {
                if (err) {
                    reject(new ClientError('watch failed'));
                }
            });
        });
    }
    unwatch(watchid) {
        const response = this.lib.unwatch(this.client, watchid);
        if (ref.isNull(response)) {
            throw new ClientError('UnWatch failed');
        }
        const Obj = JSON.parse(JSON.stringify(response.deref()));
        const result = {
            success: Obj.success,
            error: Obj.error
        };
        this.lib.free_unwatch_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        if (this.watches[watchid] != null) {
            clearInterval(this.watches[watchid]);
            delete this.watches[watchid];
        }
    }

    queues = {}
    register_queue({ queuename }, callback) {
        this.info('register queue invoked');
        const req = new RegisterQueueRequestWrapper({
            queuename: ref.allocCString(queuename)
        });
        this.info('call register_queue');
        const response = this.lib.register_queue(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_register_queue_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        queuename = result.queuename;
        this.queues[queuename] = setInterval(() => {
            if (this.connected == false) {
                clearInterval(this.queues[queuename]);
                delete this.queues[queuename];
                return;
            }
            let hadone = false;
            do {
                // this.log('call next');
                const responsePtr = this.lib.next_queue_event(ref.allocCString(queuename));
                const result = JSON.parse(JSON.stringify(responsePtr.deref()));
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
                    // this.log('call next had result', event);
                    callback(event);
                    // callback(JSON.parse(result));
                } else {
                    hadone = false;
                }
                this.lib.free_queue_event(responsePtr);
            } while (hadone);
        }, 1000);
        return result.queuename;
    }
    register_exchange({ exchangename, algorithm, routingkey, addqueue }, callback) {
        this.info('register exchange invoked');
        if (exchangename == null || exchangename == "") throw new ClientError('exchangename is required');
        if (algorithm == null) algorithm = "";
        if (routingkey == null) routingkey = "";
        if (addqueue == null) addqueue = true;
        const req = new RegisterExchangeRequestWrapper({
            exchangename: ref.allocCString(exchangename),
            algorithm: ref.allocCString(algorithm),
            routingkey: ref.allocCString(routingkey),
            addqueue: addqueue
        });
        this.info('call register_exchange');
        const response = this.lib.register_exchange(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_register_exchange_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        let queuename = result.queuename;
        if (queuename != null && queuename != "") {
            this.queues[queuename] = setInterval(() => {
                if (this.connected == false) {
                    clearInterval(this.queues[queuename]);
                    delete this.queues[queuename];
                    return;
                }
                let hadone = false;
                do {
                    // this.log('call next');
                    const responsePtr = this.lib.next_queue_event(ref.allocCString(queuename));
                    const result = JSON.parse(JSON.stringify(responsePtr.deref()));
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
                        // this.log('call next had result', event);
                        callback(event);
                        // callback(JSON.parse(result));
                    } else {
                        hadone = false;
                    }
                    this.lib.free_queue_event(responsePtr);
                } while (hadone);
            }, 1000);
        }
        return result.queuename;
    }
    unregister_queue(queuename) {
        const response = this.lib.unregister_queue(this.client, queuename);
        if (ref.isNull(response)) {
            throw new ClientError('unregister_queue failed');
        }
        const Obj = JSON.parse(JSON.stringify(response.deref()));
        const result = {
            success: Obj.success,
            error: Obj.error
        };
        this.lib.free_unregister_queue_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        if (this.queues[queuename] != null) {
            clearInterval(this.queues[queuename]);
            delete this.queues[queuename];
        }
    }
    push_workitem({ wiq = "", wiqid = "", name, payload = "{}", nextrun = 0, success_wiqid = "", failed_wiqid = "", success_wiq = "", failed_wiq = "", priority = 2,
        files = []
     }) {
        this.verbose('push_workitem invoked');
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
            payload: payload,
            nextrun: nextrun,
            success_wiqid: success_wiqid,
            failed_wiqid: failed_wiqid,
            success_wiq: success_wiq,
            failed_wiq: failed_wiq,
            priority: priority,
            files: files,
            files_len: files.length
        };
        for(let i = 0; i < files.length; i++) {
            let file = files[i];
            // is file a string ?
            if( typeof file === 'string' ) {
                // then convert it to a file object
                files[i] = {
                    filename: file,
                    id: "",
                    // compressed: false,
                    // file: [null]
                }
            } else {
                // if(file.file != null && file.file.length > 0) {
                //     file.file.push(null); // terminate array
                // }
            }
            const fileptr = encodeStruct(files[i], WorkitemFileWrapper);
            files[i] = fileptr;
        }
        if(files.length == 0 || files[-1] != null) {
            files.push(null); // terminate array
        }
        const reqptr = encodeStruct(req, PushWorkitemRequestWrapper);
        this.verbose('call push_workitem');
        const response = this.lib.push_workitem(this.client, reqptr);
        this.verbose('decode response');
        const result = koffi.decode(response, PushWorkitemResponseWrapper);
        this.verbose('free_push_workitem_response');
        this.lib.free_push_workitem_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.id;
    }
    push_workitem_async({ wiq = "", wiqid = "", name, payload = "{}", nextrun = 0, success_wiqid = "", failed_wiqid = "", success_wiq = "", failed_wiq = "", priority = 2,
        files = []
        }) {
        this.verbose('push_workitem invoked');
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
                payload: payload,
                nextrun: nextrun,
                success_wiqid: success_wiqid,
                failed_wiqid: failed_wiqid,
                success_wiq: success_wiq,
                failed_wiq: failed_wiq,
                priority: priority,
                files: files,
                files_len: files.length
            };
            for(let i = 0; i < files.length; i++) {
                let file = files[i];
                // is file a string ?
                if( typeof file === 'string' ) {
                    // then convert it to a file object
                    files[i] = {
                        filename: file,
                        id: "",
                        // compressed: false,
                        // file: [null]
                    }
                } else {
                    // if(file.file != null && file.file.length > 0) {
                    //     file.file.push(null); // terminate array
                    // }
                }
                const fileptr = encodeStruct(files[i], WorkitemFileWrapper);
                files[i] = fileptr;
            }
            if(files.length == 0 || files[-1] != null) {
                files.push(null); // terminate array
            }
            const reqptr = encodeStruct(req, PushWorkitemRequestWrapper);
            this.verbose('create callback');
            const callback = (responsePtr) => {
                this.verbose('push_workitem_async callback');
                this.trace('decode response');
                const response = koffi.decode(responsePtr, PushWorkitemResponseWrapper);
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.id);
                }
                this.trace('free_push_workitem_response');
                this.lib.free_push_workitem_response(responsePtr);
            };

            const cb = koffi.register(callback, koffi.pointer(this.lib.push_workitemCallback));
            this.verbose('call push_workitem_async');
            if(files.length > 0) {
                let f = files[0];
            }
            this.lib.push_workitem_async(this.client, reqptr, cb, (err) => {
                if (err) {
                    reject(new ClientError('PushWorkitem async failed'));
                }
            }
            );
        });
    }    
    pop_workitem({ wiq = "", wiqid = "", downloadfolder = "." }) {
        this.verbose('pop_workitem invoked');
        if(downloadfolder == null || downloadfolder == "") downloadfolder = ".";
        const req = {
            wiq: wiq,
            wiqid: wiqid
        };
        const reqptr = encodeStruct(req, PopWorkitemRequestWrapper);
        this.trace('call pop_workitem');
        const response = this.lib.pop_workitem(this.client, reqptr, downloadfolder);
        this.trace('decode response');
        const result = koffi.decode(response, PopWorkitemResponseWrapper);
        this.trace('free_pop_workitem_response');
        this.lib.free_pop_workitem_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        if(result.workitem != null) {
            var workitem = koffi.decode(result.workitem, WorkitemWrapper);
            let _files = [];
            if(workitem.files_len > 0) {
                var files = koffi.decode(workitem.files, 'WorkitemFileWrapper ***', workitem.files_len);
                for(let i = 0; i < workitem.files_len; i++) {
                    let file = files[i];
                    if(file != null) {
                        var _file = koffi.decode(file, WorkitemFileWrapper);
                        delete _file.compressed;
                        delete _file.file;
                        _files.push(_file);
                    }
                }
            }
            workitem.files = _files;
            if(workitem.nextrun > 0) {
                workitem.nextrun = new Date(workitem.nextrun * 1000);
            } else {
                delete workitem.nextrun;
            }
            if(workitem.lastrun > 0) {
                workitem.lastrun = new Date(workitem.lastrun * 1000);
            } else {
                delete workitem.lastrun;
            }
            try {
                if(workitem.payload != null && workitem.payload != "") {
                    workitem.payload = JSON.parse(workitem.payload);
                }
            } catch (error) {
            }
            return workitem;
        }
        return null;
    }

}

module.exports = {
    Client,
    ClientError,
    LibraryLoadError,
    ClientCreationError
};
