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

const DownloadResponseWrapper = StructType({
    success: bool,
    filename: CString,
    error: CString
});
const DownloadResponseWrapperPtr = ref.refType(DownloadResponseWrapper);

const DownloadRequestWrapper = StructType({
    collectionname: CString,
    id: CString,
    folder: CString,
    filename: CString
});
const DownloadRequestWrapperPtr = ref.refType(DownloadRequestWrapper);

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
            'client_connect': [ClientWrapperPtr, [CString]],
            'free_client': ['void', [ClientWrapperPtr]],
            'client_signin': [SigninResponseWrapperPtr, [voidPtr, SigninRequestWrapperPtr]],
            'free_signin_response': ['void', [SigninResponseWrapperPtr]],
            'client_query': [QueryResponseWrapperPtr, [voidPtr, QueryRequestWrapperPtr]],
            'free_query_response': ['void', [QueryResponseWrapperPtr]],
            'client_download': [DownloadResponseWrapperPtr, [voidPtr, DownloadRequestWrapperPtr]],
            'free_download_response': ['void', [DownloadResponseWrapperPtr]],
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
    constructor(url = "") {
        this.lib = loadLibrary();
        this.client = this.createClient(url);
    }

    createClient(url) {
        const client = this.lib.client_connect(url);
        const clientres = client.deref();
        if (!clientres.success) {
            throw new ClientCreationError(clientres.error);
        }
        return client;
    }

    signin(username, password) {
        let jwt = "";
        if(username == null) username = '';
        if(password == null) password = '';
        if(username != "" && password == "") {
            jwt = username;
            username = "";
        }
        // Allocate C strings for the SigninRequestWrapper fields
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

        // Call the client_signin function
        const response = this.lib.client_signin(this.client, req.ref());
        if (ref.isNull(response)) {
            throw new ClientError('Signin failed or user is null');
        }
        const Obj = response.deref();
        const result = {
            success: Obj.success,
            jwt: Obj.jwt,
            error: Obj.error
        };
        this.lib.free_signin_response(response);
        return result;
    }

    query({collectionname, query, projection, orderby, queryas, explain, skip, top}) {
        // Allocate C strings for the QueryRequestWrapper fields
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
        const response = this.lib.client_query(this.client, req.ref());
        if (ref.isNull(response)) {
            throw new ClientError('Query failed');
        }
        const Obj = response.deref();
        const result = {
            success: Obj.success,
            results: JSON.parse(Obj.results),
            error: Obj.error
        };
        this.lib.free_query_response(response);
        return result;

    }
    download({collectionname, id, folder, filename}) {
        // Allocate C strings for the DownloadRequestWrapper fields
        const req = new DownloadRequestWrapper({
            collectionname: ref.allocCString(collectionname),
            id: ref.allocCString(id),
            folder: ref.allocCString(folder),
            filename: ref.allocCString(filename)
        });
        const response = this.lib.client_download(this.client, req.ref());
        if (ref.isNull(response)) {
            throw new ClientError('Download failed');
        }
        const Obj = response.deref();
        const result = {
            success: Obj.success,
            filename: Obj.filename,
            error: Obj.error
        };
        this.lib.free_download_response(response);
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
