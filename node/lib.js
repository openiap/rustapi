const koffi = require('koffi');
const path = require('path');
const fs = require('fs');

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

// Define the SigninRequestWrapper struct
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

// Define the SigninResponseWrapper struct
const SigninResponseWrapper = koffi.struct('SigninResponseWrapper', {
    success: bool,
    jwt: CString,
    error: CString
});
const SigninResponseWrapperPtr = koffi.pointer(SigninResponseWrapper);

// Define the SigninRequestWrapper struct
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

// Define the SigninResponseWrapper struct
const QueryResponseWrapper = koffi.struct('QueryResponseWrapper', {
    success: bool,
    results: CString,
    error: CString
});
const QueryResponseWrapperPtr = koffi.pointer(QueryResponseWrapper);

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
        return koffi.load(libPath)
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
    connected = false;
    free() {
        if (this.client) {
            // this.lib.free_client(this.client);
            this.lib.func('void free_client(ClientWrapper*)')(this.client);
        }
        this.connected = false;
    }

    enable_tracing(rust_log = '', tracing = '') {
        // if (rust_log == null || rust_log == '') { rust_log = ''; }
        // if (tracing == null || tracing == '') { tracing = ''; }
        // rust_log = ref.allocCString(rust_log);
        // tracing = ref.allocCString(tracing);
        this.log('Node.js: enable_tracing invoked', rust_log, tracing);
        this.lib.func('void enable_tracing(const char* rust_log, const char* tracing)')(rust_log, tracing);
        this.log('Node.js: enable_tracing called');
    }
    disable_tracing() {
        this.lib.func('void disable_tracing()')();
    }
    log(...args) {
        console.log(...args);
    }

    async connect(url) {
        this.connected = false;

        // const connect = this.lib.func('ClientWrapper *connect(const char *server_address)');
        // const connect = this.lib.func('connect', 'ClientWrapper', ['str']);

        const connect = this.lib.func('connect2', ClientWrapperPtr, ['str']);
        const _clientWrapperPtr = connect(url);
        if (_clientWrapperPtr === 0) {
            throw new Error('Received a null pointer from Rust function');
        }
        const clientWrapper = koffi.decode(_clientWrapperPtr,ClientWrapper);

        this.connected = true;
        this.client = _clientWrapperPtr;
        return clientWrapper;
    }
    // createPromise() {
    //     let resolvePromise, pending = true;
    //     const promise = new Promise(async (resolve) => {
    //         console.log("createPromise::promise");
    //         resolvePromise = resolve;
    //         while(pending) {
    //             await new Promise((resolve, reject) => { setTimeout(resolve, 200); });
    //         }
    //         console.log("createPromise::exit, no longer pending");
    //     });
    //     return { promise, resolve: (value)=> {
    //         pending = false;
    //         resolvePromise(value)
    //     }
    //      };
    // }
    // async test_add() {
    //     let { promise, resolve } = this.createPromise();
    //     console.log("test_add::begin");
    //     const TransferCallback = koffi.proto('TransferCallback', 'void', ['int']);
    //     // Sync
    //     const add = this.lib.func('add', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
    //     // Async
    //     const addAsync = this.lib.func('add_async', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
    //     function callback(value) {
    //         console.log("callback with value", value);
    //         if(value==4) {
    //             resolve();
    //         }
    //     }
    //     const cb = koffi.register(callback, koffi.pointer(TransferCallback));
    //     console.log("call::add");
    //     add(1, 2, cb)
    //     console.log("call::addAsync");
    //     addAsync(2, 2, cb)
    //     console.log("test_add::await promise");
    //     await promise;
    //     console.log("test_add::done");
    // }

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
    // test_add() {
    //     return new Promise(async (resolve, reject) => {
    //         console.log("test_add::begin");
    //         const TransferCallback = koffi.proto('TransferCallback', 'void', ['int']);
    //         // Sync
    //         const add = this.lib.func('add', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
    //         // Async
    //         const addAsync = this.lib.func('add_async', 'void', ['int', 'int', koffi.pointer(TransferCallback)]);
    //         function callback(value) {
    //             console.log("callback with value", value);
    //             if(value==4) {
    //                 resolve();
    //             }
    //         }
    //         const cb = koffi.register(callback, koffi.pointer(TransferCallback));
    //         console.log("call::add");
    //         add(1, 2, cb)
    //         console.log("call::addAsync");
    //         addAsync(2, 2, cb)
    //         console.log("test_add::sleep");
    //         // await new Promise((resolve, reject) => { setTimeout(resolve, 2000); }); // wait longer than the async callback ( so more than 1 second )
    //         console.log("test_add::done");
    //     });
    // }

    connect_async(url) {
        this.connected = false;
        return new Promise((resolve, reject) => {
            try {

                // // const connect_cb = koffi.proto('connect_cb', 'void', [ClientWrapperPtr]);
                // const connect_cb = koffi.proto('void connectcb(ClientWrapper*)');
                // const connect_async = this.lib.func('connect_async', 'void', ['str', koffi.pointer(connect_cb)]);

                // const callback = (_ClientWrapperPtr) => {
                //     this.log('Node.js: Callback invoked');
                //     try {
                //         if (_clientWrapperPtr === 0) {
                //             throw new Error('Received a null pointer from Rust function');
                //         }
                //         const clientWrapper = koffi.decode(_clientWrapperPtr,ClientWrapper);
                //         this.client = _clientWrapperPtr;
                //         if (!clientWrapper.success) {
                //             reject(new ClientCreationError(clientWrapper.error));
                //         } else {
                //             this.connected = true;
                //             resolve(clientWrapper);
                //         }
                //     } catch (error) {
                //         reject(new ClientCreationError(error.message));
                //     }
                // };

                // let cb = koffi.register(callback, koffi.pointer(connect_cb));

                // connect_async(url, cb);
                
                // // this.lib.func('void connect_async(const char* url, void (*callback)(ClientWrapper*))')(url, callback);
                // this.log('Node.js: connect_async called');

                // const ConnectCallback = koffi.proto('void ConnectCallback(ClientWrapper*)');
                // // const connect_async = this.lib.func('connect_async', 'void', ['str', koffi.pointer(ConnectCallback)]);
                // const connect_async = this.lib.func('void connect_async(const char* url, ConnectCallback* callback)');

                // const cb = koffi.register((wrapper) => {
                //     this.log('Node.js: Callback invoked');
                //     try {
                //         if (wrapper === 0) {
                //             throw new Error('Received a null pointer from Rust function');
                //         }
                //         const clientWrapper = koffi.decode(wrapper, ClientWrapper);
                //         this.client = wrapper;
                //         if (!clientWrapper.success) {
                //             reject(new ClientCreationError(clientWrapper.error));
                //         } else {
                //             this.connected = true;
                //             resolve(clientWrapper);
                //         }
                //     } catch (error) {
                //         reject(new ClientCreationError(error.message));
                //     } 
                // }, koffi.pointer(ConnectCallback));
                // connect_async(url, cb);



                setTimeout(() => {
                    resolve({ success: true, error: null });
                }, 10000);

                
            } catch (error) {
                reject(new ClientCreationError(error.message));
            }
        });
    }

    signin(username = '', password = '') {
        this.log('Node.js: signin invoked');
        let jwt = "";
        if (username == null) username = '';
        if (password == null) password = '';
        if (username != "" && password == "") {
            jwt = username;
            username = "";
        }
        // const req = new SigninRequestWrapper({
        const req = {
            username: username,
            password: password,
            jwt: jwt,
            agent: 'node',
            version: '',
            longtoken: false,
            validateonly: false,
            ping: false
        };
        // const reqptr = koffi.encode(req, SigninRequestWrapper);
        const reqptr = encodeStruct(req, SigninRequestWrapper);

        this.log('Node.js: call signin');
        // const response = this.lib.func('SigninResponseWrapper* signin(void* client, SigninRequestWrapper* req)')(this.client, reqptr);
        const response = this.lib.func('signin', koffi.pointer(SigninResponseWrapper), [ClientWrapperPtr, SigninRequestWrapperPtr])(this.client, reqptr);
        const result = koffi.decode(response,SigninResponseWrapper);

        // const result = JSON.parse(JSON.stringify(response.deref()));
        // this.lib.free_signin_response(response);
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
    signin_async(username, password) {
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
            const callback = koffi.proto('void(SigninResponseWrapper*)', (responsePtr) => {
                this.log('Node.js: signin_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response);
                }
                this.lib.free_signin_response(responsePtr);
            });

            this.log('Node.js: call signin_async');
            this.lib.func('void signin_async(void* client, SigninRequestWrapper* req, void (*callback)(SigninResponseWrapper*))')(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError(err));
                }
            });
        });
    }

    query({ collectionname, query, projection = "", orderby = "", skip = 0, top = 100, queryas = "", explain = false,  }) {
        this.log('Node.js: query invoked');
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
        const response = this.lib.query(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_query_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.results);
    }

    refs = [];
    query_async({ collectionname, query, projection = "", orderby = "", skip = 0, top = 100, queryas = "", explain = false,  }) {
        this.log('Node.js: query invoked');
        return new Promise((resolve, reject) => {
            const req = new QueryRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                query: ref.allocCString(query),
                projection: ref.allocCString(projection),
                orderby: ref.allocCString(orderby),
                queryas: ref.allocCString(queryas),
                explain: ref.alloc(bool, explain),
                skip: ref.alloc(int, skip),
                top: ref.alloc(int, top)
            });
            this.log('Node.js: create callback');
            this.refs.push(req);
            const callback = ffi.Callback('void', [koffi.pointer(QueryResponseWrapper)], (responsePtr) => {
                this.log('Node.js: query_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.lib.free_query_response(responsePtr);
            });


            this.log('Node.js: call query_async');
            this.lib.query_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Query failed'));
                }
            });
        });
    }
    aggregate({ collectionname, aggregates, queryas = "", hint = "", explain = false }) {
        this.log('Node.js: aggregate invoked');
        if (aggregates == null) aggregates = '[]';
        if (queryas == null) queryas = '';
        if (hint == null) hint = '';
        const req = new AggregateRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            aggregates: ref.allocCString(aggregates),
            queryas: ref.allocCString(queryas),
            hint: ref.allocCString(hint),
            explain: explain
        });
        this.log('Node.js: create callback');
        const response = this.lib.aggregate(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_aggregate_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.results);
    }
    aggregate_async({ collectionname, aggregates, queryas = "", hint = "", explain = false }) {
        this.log('Node.js: aggregate invoked');
        if (aggregates == null) aggregates = '[]';
        if (queryas == null) queryas = '';
        if (hint == null) hint = '';
        return new Promise((resolve, reject) => {
            const req = new AggregateRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                aggregates: ref.allocCString(aggregates),
                queryas: ref.allocCString(queryas),
                hint: ref.allocCString(hint),
                explain: explain
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(AggregateResponseWrapper)], (responsePtr) => {
                this.log('Node.js: aggregate_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.results));
                }
                this.lib.free_aggregate_response(responsePtr);
            });

            this.log('Node.js: call aggregate_async');
            this.lib.aggregate_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Aggregate failed'));
                }
            });
        });
    }
    count({ collectionname, query = "", queryas = "", explain = false}) {
        this.log('Node.js: count invoked');
        const req = new CountRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            query: ref.allocCString(query),
            queryas: ref.allocCString(queryas),
            explain: explain
        });
        this.log('Node.js: call count_async');
        const response = this.lib.count(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_count_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.result;
    }
    count_async({ collectionname, query = "", queryas = "", explain = false}) {
        this.log('Node.js: count invoked');
        return new Promise((resolve, reject) => {
            const req = new CountRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                query: ref.allocCString(query),
                queryas: ref.allocCString(queryas),
                explain: explain
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(CountResponseWrapper)], (responsePtr) => {
                this.log('Node.js: count_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.result);
                }
                this.lib.free_count_response(responsePtr);
            });

            this.log('Node.js: call count_async');
            this.lib.count_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Count failed'));
                }
            });
        });
    }
    distinct({ collectionname, field, query = "", queryas = "", explain = false }) {
        this.log('Node.js: distinct invoked');
        const req = new DistinctRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            field: ref.allocCString(field),
            query: ref.allocCString(query),
            queryas: ref.allocCString(queryas),
            explain: explain
        });
        this.log('Node.js: call distinct_async');
        const response = this.lib.distinct(this.client, req.ref());
        const result = response.deref();
        if (!result.success) {
            const errorMsg = JSON.parse(JSON.stringify(result.error));
            this.lib.free_distinct_response(response);
            throw new ClientError(errorMsg);
        }


        const resultsArrayPtr = result.results;
        const resultsCount = result.results_count;
        const _results = [];

        for (let i = 0; i < resultsCount; i++) {
            const cstrPtr = resultsArrayPtr.readPointer(i * ref.sizeof.pointer);
            const jsString = ref.readCString(cstrPtr);
            _results.push(jsString);
        }
        const results = JSON.parse(JSON.stringify(_results));
        this.lib.free_distinct_response(response);
        return results;
    }
    distinct_async({ collectionname, field, query = "", queryas = "", explain = false }) {
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
                this.log('Node.js: distinct_async callback');
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

            this.log('Node.js: call distinct_async');
            this.lib.distinct_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Distinct failed'));
                }
            });
        });
    }
    insert_one({ collectionname, document, w = 1, j = false }) {
        this.log('Node.js: insert_one invoked');
        const req = new InsertOneRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            item: ref.allocCString(document),
            w: w,
            j: j
        });
        this.log('Node.js: call insert_one_async');
        const response = this.lib.insert_one(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_insert_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_one_async({ collectionname, document, w = 1, j = false }) {
        this.log('Node.js: insert_one invoked');
        return new Promise((resolve, reject) => {
            const req = new InsertOneRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                item: ref.allocCString(document),
                w: w,
                j: j
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(InsertOneResponseWrapper)], (responsePtr) => {
                this.log('Node.js: insert_one_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.lib.free_insert_one_response(responsePtr);
            });

            this.log('Node.js: call insert_one_async');
            this.lib.insert_one_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('InsertOne failed'));
                }
            });
        });
    };
    insert_many({ collectionname, documents, w = 1, j = false, skipresults = false }) {
        this.log('Node.js: insert_many invoked');
        const req = new InsertManyRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            items: ref.allocCString(documents),
            w: w,
            j: j,
            skipresults: skipresults
        });
        this.log('Node.js: call insert_many_async');
        const response = this.lib.insert_many(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_insert_many_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_many_async({ collectionname, documents, w = 1, j = false, skipresults = false }) {
        this.log('Node.js: insert_many invoked');
        return new Promise((resolve, reject) => {
            const req = new InsertManyRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                items: ref.allocCString(documents),
                w: w,
                j: j,
                skipresults: skipresults
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(InsertManyResponseWrapper)], (responsePtr) => {
                this.log('Node.js: insert_many_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.lib.free_insert_many_response(responsePtr);
            });

            this.log('Node.js: call insert_many_async');
            this.lib.insert_many_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('InsertMany failed'));
                }
            });
        });
    }
    update_one({ collectionname, item, w = 1, j = false }) {
        this.log('Node.js: update_one invoked');
        const req = new UpdateOneRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            item: ref.allocCString(item),
            w: w,
            j: j
        });
        this.log('Node.js: call update_one_async');
        const response = this.lib.update_one(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_update_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    update_one_async({ collectionname, item, w = 1, j = false }) {
        this.log('Node.js: update_one invoked');
        return new Promise((resolve, reject) => {
            const req = new UpdateOneRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                item: ref.allocCString(item),
                w: w,
                j: j
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(UpdateOneResponseWrapper)], (responsePtr) => {
                this.log('Node.js: update_one_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.lib.free_update_one_response(responsePtr);
            });

            this.log('Node.js: call update_one_async');
            this.lib.update_one_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('UpdateOne failed'));
                }
            });
        });
    }
    insert_or_update_one({ collectionname, item, uniqeness = "_id", w = 1, j = false }) {
        this.log('Node.js: insert_or_update_one invoked');
        const req = new InsertOrUpdateOneRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            uniqeness: ref.allocCString(uniqeness),
            item: ref.allocCString(item),
            w: w,
            j: j
        });
        this.log('Node.js: call insert_or_update_one');
        const response = this.lib.insert_or_update_one(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_insert_or_update_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return JSON.parse(result.result);
    }
    insert_or_update_one_async({ collectionname, item, uniqeness = "_id", w = 1, j = false }) {
        this.log('Node.js: insert_or_update_one invoked');
        return new Promise((resolve, reject) => {
            const req = new InsertOrUpdateOneRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                uniqeness: ref.allocCString(uniqeness),
                item: ref.allocCString(item),
                w: w,
                j: j
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(InsertOrUpdateOneResponseWrapper)], (responsePtr) => {
                this.log('Node.js: insert_or_update_one_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(JSON.parse(response.result));
                }
                this.lib.free_insert_or_update_one_response(responsePtr);
            });

            this.log('Node.js: call insert_or_update_one_async');
            this.lib.insert_or_update_one_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('InsertOrUpdateOne failed'));
                }
            });
        });
    }
    delete_one({ collectionname, id, recursive }) {
        this.log('Node.js: delete_one invoked');
        const req = new DeleteOneRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            id: ref.allocCString(id),
            recursive: recursive
        });
        this.log('Node.js: call delete_one_async');
        const response = this.lib.delete_one(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_delete_one_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.affectedrows;
    }
    delete_one_async({ collectionname, id, recursive }) {
        this.log('Node.js: delete_one invoked');
        return new Promise((resolve, reject) => {
            const req = new DeleteOneRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                id: ref.allocCString(id),
                recursive: recursive
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(DeleteOneResponseWrapper)], (responsePtr) => {
                this.log('Node.js: delete_one_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.affectedrows);
                }
                this.lib.free_delete_one_response(responsePtr);
            });

            this.log('Node.js: call delete_one_async');
            this.lib.delete_one_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('DeleteOne failed'));
                }
            });
        });
    }
    delete_many({ collectionname, query = "", ids = [], recursive = false }) {
        this.log('Node.js: delete_many invoked');
        const idsCStringArray = ids.map(id => ref.allocCString(id));
        const req = new DeleteManyRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            query: ref.allocCString(query),
            ids: idsCStringArray,
            recursive: recursive
        });
        this.log('Node.js: call delete_many');
        const response = this.lib.delete_many(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_delete_many_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.affectedrows;
    }
    delete_many_async({ collectionname, query = "", ids = [], recursive = false }) {
        this.log('Node.js: delete_many invoked');
        const idsCStringArray = ids.map(id => ref.allocCString(id));
        return new Promise((resolve, reject) => {
            const req = new DeleteManyRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                query: ref.allocCString(query),
                ids: idsCStringArray,
                recursive: recursive
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(DeleteManyResponseWrapper)], (responsePtr) => {
                this.log('Node.js: delete_many_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.affectedrows);
                }
                this.lib.free_delete_many_response(responsePtr);
            });

            this.log('Node.js: call delete_many_async');
            this.lib.delete_many_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('DeleteMany failed'));
                }
            });
        });
    }
    download({ collectionname, id, folder, filename }) {
        this.log('Node.js: download invoked');
        const req = new DownloadRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            id: ref.allocCString(id),
            folder: ref.allocCString(folder),
            filename: ref.allocCString(filename)
        });
        this.log('Node.js: call download_async');
        const response = this.lib.download(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_download_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.filename;
    }
    download_async({ collectionname, id, folder, filename }) {
        this.log('Node.js: download invoked');
        return new Promise((resolve, reject) => {
            const req = new DownloadRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                id: ref.allocCString(id),
                folder: ref.allocCString(folder),
                filename: ref.allocCString(filename)
            });
            this.log('Node.js: create callback');
            const callback = ffi.Callback('void', [koffi.pointer(DownloadResponseWrapper)], (responsePtr) => {
                this.log('Node.js: download_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.filename);
                }
                this.lib.free_download_response(responsePtr);
            });

            this.log('Node.js: call download_async');
            this.lib.download_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Download failed'));
                }
            });
        });
    }
    upload({ filepath, filename, mimetype, metadata, collectionname }) {
        this.log('Node.js: upload invoked');
        const req = new UploadRequestWrapper({
            filepath: ref.allocCString(filepath),
            filename: ref.allocCString(filename),
            mimetype: ref.allocCString(mimetype),
            metadata: ref.allocCString(metadata),
            collectionname: ref.allocCString(collectionname)
        });
        this.log('Node.js: call upload_async');
        const response = this.lib.upload(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_upload_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.id;
    }
    upload_async({ filepath, filename, mimetype, metadata, collectionname }) {
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
            const callback = ffi.Callback('void', [koffi.pointer(UploadResponseWrapper)], (responsePtr) => {
                this.log('Node.js: upload_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.id);
                }
                this.lib.free_upload_response(responsePtr);
            });

            this.log('Node.js: call upload_async');
            this.lib.upload_async.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Upload failed'));
                }
            });
        });
    }
    watches = {}
    watch({ collectionname, paths }, callback) {
        this.log('Node.js: watch invoked');
        const req = new WatchRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            paths: ref.allocCString(paths)
        });
        this.log('Node.js: call watch');
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
                // this.log('Node.js: call next');
                const responsePtr = this.lib.next_watch_event(ref.allocCString(watchid));
                const result = responsePtr.deref();
                if (result.id != null && result.id != "") {
                    hadone = true;
                    let event = {
                        id: result.id,
                        operation: result.operation,
                        document: JSON.parse(result.document),
                    }
                    // this.log('Node.js: call next had result', event);
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
        this.log('Node.js: watch invoked');
        return new Promise((resolve, reject) => {
            this.log('Node.js: create event_callbackPtr');
            const event_callbackPtr = ffi.Callback('void', ['string'], (data) => {
                this.log('Node.js: watch_async event callback');
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

            this.log('Node.js: create callback');
            const callbackPtr = ffi.Callback('void', [koffi.pointer(WatchResponseWrapper)], (responsePtr) => {
                this.log('Node.js: watch_async callback');
                const response = JSON.parse(JSON.stringify(responsePtr.deref()));
                if (!response.success) {
                    const errorMsg = response.error;
                    reject(new ClientError(errorMsg));
                } else {
                    resolve(response.watchid);
                }
                this.lib.free_watch_response(responsePtr);
            });

            this.log('Node.js: call watch_async');
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
        this.log('Node.js: register queue invoked');
        const req = new RegisterQueueRequestWrapper({
            queuename: ref.allocCString(queuename)
        });
        this.log('Node.js: call register_queue');
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
                // this.log('Node.js: call next');
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
                    // this.log('Node.js: call next had result', event);
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
        this.log('Node.js: register exchange invoked');
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
        this.log('Node.js: call register_exchange');
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
                    // this.log('Node.js: call next');
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
                        // this.log('Node.js: call next had result', event);
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
        this.log('Node.js: push_workitem invoked');
        // if nextrun is not null and nextrun is a date
        if (nextrun != null && nextrun instanceof Date) {
            this.log('Node.js: nextrun before', nextrun);
            // then convert nextrun to a number ( POSIX time )
            nextrun = Math.floor(nextrun.getTime() / 1000); // Convert to seconds
        } else {
            nextrun = 0;
        }
        let filelist = [];
        this.log('Node.js: nextrun after', nextrun);
        for(let i = 0; i < files.length; i++) {
            const fileinstance = new WorkitemFileWrapper({
                filename : ref.allocCString(files[i]),
                id : ref.allocCString(""),
                file : Buffer.from("")
            });
            filelist.push(fileinstance.ref());
        }

        // const workitemFileInstance = new WorkitemFileWrapper({
        //     filename: 'example.txt',
        //     id: 'file-id',
        //     compressed: false,
        //     file: Buffer.from([/* file data as bytes */])
        // });

        const req = new PushWorkitemRequestWrapper({
            wiq: ref.allocCString(wiq),
            wiqid: ref.allocCString(wiqid),
            name: ref.allocCString(name),
            payload: ref.allocCString(payload),
            nextrun: nextrun,
            success_wiqid: ref.allocCString(success_wiqid),
            failed_wiqid: ref.allocCString(failed_wiqid),
            success_wiq: ref.allocCString(success_wiq),
            failed_wiq: ref.allocCString(failed_wiq),
            priority: priority,
            files: filelist,
            files_len: filelist.length
            // files: [workitemFileInstance.ref()]
        });
        this.log('Node.js: call push_workitem');
        const response = this.lib.push_workitem(this.client, req.ref());
        const result = JSON.parse(JSON.stringify(response.deref()));
        this.lib.free_push_workitem_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.success;
    }
    pop_workitem({ wiq = "", wiqid = "", downloadfolder = "" }) {
        this.log('Node.js: pop_workitem invoked');
        const req = new PopWorkitemRequestWrapper({
            wiq: ref.allocCString(wiq),
            wiqid: ref.allocCString(wiqid)
        });
        this.log('Node.js: call pop_workitem');
        const _downloadfolder = ref.allocCString(downloadfolder);
        const response = this.lib.pop_workitem(this.client, req.ref(), _downloadfolder);

        this.log('Node.js: pop_workitem deref');
        const _result = response.deref();
        const result = JSON.parse(JSON.stringify(_result));
        if(_result.workitem != null && _result.workitem.isNull() == false) {
            this.log('Node.js: workitem deref');
            let workitem = _result.workitem.deref();
            result.workitem = JSON.parse(JSON.stringify(workitem));

            // let _files = workitem.files.deref();
            let _files = workitem.files;
            let addr = _files.ref().address();
            let addrashex = addr.toString(16);            
            // this.log('Node.js: workitem files ref: [0x' + addrashex + "] files_len:", workitem.files_len);
            // this.log('Node.js: workitem files deref', workitem.files.length);
            const files = [];
            for(let i = 0; i < workitem.files_len; i++) {
                const file = JSON.parse(JSON.stringify(workitem.files[i].deref()));
                // // this.log('Node.js: workitem file deref', file);
                // const fileInstance = {
                //     filename: file.filename,
                //     id: file.id,
                //     compressed: file.compressed,
                //     file: file.file.buffer
                // };
                // this.log('Node.js: fileInstance', fileInstance);
                delete file.compressed;
                delete file.file;
                files.push(file);
            }
            result.workitem.files = files;
        } else {
            result.workitem = null;
        }
        this.lib.free_pop_workitem_response(response);
        if (!result.success) {
            const errorMsg = result.error;
            throw new ClientError(errorMsg);
        }
        return result.workitem;
    }

}

module.exports = {
    Client,
    ClientError,
    LibraryLoadError,
    ClientCreationError
};
