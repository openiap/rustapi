const StructType = require('ref-struct-napi');
const ffi = require('ffi-napi');
const ref = require('ref-napi');
const path = require('path');

const CString = ref.types.CString;
const voidPtr = ref.refType(ref.types.void);
const bool = ref.types.bool;

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

// Function to load the correct library file based on the operating system
function loadLibrary() {
    const libDir = path.join(__dirname, 'lib');
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

    try {
        return ffi.Library(libPath, {
            'client_connect': [ClientWrapperPtr, [CString]],
            'free_client': ['void', [ClientWrapperPtr]],
            'client_signin': [SigninResponseWrapperPtr, [voidPtr, SigninRequestWrapperPtr]],
            'free_signin_response': ['void', [SigninResponseWrapperPtr]]
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
    constructor(url) {
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
        const user = this.lib.client_signin(this.client, req.ref());
        if (ref.isNull(user)) {
            throw new ClientError('Signin failed or user is null');
        }
        const userObj = user.deref();
        const result = {
            success: userObj.success,
            jwt: userObj.jwt,
            error: userObj.error
        };
        this.lib.free_signin_response(user);
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
