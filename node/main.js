const StructType = require('ref-struct-napi');
const ffi = require('ffi-napi');
const ref = require('ref-napi');
const path = require('path');
const fs = require('fs');

const CString = ref.types.CString;
const voidPtr = ref.refType(ref.types.void);
const bool = ref.types.bool;
const int = ref.types.int;

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

// Function to load the correct library file based on the operating system
function loadLibrary() {
    let libDir = path.join(__dirname, 'lib');
    let libPath;
    switch (process.platform) {
        case 'win32':
            libPath = path.join(libDir, 'libopeniap.dll');
            break;
        case 'darwin':
            libPath = path.join(libDir, 'libopeniap.dylib');
            break;
        default:
            libPath = path.join(libDir, 'libopeniap.so');
            break;
    }
    if(!fs.existsSync(libPath)) {
        libDir = path.join(__dirname, '../target/debug/');
        switch (process.platform) {
            case 'win32':
                libPath = path.join(libDir, 'libopeniap.dll');
                break;
            case 'darwin':
                libPath = path.join(libDir, 'libopeniap.dylib');
                break;
            default:
                libPath = path.join(libDir, 'libopeniap.so');
                break;
        }
    
    }

    try {
        return ffi.Library(libPath, {
            'client_connect': ['void', [CString, 'pointer']],
            'free_client': ['void', [ClientWrapperPtr]],
            'client_signin': ['void', [voidPtr, SigninRequestWrapperPtr, 'pointer']],
            'free_signin_response': ['void', [SigninResponseWrapperPtr]],
            'client_query': ['void', [voidPtr, QueryRequestWrapperPtr, 'pointer']],
            'free_query_response': ['void', [QueryResponseWrapperPtr]],
            'client_download': ['void', [voidPtr, DownloadRequestWrapperPtr, 'pointer']],
            'free_download_response': ['void', [DownloadResponseWrapperPtr]],
            'client_upload': ['void', [voidPtr, UploadRequestWrapperPtr, 'pointer']],
            'free_upload_response': ['void', [UploadResponseWrapperPtr]],
            'client_watch': ['void', [voidPtr, WatchRequestWrapperPtr, 'pointer', 'pointer']],
            'free_watch_response': ['void', [WatchResponseWrapperPtr]],
            'client_unwatch': [UnWatchResponseWrapperPtr, [voidPtr, CString]],
            'free_unwatch_response': ['void', [UnWatchResponseWrapperPtr]],
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

    connect(url) {
        return new Promise((resolve, reject) => {
            try {
                const callback = ffi.Callback('void', [ClientWrapperPtr], (clientPtr) => {
                    console.log('Node.js: Callback invoked');
                    try {
                        this.client = clientPtr;
                        const clientres = clientPtr.deref();
                        console.log('Node.js: Client result');
                        if (!clientres.success) {
                            reject(new ClientCreationError(clientres.error));
                        } else {
                            resolve(clientPtr);
                        }
                    } catch (error) {
                        reject(new ClientCreationError(error.message));                        
                    }
                });
                console.log('Node.js: Calling client_connect');
                this.lib.client_connect(url, callback);
                console.log('Node.js: client_connect called');
            } catch (error) {
                reject(new ClientCreationError(error.message));
            }
        });
    }

    signin(username, password) {
        console.log('Node.js: signin invoked');
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
    
            console.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(SigninResponseWrapper)], (responsePtr) => {
                console.log('Node.js: client_signin callback');
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
    
            console.log('Node.js: call client_signin');
            this.lib.client_signin.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Signin failed or user is null'));
                }
            });
        });
    }
    

    query({collectionname, query, projection, orderby, queryas, explain, skip, top}) {
        console.log('Node.js: query invoked');
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
            console.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(QueryResponseWrapper)], (responsePtr) => {
                console.log('Node.js: client_query callback');
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
                this.lib.free_query_response(responsePtr);
            });

            console.log('Node.js: call client_query');
            this.lib.client_query.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Query failed'));
                }
            });
        });
    }
    download({collectionname, id, folder, filename}) {
        console.log('Node.js: download invoked');
        return new Promise((resolve, reject) => {
            const req = new DownloadRequestWrapper({
                collectionname: ref.allocCString(collectionname),
                id: ref.allocCString(id),
                folder: ref.allocCString(folder),
                filename: ref.allocCString(filename)
            });
            console.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(DownloadResponseWrapper)], (responsePtr) => {
                console.log('Node.js: client_download callback');
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

            console.log('Node.js: call client_download');
            this.lib.client_download.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Download failed'));
                }
            });
        });
    }
    upload({filepath, filename, mimetype, metadata, collectionname}) {
        console.log('Node.js: upload invoked');
        return new Promise((resolve, reject) => {
            const req = new UploadRequestWrapper({
                filepath: ref.allocCString(filepath),
                filename: ref.allocCString(filename),
                mimetype: ref.allocCString(mimetype),
                metadata: ref.allocCString(metadata),
                collectionname: ref.allocCString(collectionname)
            });
            console.log('Node.js: create callback');
            const callback = ffi.Callback('void', [ref.refType(UploadResponseWrapper)], (responsePtr) => {
                console.log('Node.js: client_upload callback');
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

            console.log('Node.js: call client_upload');
            this.lib.client_upload.async(this.client, req.ref(), callback, (err) => {
                if (err) {
                    reject(new ClientError('Upload failed'));
                }
            });
        });
    }
    watch({collectionname, paths}, callback) {
        console.log('Node.js: watch invoked');
        return new Promise((resolve, reject) => {
            console.log('Node.js: create event_callbackPtr');
            const event_callbackPtr = ffi.Callback('void', ['string'], (data) => {
                console.log('Node.js: client_watch event callback');
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

            console.log('Node.js: create callback');
            const callbackPtr = ffi.Callback('void', [ref.refType(WatchResponseWrapper)], (responsePtr) => {
                console.log('Node.js: client_watch callback');
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

            console.log('Node.js: call client_watch');
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
