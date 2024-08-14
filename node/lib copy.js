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
const uint64 = ref.types.uint64;
const size_t = ref.types.size_t;









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

const WatchEvent = StructType({
    id: CString,
    operation: CString,
    document: CString,
})
const WatchEventPtr = ref.refType(WatchEvent);

const RegisterQueueRequestWrapper = StructType({
    queuename: CString
});
const RegisterQueueRequestWrapperPtr = ref.refType(RegisterQueueRequestWrapper);
const RegisterQueueResponseWrapper = StructType({
    success: bool,
    queuename: CString,
    error: CString
});
const RegisterQueueResponseWrapperPtr = ref.refType(RegisterQueueResponseWrapper);

const RegisterExchangeRequestWrapper = StructType({
    exchangename: CString,
    algorithm: CString,
    routingkey: CString,
    addqueue: bool,
});
const RegisterExchangeRequestWrapperPtr = ref.refType(RegisterExchangeRequestWrapper);
const RegisterExchangeResponseWrapper = StructType({
    success: bool,
    queuename: CString,
    error: CString
});
const RegisterExchangeResponseWrapperPtr = ref.refType(RegisterExchangeResponseWrapper);

const UnRegisterQueueResponseWrapper = StructType({
    success: bool,
    error: CString
});
const UnRegisterQueueResponseWrapperPtr = ref.refType(UnRegisterQueueResponseWrapper);

const QueueEvent = StructType({
    queuename: CString,
    correlation_id: CString,
    replyto: CString,
    routingkey: CString,
    exchangename: CString,
    data: CString,
});
const QueueEventPtr = ref.refType(QueueEvent);


const WorkitemFileWrapper = StructType({
    filename: CString,
    id: CString,
    compressed: bool,
    file: ArrayType(ref.types.uint8) // This represents Vec<u8> in Rust
});
const WorkitemFileWrapperPtr = ref.refType(WorkitemFileWrapper);
const WorkitemFileWrapperPtrArray = ArrayType(WorkitemFileWrapperPtr);

const WorkitemWrapper = StructType({
    id: CString,
    name: CString,
    payload: CString,
    priority: int,
    nextrun: uint64,
    lastrun: uint64,
    /// files: ref.refType(WorkitemFileWrapperPtrArray),
    files: WorkitemFileWrapperPtrArray,
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
const WorkitemWrapperPtr = ref.refType(WorkitemWrapper);
const PushWorkitemRequestWrapper = StructType({
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
    files: WorkitemFileWrapperPtrArray,
    files_len: int,
});
const PushWorkitemRequestWrapperPtr = ref.refType(PushWorkitemRequestWrapper);
const PushWorkitemResponseWrapper = StructType({
    success: bool,
    error: CString
});
const PushWorkitemResponseWrapperPtr = ref.refType(PushWorkitemResponseWrapper);

const PopWorkitemRequestWrapper = StructType({
    wiq: CString,
    wiqid: CString,
});
const PopWorkitemRequestWrapperPtr = ref.refType(PopWorkitemRequestWrapper);
const PopWorkitemResponseWrapper = StructType({
    success: bool,
    error: CString,
    workitem: WorkitemWrapperPtr
});
const PopWorkitemResponseWrapperPtr = ref.refType(PopWorkitemResponseWrapper);


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
            'watch': [WatchResponseWrapperPtr, [voidPtr, WatchRequestWrapperPtr]],
            'next_watch_event': [WatchEventPtr, [CString]],
            'watch_async': ['void', [voidPtr, WatchRequestWrapperPtr, 'pointer', 'pointer']],
            'free_watch_response': ['void', [WatchResponseWrapperPtr]],
            'free_watch_event': ['void', [WatchEventPtr]],
            'unwatch': [UnWatchResponseWrapperPtr, [voidPtr, CString]],
            'free_unwatch_response': ['void', [UnWatchResponseWrapperPtr]],
            'register_queue': [RegisterQueueResponseWrapperPtr, [voidPtr, RegisterQueueRequestWrapperPtr]],
            'free_register_queue_response': ['void', [RegisterQueueResponseWrapperPtr]],
            'register_exchange': [RegisterExchangeResponseWrapperPtr, [voidPtr, RegisterExchangeRequestWrapperPtr]],
            'free_register_exchange_response': ['void', [RegisterExchangeResponseWrapperPtr]],
            'unregister_queue': [UnRegisterQueueResponseWrapperPtr, [voidPtr, CString]],
            'free_unregister_queue_response': ['void', [UnRegisterQueueResponseWrapperPtr]],
            'next_queue_event': [QueueEventPtr, [CString]],
            'free_queue_event': ['void', [QueueEventPtr]],

            'push_workitem': [PushWorkitemResponseWrapperPtr, [voidPtr, PushWorkitemRequestWrapperPtr]],
            'free_push_workitem_response': ['void', [PushWorkitemResponseWrapperPtr]],
            'pop_workitem': [PopWorkitemResponseWrapperPtr, [voidPtr, PopWorkitemRequestWrapperPtr, CString]],
            'free_pop_workitem_response': ['void', [PopWorkitemResponseWrapperPtr]],

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
            const callbackPtr = ffi.Callback('void', [ref.refType(WatchResponseWrapper)], (responsePtr) => {
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
