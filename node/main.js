const ffi = require('ffi-napi');
const ref = require('ref-napi');
const StructType = require('ref-struct-di')(ref);
const ArrayType = require('ref-array-di')(ref);
const path = require('path');
const fs = require('fs');

const CString = ref.types.CString;
const CStringArray = ArrayType(CString);

const voidPtr = ref.refType(ref.types.void);
const bool = ref.types.bool;
const int = ref.types.int;
const size_t = ref.types.size_t;


// Define the ClientWrapper struct
const ClientWrapper = StructType({
    success: bool,
    error: CString,
    client: voidPtr,
    runtime: voidPtr
});
const ClientWrapperPtr = ref.refType(ClientWrapper);

// Define the SigninRequestWrapper struct
const SigninRequestWrapper = StructType({
    username: CString,
    password: CString,
    jwt: CString,
    agent: CString,
    version: CString,
    longtoken: bool,
    validateonly: bool,
    ping: bool,
});
const SigninRequestWrapperPtr = ref.refType(SigninRequestWrapper);

// Define the SigninResponseWrapper struct
const SigninResponseWrapper = StructType({
    success: bool,
    jwt: CString,
    error: CString
});
const SigninResponseWrapperPtr = ref.refType(SigninResponseWrapper);

// Define the SigninRequestWrapper struct
const QueryRequestWrapper = StructType({
    collectionname: CString,
    query: CString,
    projection: CString,
    orderby: CString,
    queryas: CString,
    explain: bool,
    skip: int,
    top: int,
});
const QueryRequestWrapperPtr = ref.refType(QueryRequestWrapper);

// Define the SigninResponseWrapper struct
const QueryResponseWrapper = StructType({
    success: bool,
    results: CString,
    error: CString
});
const QueryResponseWrapperPtr = ref.refType(QueryResponseWrapper);

// Define the SigninRequestWrapper struct
const AggregateRequestWrapper = StructType({
    collectionname: CString,
    aggregates: CString,
    queryas: CString,
    hint: CString,
    explain: bool,
});
const AggregateRequestWrapperPtr = ref.refType(AggregateRequestWrapper);

// Define the SigninResponseWrapper struct
const AggregateResponseWrapper = StructType({
    success: bool,
    results: CString,
    error: CString
});
const AggregateResponseWrapperPtr = ref.refType(AggregateResponseWrapper);

const CountRequestWrapper = StructType({
    collectionname: CString,
    query: CString,
    queryas: CString,
    explain: bool,
});
const CountRequestWrapperPtr = ref.refType(CountRequestWrapper);
const CountResponseWrapper = StructType({
    success: bool,
    result: int,
    error: CString
});
const CountResponseWrapperPtr = ref.refType(CountResponseWrapper);

const DistinctRequestWrapper = StructType({
    collectionname: CString,
    field: CString,
    query: CString,
    queryas: CString,
    explain: bool,
});
const DistinctRequestWrapperPtr = ref.refType(DistinctRequestWrapper);
const DistinctResponseWrapper = StructType({
    success: bool,
    results: ref.refType(CStringArray),
    results_count: size_t,
    error: CString
});
const DistinctResponseWrapperPtr = ref.refType(DistinctResponseWrapper);

// Define the SigninRequestWrapper struct
const InsertOneRequestWrapper = StructType({
    collectionname: CString,
    item: CString,
    w: int,
    j: bool,
});
const InsertOneRequestWrapperPtr = ref.refType(InsertOneRequestWrapper);

// Define the SigninResponseWrapper struct
const InsertOneResponseWrapper = StructType({
    success: bool,
    rresult: CString,
    error: CString
});
const InsertOneResponseWrapperPtr = ref.refType(InsertOneResponseWrapper);

const DownloadRequestWrapper = StructType({
    collectionname: CString,
    id: CString,
    folder: CString,
    filename: CString
});
const DownloadRequestWrapperPtr = ref.refType(DownloadRequestWrapper);
const DownloadResponseWrapper = StructType({
    success: bool,
    filename: CString,
    error: CString
});
const DownloadResponseWrapperPtr = ref.refType(DownloadResponseWrapper);

const UploadRequestWrapper = StructType({
    filepath: CString,
    filename: CString,
    mimetype: CString,
    metadata: CString,
    collectionname: CString
});
const UploadRequestWrapperPtr = ref.refType(UploadRequestWrapper);
const UploadResponseWrapper = StructType({
    success: bool,
    id: CString,
    error: CString
});
const UploadResponseWrapperPtr = ref.refType(UploadResponseWrapper);

const WatchRequestWrapper = StructType({
    collectionname: CString,
    paths: CString,
});
const WatchRequestWrapperPtr = ref.refType(WatchRequestWrapper);
const WatchResponseWrapper = StructType({
    success: bool,
    watchid: CString,
    error: CString
});
const WatchResponseWrapperPtr = ref.refType(WatchResponseWrapper);

const UnWatchResponseWrapper = StructType({
    success: bool,
    error: CString
});
const UnWatchResponseWrapperPtr = ref.refType(UnWatchResponseWrapper);

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
    if(!fs.existsSync(libPath)) {
        libDir = path.join(__dirname, '../target/debug/');
        switch (process.platform) {
            case 'win32':
                libPath = path.join(libDir, 'openiap.dll');
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
        return ffi.Library(libPath, {
            'enable_tracing': ['void', [CString, CString]],
            'disable_tracing': ['void', []],
            'client_connect': ['void', [CString, 'pointer']],
            'free_client': ['void', [ClientWrapperPtr]],
            'client_signin': ['void', [voidPtr, SigninRequestWrapperPtr, 'pointer']],
            'free_signin_response': ['void', [SigninResponseWrapperPtr]],
            'client_query': ['void', [voidPtr, QueryRequestWrapperPtr, 'pointer']],
            'free_query_response': ['void', [QueryResponseWrapperPtr]],
            'client_aggregate': ['void', [voidPtr, AggregateRequestWrapperPtr, 'pointer']],
            'free_aggregate_response': ['void', [AggregateResponseWrapperPtr]],
            'client_count': ['void', [voidPtr, CountRequestWrapperPtr, 'pointer']],
            'free_count_response': ['void', [CountResponseWrapperPtr]],
            'client_distinct': ['void', [voidPtr, DistinctRequestWrapperPtr, 'pointer']],
            'free_distinct_response': ['void', [DistinctResponseWrapperPtr]],
            'client_insert_one': ['void', [voidPtr, InsertOneRequestWrapperPtr, 'pointer']],
            'free_insert_one_response': ['void', [InsertOneResponseWrapperPtr]],
            'client_download': ['void', [voidPtr, DownloadRequestWrapperPtr, 'pointer']],
            'free_download_response': ['void', [DownloadResponseWrapperPtr]],
            'client_upload': ['void', [voidPtr, UploadRequestWrapperPtr, 'pointer']],
            'free_upload_response': ['void', [UploadResponseWrapperPtr]],
            'client_watch': ['void', [voidPtr, WatchRequestWrapperPtr, 'pointer', 'pointer']],
            'free_watch_response': ['void', [WatchResponseWrapperPtr]],
            'client_unwatch': [UnWatchResponseWrapperPtr, [voidPtr, CString]],
            'free_unwatch_response': ['void', [UnWatchResponseWrapperPtr]],

            // 'run_async_in_node': ['void', ['pointer']]
        });
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
    }

    enable_tracing(rust_log = '', tracing = '') {
        if(rust_log == null || rust_log == '') { rust_log = ''; }
        if(tracing == null || tracing == '') { tracing = ''; }
        rust_log = ref.allocCString(rust_log);
        tracing = ref.allocCString(tracing);
        this.lib.enable_tracing(rust_log, tracing);
    }
    disable_tracing() {
        this.lib.disable_tracing();
    }
    log(message) {
        console.log(message);
    }

    // refs = [];
    // run_async_in_node(callback) {
    //     console.log("NodeJS: run_async_in_node");
    //     let _callback = ffi.Callback('void', [], () => {
    //         console.log("NodeJS: Callback called from Rust on main thread!");
    //         callback()
    //     });
    //     this.lib.run_async_in_node(_callback);
    //     this.refs.push(_callback);
    //     console.log("NodeJS: run_async_in_node done");
    // }

    connect(url) {
        return new Promise((resolve, reject) => {
            try {
                const callback = ffi.Callback('void', [ClientWrapperPtr], (clientPtr) => {
                    this.log('Node.js: Callback invoked');
                    try {
                        this.client = clientPtr;
                        const clientres = clientPtr.deref();
                        this.log('Node.js: Client result');
                        if (!clientres.success) {
                            reject(new ClientCreationError(clientres.error));
                        } else {
                            resolve(clientPtr);
                        }
                    } catch (error) {
                        reject(new ClientCreationError(error.message));                        
                    }
                });
                this.log('Node.js: Calling client_connect');
                this.lib.client_connect(url, callback);
                this.log('Node.js: client_connect called');
            } catch (error) {
                reject(new ClientCreationError(error.message));
            }
        });
    }

    signin(username, password) {
        this.log('Node.js: signin invoked');
        return new Promise((resolve, reject) => {
            let jwt = "";
            if (username == null) username = '';
            if (password == null) password = '';
            if (username != "" && password == "") {
                jwt = username;
                username = "";
            }
            const req = new SigninRequestWrapper({
                username: ref.allocCString(username),
                password: ref.allocCString(password),
                jwt: ref.allocCString(jwt),
                agent: ref.allocCString('node'),
                version: ref.allocCString(''),
                longtoken: false,
                validateonly: false,
                ping: false
            });
    
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(SigninResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_signin callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        jwt: response.jwt,
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_signin_response(responsePtr);
            });
    
            this.log('Node.js: call client_signin');
            this.lib.client_signin.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Signin failed or user is null'));
                }
            });
        });
    }

    query({collectionname, query, projection, orderby, queryas, explain, skip, top}) {
        this.log('Node.js: query invoked');
        return new Promise((resolve, reject) => {
            const req = new QueryRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                query: ref.allocCString(query),
                projection: ref.allocCString(projection),
                orderby: ref.allocCString(orderby),
                queryas: ref.allocCString(queryas),
                explain: explain,
                skip: skip,
                top: top
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(QueryResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_query callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        results: JSON.parse(response.results),
                        error: null
                    };
                    resolve(result);
                }
                // this.lib.free_query_response(responsePtr);
            });

            this.log('Node.js: call client_query');
            this.lib.client_query.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Query failed'));
                }
            });
        });
    }
    aggregate({collectionname, aggregates, queryas, hint, explain}) {
        this.log('Node.js: aggregate invoked');
        if(aggregates == null) aggregates = '[]';
        if(queryas == null) queryas = '';
        if(hint == null) hint = '';
        return new Promise((resolve, reject) => {
            const req = new AggregateRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                aggregates: ref.allocCString(aggregates),
                queryas: ref.allocCString(queryas),
                hint: ref.allocCString(hint),
                explain: explain
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(AggregateResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_aggregate callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        results: JSON.parse(response.results),
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_aggregate_response(responsePtr);
            });

            this.log('Node.js: call client_aggregate');
            this.lib.client_aggregate.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Aggregate failed'));
                }
            });
        });
    }
    count({collectionname, query, queryas, explain}) {
        this.log('Node.js: count invoked');
        return new Promise((resolve, reject) => {
            const req = new CountRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                query: ref.allocCString(query),
                queryas: ref.allocCString(queryas),
                explain: explain
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(CountResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_count callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        result: response.result,
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_count_response(responsePtr);
            });

            this.log('Node.js: call client_count');
            this.lib.client_count.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Count failed'));
                }
            });
        });
    }
    distinct({collectionname, field, query = "", queryas = "", explain = false}) {
        this.log('Node.js: distinct invoked');
        return new Promise((resolve, reject) => {
            const req = new DistinctRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                field: ref.allocCString(field),
                query: ref.allocCString(query),
                queryas: ref.allocCString(queryas),
                explain: explain
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [DistinctResponseWrapperPtr], (responsePtr) => {
                this.log('Node.js: client_distinct callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {






                    const resultsArrayPtr = response.results;
                    const resultsCount = response.results_count;
                    const results = [];

                    for (let i = 0; i < resultsCount; i++) {
                        const cstrPtr = resultsArrayPtr.readPointer(i * ref.sizeof.pointer);
                        const jsString = ref.readCString(cstrPtr);
                        results.push(jsString);
                    }

                    const result = {
                        success: response.success,
                        results: results,
                        error: null
                    };
                    resolve(result);






                    // const result = {
                    //     success: response.success,
                    //     results: response.results,
                    //     error: null
                    // };
                    // resolve(result);
                }
                this.lib.free_distinct_response(responsePtr);
            });

            this.log('Node.js: call client_distinct');
            this.lib.client_distinct.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Distinct failed'));
                }
            });
        });
    }
    insert_one({collectionname, document, w, j}) {
        this.log('Node.js: insert_one invoked');
        return new Promise((resolve, reject) => {
            const req = new InsertOneRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                item: ref.allocCString(document),
                w: w,
                j: j
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(InsertOneResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_insert_one callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        rresult: JSON.parse(response.rresult),
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_insert_one_response(responsePtr);
            });

            this.log('Node.js: call client_insert_one');
            this.lib.client_insert_one.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('InsertOne failed'));
                }
            });
        });
    };
    download({collectionname, id, folder, filename}) {
        this.log('Node.js: download invoked');
        return new Promise((resolve, reject) => {
            const req = new DownloadRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                id: ref.allocCString(id),
                folder: ref.allocCString(folder),
                filename: ref.allocCString(filename)
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(DownloadResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_download callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        filename: response.filename,
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_download_response(responsePtr);
            });

            this.log('Node.js: call client_download');
            this.lib.client_download.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Download failed'));
                }
            });
        });
    }
    upload({filepath, filename, mimetype, metadata, collectionname}) {
        this.log('Node.js: upload invoked');
        return new Promise((resolve, reject) => {
            const req = new UploadRequestWrapper({
                filepath: ref.allocCString(filepath),
                filename: ref.allocCString(filename),
                mimetype: ref.allocCString(mimetype),
                metadata: ref.allocCString(metadata),
                collectionname: ref.allocCString(collectionname)
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(UploadResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_upload callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        id: response.id,
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_upload_response(responsePtr);
            });

            this.log('Node.js: call client_upload');
            this.lib.client_upload.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Upload failed'));
                }
            });
        });
    }
    watch({collectionname, paths}, callback) {
        this.log('Node.js: watch invoked');
        return new Promise((resolve, reject) => {
            this.log('Node.js: create event_callbackPtr');
            const event_callbackPtr = ffi.Callback('void', ['string'], (data) => {
                this.log('Node.js: client_watch event callback');
                try {
                    const event = JSON.parse(data);
                    // event.document = JSON.parse(event.document);
                    callback(event);
                } catch (error) {
                    console.error(`watch callback error: ${error}`);                
                }            
            });
            const req = new WatchRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                paths: ref.allocCString(paths)
            });

            this.log('Node.js: create callback');
            const callbackPtr = ffi.Callback('void', [ref.refType(WatchResponseWrapper)], (responsePtr) => {
                this.log('Node.js: client_watch callback');
                const response = responsePtr.deref();
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    const result = {
                        success: response.success,
                        watchid: response.watchid,
                        error: null
                    };
                    resolve(result);
                }
                this.lib.free_watch_response(responsePtr);
            });

            this.log('Node.js: call client_watch');
            this.lib.client_watch.async(this.client, req.ref(), callbackPtr, event_callbackPtr, (err) => {
                if (err) {
                    reject(new ClientError('watch failed'));
                }
            });
        });
    }
    unwatch(watchid) {
        const response = this.lib.client_unwatch(this.client, watchid);
        if (ref.isNull(response)) {
            throw new ClientError('UnWatch failed');
        }
        const Obj = response.deref();
        const result = {
            success: Obj.success,
            error: Obj.error
        };
        this.lib.free_unwatch_response(response);
        return result;
    }

    free() {
        if (this.client) {
            this.lib.free_client(this.client);
        }
    }
}

module.exports = {
    Client,
    ClientError,
    LibraryLoadError,
    ClientCreationError
};
