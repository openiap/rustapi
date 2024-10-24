import ctypes
import json
import os
import sys
from ctypes import CDLL, Structure, c_char_p, c_void_p, c_bool, c_int, c_uint64, CFUNCTYPE, POINTER, byref, pointer
import threading
import time

# Define the ctypes types for C types
CALLBACK = CFUNCTYPE(None, c_char_p)

# Define the ClientWrapper struct
class ClientWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p),
                ("client", c_void_p)]
    
# Define the ClientEventWrapper struct
class ClientEventWrapper(Structure):
    _fields_ = [("event", c_char_p),
                ("reason", c_char_p)]

class ClientEventResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("eventid", c_char_p),
                ("error", c_char_p)]

ClientEventCallback = CFUNCTYPE(None, POINTER(ClientEventWrapper))

class OffClientEventResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]


# Define the ConnectResponseWrapper struct
class ConnectResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]

ConnectCallback = CFUNCTYPE(None, POINTER(ConnectResponseWrapper))

# Define the SigninRequestWrapper struct
class SigninRequestWrapper(Structure):
    _fields_ = [("username", c_char_p),
                ("password", c_char_p),
                ("jwt", c_char_p),
                ("agent", c_char_p),
                ("version", c_char_p),
                ("longtoken", c_bool),
                ("validateonly", c_bool),
                ("ping", c_bool)]

# Define the SigninResponseWrapper struct
class SigninResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("jwt", c_char_p),
                ("error", c_char_p)]
SigninCallback = CFUNCTYPE(None, POINTER(SigninResponseWrapper))

class ColCollationWrapper(Structure):
    _fields_ = [("locale", c_char_p),
                ("case_level", c_bool),
                ("case_first", c_char_p),
                ("strength", c_int),
                ("numeric_ordering", c_bool),
                ("alternate", c_char_p),
                ("max_variable", c_char_p),
                ("backwards", c_bool)]
class ColTimeseriesWrapper(Structure):
    _fields_ = [("time_field", c_char_p),
                ("meta_field", c_char_p),
                ("granularity", c_char_p)]
class CreateCollectionRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("collation", POINTER(ColCollationWrapper)),
                ("timeseries", POINTER(ColTimeseriesWrapper)),
                ("expire_after_seconds", c_int),
                ("change_stream_pre_and_post_images", c_bool),
                ("capped", c_bool),
                ("max", c_int),
                ("size", c_int)]
    
class QueryRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("query", c_char_p),
                ("projection", c_char_p),
                ("orderby", c_char_p),
                ("queryas", c_char_p),
                ("explain", c_bool),
                ("skip", c_int),
                ("top", c_int)]
class QueryResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("results", c_char_p),
                ("error", c_char_p)]
QueryCallback = CFUNCTYPE(None, POINTER(QueryResponseWrapper))

class AggregateRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("aggregates", c_char_p),
                ("queryas", c_char_p),
                ("hint", c_char_p),
                ("explain", c_bool)]
class AggregateResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("results", c_char_p),
                ("error", c_char_p)]
AggregateCallback = CFUNCTYPE(None, POINTER(AggregateResponseWrapper))

class CountRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("query", c_char_p),
                ("queryas", c_char_p),
                ("explain", c_bool)]
class CountResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("result", c_int),
                ("error", c_char_p)]
CountCallback = CFUNCTYPE(None, POINTER(CountResponseWrapper))

class DistinctRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("field", c_char_p),
                ("query", c_char_p),
                ("queryas", c_char_p),
                ("explain", c_bool)]
class DistinctResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ('results', POINTER(c_char_p)),
                ("error", c_char_p),
                ('results_len', c_int),]
DistinctCallback = CFUNCTYPE(None, POINTER(DistinctResponseWrapper))

class InsertOneRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("item", c_char_p),
                ("w", c_int),
                ("j", c_bool)]
class InsertOneResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("result", c_char_p),
                ("error", c_char_p)]
InsertOneCallback = CFUNCTYPE(None, POINTER(InsertOneResponseWrapper))

class InsertManyRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("items", c_char_p),
                ("w", c_int),
                ("j", c_bool),
                ("skipresults", c_bool)]
class InsertManyResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("result", c_char_p),
                ("error", c_char_p)]
InsertManyCallback = CFUNCTYPE(None, POINTER(InsertManyResponseWrapper))

class UpdateOneRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("item", c_char_p),
                ("w", c_int),
                ("j", c_bool)]
class UpdateOneResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("result", c_char_p),
                ("error", c_char_p)]
UpdateOneCallback = CFUNCTYPE(None, POINTER(UpdateOneResponseWrapper))
    
class InsertOrUpdateOneRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("uniqeness", c_char_p),
                ("item", c_char_p),
                ("w", c_int),
                ("j", c_bool)]
class InsertOrUpdateOneResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("result", c_char_p),
                ("error", c_char_p)]
InsertOrUpdateOneCallback = CFUNCTYPE(None, POINTER(InsertOrUpdateOneResponseWrapper))
    
class DownloadRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("id", c_char_p),
                ("folder", c_char_p),
                ("filename", c_char_p)]
class DownloadResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("filename", c_char_p),
                ("error", c_char_p)]
DownloadCallback = CFUNCTYPE(None, POINTER(DownloadResponseWrapper))

class UploadRequestWrapper(Structure):
    _fields_ = [("filepath", c_char_p),
                ("filename", c_char_p),
                ("mimetype", c_char_p),
                ("metadata", c_char_p),
                ("collectionname", c_char_p)]
class UploadResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("id", c_char_p),
                ("error", c_char_p)]
UploadCallback = CFUNCTYPE(None, POINTER(UploadResponseWrapper))

class WorkitemFileWrapper(Structure):
    _fields_ = [("filename", c_char_p),
                ("id", c_char_p),
                ("compressed", c_bool),
                #("file", c_char_p)
                ]
class WorkitemWrapper(Structure):
    _fields_ = [("id", c_char_p),
                ("name", c_char_p),
                ("payload", c_char_p),
                ("priority", c_int),
                ("nextrun", c_uint64),
                ("lastrun", c_uint64),
                ("files", POINTER(POINTER(WorkitemFileWrapper))),
                ("files_len", c_int),
                ("state", c_char_p),
                ("wiq", c_char_p),
                ("wiqid", c_char_p),
                ("retries", c_int),
                ("username", c_char_p),
                ("success_wiqid", c_char_p),
                ("failed_wiqid", c_char_p),
                ("success_wiq", c_char_p),
                ("failed_wiq", c_char_p),
                ("errormessage", c_char_p),
                ("errorsource", c_char_p),
                ("errortype", c_char_p),
                ]
class PushWorkitemRequestWrapper(Structure):
    _fields_ = [("wiq", c_char_p),
                ("wiqid", c_char_p),
                ("name",  c_char_p),
                ("payload",  c_char_p),
                ("nextrun", c_uint64),
                ("success_wiqid",  c_char_p),
                ("failed_wiqid",  c_char_p),
                ("success_wiq",  c_char_p),
                ("failed_wiq",  c_char_p),
                ("priority", c_int),
                ("files", POINTER(POINTER(WorkitemFileWrapper))),
                ("files_len", c_int)
    ]
class PushWorkitemResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p),
                ("workitem", POINTER(WorkitemWrapper)),
                ]
PushWorkitemCallback = CFUNCTYPE(None, POINTER(PushWorkitemResponseWrapper))

class PopWorkitemRequestWrapper(Structure):
    _fields_ = [("wiq", c_char_p),
                ("wiqid", c_char_p)]
class PopWorkitemResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p),
                ("workitem", POINTER(WorkitemWrapper))]
PopWorkitemCallback = CFUNCTYPE(None, POINTER(PopWorkitemResponseWrapper))

class UpdateWorkitemRequestWrapper(Structure):
    _fields_ = [("workitem", POINTER(WorkitemWrapper)),
                ("ignoremaxretries", c_bool),
                ("files", POINTER(POINTER(WorkitemFileWrapper))),
                ("files_len", c_int)
                ]
class UpdateWorkitemResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p),
                ("workitem", POINTER(WorkitemWrapper))]
UpdateWorkitemCallback = CFUNCTYPE(None, POINTER(UpdateWorkitemResponseWrapper))

class DeleteWorkitemRequestWrapper(Structure):
    _fields_ = [("id", c_char_p)]
class DeleteWorkitemResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]
DeleteWorkitemCallback = CFUNCTYPE(None, POINTER(DeleteWorkitemResponseWrapper))

class DeleteOneRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("id", c_char_p),
                ("recursive", c_bool)]
class DeleteOneResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("affectedrows", c_int),
                ("error", c_char_p)]
DeleteOneCallback = CFUNCTYPE(None, POINTER(DeleteOneResponseWrapper))

class DeleteManyRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("query", c_char_p),
                ("recursive", c_bool),
                # ids is a list of strings
                ("ids", POINTER(c_char_p))]
class DeleteManyResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("affectedrows", c_int),
                ("error", c_char_p)]
DeleteManyCallback = CFUNCTYPE(None, POINTER(DeleteManyResponseWrapper))

class WatchEventWrapper(Structure):
    _fields_ = [("id", c_char_p),
                ("operation", c_char_p),
                ("document", c_char_p)]
WatchEventCallback = CFUNCTYPE(None, POINTER(WatchEventWrapper))
class WatchRequestWrapper(Structure):
    _fields_ = [("collectionname", c_char_p),
                ("paths", c_char_p)]
class WatchResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("watchid", c_char_p),
                ("error", c_char_p)]
WatchCallback = CFUNCTYPE(None, POINTER(WatchResponseWrapper))

class UnWatchResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]

class RegisterQueueRequestWrapper(Structure):
    _fields_ = [("queuename", c_char_p)]
class RegisterQueueResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("queuename", c_char_p),
                ("error", c_char_p)]
class QueueEventWrapper(Structure):
    _fields_ = [("queuename", c_char_p),
                ("correlation_id", c_char_p),
                ("replyto", c_char_p),
                ("routingkey", c_char_p),
                ("exchangename", c_char_p),
                ("data", c_char_p) ]
RegisterQueueCallback = CFUNCTYPE(None, POINTER(RegisterQueueResponseWrapper))
QueueEventCallback = CFUNCTYPE(None, POINTER(QueueEventWrapper))

class RegisterExchangeRequestWrapper(Structure):
    _fields_ = [("exchangename", c_char_p),
                ("algorithm", c_char_p),
                ("routingkey", c_char_p),
                ("addqueue", c_bool)]
class RegisterExchangeResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("queuename", c_char_p),
                ("error", c_char_p)]

class UnRegisterQueueResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]

class QueueMessageRequestWrapper(Structure):
    _fields_ = [("queuename", c_char_p),
                ("correlation_id", c_char_p),
                ("replyto", c_char_p),
                ("routingkey", c_char_p),
                ("exchangename", c_char_p),
                ("data", c_char_p),
                ("striptoken", c_bool),
                ("expiration", c_int)]
class QueueMessageResponseWrapper(Structure):
    _fields_ = [("success", c_bool),
                ("error", c_char_p)]

# Custom exception classes
class ClientError(Exception):
    """Base class for exceptions in this module."""
    pass

class LibraryLoadError(ClientError):
    """Exception raised for errors in loading the shared library."""
    def __init__(self, message):
        self.message = message
        super().__init__(self.message)

class ClientCreationError(ClientError):
    """Exception raised for errors in creating the client."""
    def __init__(self, message):
        self.message = message
        super().__init__(self.message)
def encode_files(req, files):
    if files is None or len(files) == 0:
        req.files = None
        req.files_len = 0
        return None

    # Create an array of pointers to WorkitemFileWrapper
    cfile_pointers = (ctypes.POINTER(WorkitemFileWrapper) * len(files))()
    files_len = 0
    for i, f in enumerate(files):
        if f is None:
            print("File is None")
        elif isinstance(f, str):
            cfile = WorkitemFileWrapper(
                filename=c_char_p(f.encode('utf-8')),
                id=c_char_p(b''),
                compressed=False
            )
            cfile_pointers[i] = ctypes.pointer(cfile)
            files_len += 1
        else:
            cfile = WorkitemFileWrapper(
                filename=c_char_p(f["filename"].encode('utf-8')),
                id=c_char_p(f["id"].encode('utf-8')),
                compressed=f["compressed"]
            )
            cfile_pointers[i] = ctypes.pointer(cfile)
            files_len += 1

    # Assign the array of pointers to the req.files
    req.files = ctypes.cast(cfile_pointers, ctypes.POINTER(ctypes.POINTER(WorkitemFileWrapper)))
    # req.files_len = ctypes.c_size_t(files_len)
    req.files_len = files_len
    
    # Keep a reference to prevent garbage collection
    req._cfiles = cfile_pointers

def get_raw_bytes(c_char_p_ptr):
    """Access and print raw bytes for debugging."""
    if c_char_p_ptr:
        raw_bytes = ctypes.cast(c_char_p_ptr, ctypes.POINTER(ctypes.c_char * 256)).contents.raw
        print("Raw bytes:", raw_bytes)
        return raw_bytes
    return None

def decode_workitem(workitem_ptr):
    """Parses the WorkitemWrapper structure and returns a dictionary."""
    workitem = workitem_ptr.contents

    # get_raw_bytes(workitem_ptr)
    def safe_decode(c_char_p):
        if c_char_p:
            return c_char_p.decode('utf-8', errors='replace')
        return ""

    result = {
        "id": safe_decode(workitem.id),
        "name": safe_decode(workitem.name),
        "payload": safe_decode(workitem.payload),
        "priority": workitem.priority,
        "nextrun": workitem.nextrun,
        "lastrun": workitem.lastrun,
        "files": [],
        "files_len": workitem.files_len,
        "state": safe_decode(workitem.state),
        "wiq": safe_decode(workitem.wiq),
        "wiqid": safe_decode(workitem.wiqid),
        "retries": workitem.retries,
        "username": safe_decode(workitem.username),
        "success_wiqid": safe_decode(workitem.success_wiqid),
        "failed_wiqid": safe_decode(workitem.failed_wiqid),
        "success_wiq": safe_decode(workitem.success_wiq),
        "failed_wiq": safe_decode(workitem.failed_wiq),
        "errormessage": safe_decode(workitem.errormessage),
        "errorsource": safe_decode(workitem.errorsource),
        "errortype": safe_decode(workitem.errortype)
    }

    for i in range(workitem.files_len):
        file = workitem.files[i]
        file = file.contents

        # print(f"Raw filename pointer: {file}, filename: {file.filename}")  # Debugging line

        result["files"].append({
            "filename": safe_decode(file.filename),
            "id": safe_decode(file.id),
            "compressed": file.compressed
        })
    return result
def encode_workitem(workitem, files):
    """Encodes the workitem dictionary into a WorkitemWrapper structure."""
    req = WorkitemWrapper()
    req.id = c_char_p(workitem.get("id", "").encode('utf-8'))
    req.name = c_char_p(workitem.get("name", "").encode('utf-8'))
    req.payload = c_char_p(workitem.get("payload", "").encode('utf-8'))
    req.priority = workitem.get("priority", 0)
    req.nextrun = workitem.get("nextrun", 0)
    req.lastrun = workitem.get("lastrun", 0)
    req.state = c_char_p(workitem.get("state", "").encode('utf-8'))
    req.wiq = c_char_p(workitem.get("wiq", "").encode('utf-8'))
    req.wiqid = c_char_p(workitem.get("wiqid", "").encode('utf-8'))
    req.retries = workitem.get("retries", 0)
    req.username = c_char_p(workitem.get("username", "").encode('utf-8'))
    req.success_wiqid = c_char_p(workitem.get("success_wiqid", "").encode('utf-8'))
    req.failed_wiqid = c_char_p(workitem.get("failed_wiqid", "").encode('utf-8'))
    req.success_wiq = c_char_p(workitem.get("success_wiq", "").encode('utf-8'))
    req.failed_wiq = c_char_p(workitem.get("failed_wiq", "").encode('utf-8'))
    req.errormessage = c_char_p(workitem.get("errormessage", "").encode('utf-8'))
    req.errorsource = c_char_p(workitem.get("errorsource", "").encode('utf-8'))
    req.errortype = c_char_p(workitem.get("errortype", "").encode('utf-8'))

    encode_files(req, files)
    return req

class Client:
    def __init__(self):
        self.lib = self._load_library()
        self.tracing = False
        self.informing = False
        self.verbosing = False
        self.callbacks = []  # Keep a reference to the callbacks

        self.lib.create_client.argtypes = []
        self.lib.create_client.restype = POINTER(ClientWrapper)

        client_ptr = self.lib.create_client()
        response = client_ptr.contents
        if not response.success:
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            raise ClientError(f"Create client failed: {error_message}")
        self.client = client_ptr
        self.lib.client_set_agent_name(self.client, "python".encode('utf-8'))

    def free(self):
        if(self.client != None):
            self.trace("Freeing client")
            self.lib.free_client(self.client)
            self.client = None    
    def _load_library(self):
        # Determine the path to the shared library
        lib_dir = os.path.join(os.path.dirname(__file__), 'lib')
        architecture = os.uname().machine
        if sys.platform == 'win32':
            if architecture == 'x86_64':
                lib_file = 'openiap-windows-x64.dll'
            elif architecture == 'AMD64':
                lib_file = 'openiap-windows-i686.dll'
            else:
                raise LibraryLoadError("Unsupported architecture " + architecture)
        elif sys.platform == 'darwin':
            if architecture == 'x86_64':
                lib_file = 'libopeniap-macos-x64.dylib'
            elif architecture == 'arm64':
                lib_file = 'libopeniap-macos-armx64.dylib'
            else:
                raise LibraryLoadError("Unsupported architecture " + architecture)
        elif sys.platform == 'linux':
            if architecture == 'x86_64':
                # is Musl ?
                if os.path.exists('/lib/libc.musl-x86_64.so.1'):
                    lib_file = 'libopeniap-linux-musl-x64.a'
                else:
                    lib_file = 'libopeniap-linux-x64.so'
            elif architecture == 'aarch64':
                # is Musl ?
                if os.path.exists('/lib/libc.musl-aarch64.so.1'):
                    lib_file = 'libopeniap-linux-musl-armx64.a'
                else:
                    lib_file = 'libopeniap-linux-armx64.so'
            else:
                raise LibraryLoadError("Unsupported architecture " + architecture)
        elif sys.platform == 'freebsd':
            if architecture == 'x86_64':
                lib_file = 'libopeniap-freebsd-x64.so'
            else:
                raise LibraryLoadError("Unsupported architecture " + architecture)
        else:
            raise LibraryLoadError("Unsupported platform " + sys.platform)
        
        lib_path = os.path.join(lib_dir, lib_file)
        if not os.path.exists(lib_path):
            lib_dir = os.path.join(os.path.dirname(__file__), '..', 'lib')
            lib_path = os.path.join(lib_dir, lib_file)

        if not os.path.exists(lib_path):
            lib_file = 'libopeniap_clib.so' if sys.platform != 'win32' else 'libopeniap_clib.dll';
            if sys.platform == 'darwin':
                lib_file = 'libopeniap_clib.dylib'
            lib_dir = os.path.join(os.path.dirname(__file__), '../../target/debug/')
            lib_path = os.path.join(lib_dir, lib_file)

        # Load the Rust library
        try:
            print("Loading library " + lib_path)
            return CDLL(lib_path)
        except OSError as e:
            raise LibraryLoadError(f"Failed to load library: {e}")
        
    def enable_tracing(self, rust_log="", tracing=""):
        print("Calling enable_tracing", rust_log, tracing)
        self.lib.enable_tracing(rust_log.encode('utf-8'), tracing.encode('utf-8'))
        self.informing = True
        if "verbose" in rust_log:
            self.verbosing = True
        if "trace" in rust_log:
            self.tracing = True
    
    def on_client_event(self, callback = None):
        self.trace("Inside on_client_event")
        result = {"success": None, "eventid": None, "error": None}
        if not callable(callback):
            raise ValueError("Callback must be a callable function")

        self.counter = 0
        def client_event_callback(eventref):
            self.trace("Client Event callback invoked")
            event = eventref.contents
            str_event = ""
            str_reason = ""
            if event.event != None:
                str_event = event.event.decode('utf-8')
            if event.reason != None:
                str_reason = event.reason.decode('utf-8')
            result = {
                "event": str_event,
                "reason": str_reason
            }
            self.trace("Internal callback invoked", result)
            self.lib.free_client_event(eventref)
            self.counter += 1
            try:
                callback(result, self.counter)
            except Exception as e:
                self.trace("Error in callback", e)
        c_callback = ClientEventCallback(client_event_callback)
        self.callbacks.append(c_callback)

        self.lib.on_client_event_async.argtypes = [POINTER(ClientWrapper), ClientEventCallback]
        self.lib.on_client_event_async.restype = POINTER(ClientEventResponseWrapper);

        self.trace("Calling on_client_event_async")
        reqref = self.lib.on_client_event_async(self.client, c_callback)
        self.trace("on_client_event_async called")
        response = reqref.contents
        eventid = ""

        if not response.success:
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            self.lib.free_event_response(reqref)
            raise ClientError(f"Failed to register client event: {error_message}")
        eventid = response.eventid.decode('utf-8')
        self.lib.free_event_response(reqref)

        return eventid
    def off_client_event(self, eventid):
        self.trace("Inside off_client_event")
        self.lib.off_client_event.argtypes = [c_char_p]
        self.lib.off_client_event.restype = POINTER(OffClientEventResponseWrapper);

        self.trace("Calling off_client_event")
        reqref = self.lib.off_client_event(eventid.encode('utf-8'))
        self.trace("off_client_event called")
        response = reqref.contents

        if not response.success:
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            self.lib.free_off_event_response(reqref)
            raise ClientError(f"Failed to unregister client event: {error_message}")
        self.lib.free_off_event_response(reqref)
    def disable_tracing(self):
        self.trace("Calling disable_tracing")
        self.lib.disable_tracing()
        self.trace("disable_tracing called")
    def info(self, *args):
        if self.informing:
            print(*args)
    def verbose(self, *args):
        if self.verbosing:
            print(*args)
    def trace(self, *args):
        if self.tracing:
            print(*args)
    def connect(self, url=""):
        # Event to wait for the callback
        event = threading.Event()
        result = {"client": None, "error": None}

        def callback(client_ptr):
            try:
                client = client_ptr.contents
                self.trace("Callback invoked")
                if not client.success:
                    error_message = ctypes.cast(client.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientCreationError(f"Failed to create client: {error_message}")
                else:
                    result["client"] = client_ptr
            finally:
                event.set()

        cb = ConnectCallback(callback)

        self.trace("Calling connect_async")
        self.lib.connect_async(self.client, url.encode('utf-8'), cb)
        self.trace("connect_async called")

        # Wait for the callback to be invoked
        event.wait()

        if result["error"]:
            raise result["error"]
        return result["client"]
    
    def signin(self, username="", password=""):
        self.trace("Inside signin")
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Signin callback invoked")
                self.trace(response)
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Signin failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["jwt"] = ctypes.cast(response.jwt, c_char_p).value.decode('utf-8')
                self.lib.free_signin_response(response_ptr)
            finally:
                event.set()

        cb = SigninCallback(callback)

        # Prepare the SigninRequestWrapper
        jwt = ""
        if username and not password:
            jwt = username
            username = ""

        req = SigninRequestWrapper(
            username=username.encode('utf-8'),
            password=password.encode('utf-8'),
            jwt=jwt.encode('utf-8'),
            agent=b'node',
            version=b'',
            longtoken=False,
            validateonly=False,
            ping=False
        )

        self.trace("Calling signin_async")
        self.lib.signin_async(self.client, byref(req), cb)
        self.trace("signin_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        return {"success": result["success"], "jwt": result["jwt"]}
    
    def list_collections(self, includehist=False):
        self.trace("Inside list_collections")
        event = threading.Event()
        result = {"success": None, "collections": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("List collections callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"List collections failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["collections"] = ctypes.cast(response.results, c_char_p).value.decode('utf-8')
                self.lib.free_list_collections_response(response_ptr)
            finally:
                event.set()

        cb = QueryCallback(callback)

        self.trace("Calling list_collections_async")
        self.lib.list_collections_async(self.client, includehist, cb)
        self.trace("list_collections_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["collections"]
    def create_collection(self, collectionname="", collation=None, timeseries=None, expire_after_seconds=0, change_stream_pre_and_post_images=False, capped=False, max=0, size=0):
        self.trace("Inside create_collection")
        event = threading.Event()
        result = {"success": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Create collection callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Create collection failed: {error_message}")
                else:
                    result["success"] = response.success
                self.lib.free_create_collection_response(response_ptr)
            finally:
                event.set()

        cb = QueryCallback(callback)

        req = CreateCollectionRequestWrapper(
            collectionname=collectionname.encode('utf-8'),
            collation=collation,
            timeseries=timeseries,
            expire_after_seconds=expire_after_seconds,
            change_stream_pre_and_post_images=change_stream_pre_and_post_images,
            capped=capped,
            max=max,
            size=size
        )

        self.trace("Calling create_collection_async")
        self.lib.create_collection_async(self.client, byref(req), cb)
        self.trace("create_collection_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["success"]
    
    def drop_collection(self, collectionname=""):
        self.trace("Inside drop_collection")
        event = threading.Event()
        result = {"success": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Drop collection callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Drop collection failed: {error_message}")
                else:
                    result["success"] = response.success
                self.lib.free_drop_collection_response(response_ptr)
            finally:
                event.set()

        cb = QueryCallback(callback)

        self.trace("Calling drop_collection_async")
        self.lib.drop_collection_async(self.client, collectionname.encode('utf-8'), cb)
        self.trace("drop_collection_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["success"]


    def query(self, collectionname = "", query = "", projection = "", orderby = "", queryas = "", explain = False, skip = 0, top = 0):
        self.trace("Inside query")
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Query callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Query failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["results"] = ctypes.cast(response.results, c_char_p).value.decode('utf-8')
                self.lib.free_query_response(response_ptr)
            finally:
                event.set()

        cb = QueryCallback(callback)

        req = QueryRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                  query=c_char_p(query.encode('utf-8')),
                                  projection=c_char_p(projection.encode('utf-8')),
                                  orderby=c_char_p(orderby.encode('utf-8')),
                                  queryas=c_char_p(queryas.encode('utf-8')),
                                  explain=explain,
                                  skip=skip,
                                  top=top)
        
        self.trace("Calling query_async")
        self.lib.query_async(self.client, byref(req), cb)
        self.trace("query_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["results"]
    
    def aggregate(self, collectionname = "", aggregates = "", queryas = "", hint = "", explain = False):
        self.trace("Inside aggregate")
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Aggregate callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Aggregate failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["results"] = ctypes.cast(response.results, c_char_p).value.decode('utf-8')
                self.lib.free_aggregate_response(response_ptr)
            finally:
                event.set()

        cb = AggregateCallback(callback)

        req = AggregateRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                      aggregates=c_char_p(aggregates.encode('utf-8')),
                                      queryas=c_char_p(queryas.encode('utf-8')),
                                      hint=c_char_p(hint.encode('utf-8')),
                                      explain=explain)
        
        self.trace("Calling aggregate_async")
        self.lib.aggregate_async(self.client, byref(req), cb)
        self.trace("aggregate_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["results"]

    def count(self, collectionname = "", query = "", queryas = "", explain = False):
        self.trace("Inside count")
        event = threading.Event()
        result = {"success": None, "result": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Count callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Count failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["result"] = response.result
                self.lib.free_count_response(response_ptr)
            finally:
                event.set()

        cb = CountCallback(callback)

        req = CountRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                  query=c_char_p(query.encode('utf-8')),
                                  queryas=c_char_p(queryas.encode('utf-8')),
                                  explain=explain)
        
        self.trace("Calling count_async")
        self.lib.count_async(self.client, byref(req), cb)
        self.trace("count_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["result"]
    
    def distinct(self, collectionname="", field="", query="", queryas="", explain=False):
        self.trace("Inside distinct")
        event = threading.Event()
        result = {"success": None, "results": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Distinct callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Distinct failed: {error_message}")
                else:
                    result["success"] = response.success
                    results_array_ptr = response.results
                    results_len = response.results_len
                    results = []

                    for i in range(results_len):
                        cstr_ptr = results_array_ptr[i]
                        js_string = ctypes.cast(cstr_ptr, c_char_p).value.decode('utf-8')
                        results.append(js_string)

                    result["results"] = results
                self.lib.free_distinct_response(response_ptr)
            finally:
                event.set()

        cb = DistinctCallback(callback)

        req = DistinctRequestWrapper(
            collectionname=c_char_p(collectionname.encode('utf-8')),
            field=c_char_p(field.encode('utf-8')),
            query=c_char_p(query.encode('utf-8')),
            queryas=c_char_p(queryas.encode('utf-8')),
            explain=explain
        )

        self.trace("Calling distinct_async")
        self.lib.distinct_async(self.client, byref(req), cb)
        self.trace("distinct_async called")

        event.wait()

        if result["error"]:
            raise result["error"]

        return result["results"]

    
    def insert_one(self, collectionname = "", item = "", w = 0, j = False):
        self.trace("Inside insert_one")
        event = threading.Event()
        result = {"success": None, "result": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("InsertOne callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"InsertOne failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["result"] = ctypes.cast(response.result, c_char_p).value.decode('utf-8')
                self.lib.free_insert_one_response(response_ptr)
            finally:
                event.set()

        cb = InsertOneCallback(callback)

        req = InsertOneRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                      item=c_char_p(item.encode('utf-8')),
                                      w=w,
                                      j=j)
        
        self.trace("Calling insert_one_async")
        self.lib.insert_one_async(self.client, byref(req), cb)
        self.trace("insert_one_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["result"]

    def insert_many(self, collectionname = "", items = "", w = 0, j = False, skipresults = False):
        self.trace("Inside insert_many")
        event = threading.Event()
        result = {"success": None, "result": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("InsertMany callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"InsertMany failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["result"] = ctypes.cast(response.result, c_char_p).value.decode('utf-8')
                self.lib.free_insert_many_response(response_ptr)
            finally:
                event.set()

        cb = InsertManyCallback(callback)

        req = InsertManyRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                      items=c_char_p(items.encode('utf-8')),
                                      w=w,
                                      j=j,
                                      skipresults=skipresults)
        
        self.trace("Calling insert_many_async")
        self.lib.insert_many_async(self.client, byref(req), cb)
        self.trace("insert_many_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["result"]
    
    def update_one(self, collectionname = "", item = "", w = 0, j = False):
        self.trace("Inside update_one")
        event = threading.Event()
        result = {"success": None, "result": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("UpdateOne")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"UpdateOne failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["result"] = ctypes.cast(response.result, c_char_p).value.decode('utf-8')
                self.lib.free_update_one_response(response_ptr)
            finally:
                event.set()

        cb = UpdateOneCallback(callback)

        req = UpdateOneRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                        item=c_char_p(item.encode('utf-8')),
                                        w=w,
                                        j=j)
        
        self.trace("Calling update_one_async")
        self.lib.update_one_async(self.client, byref(req), cb)
        self.trace("update_one_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["result"]

    def insert_or_update_one(self, collectionname = "", uniqeness = "", item = "", w = 0, j = False):
        self.trace("Inside insert_or_update_one")
        event = threading.Event()
        result = {"success": None, "result": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("InsertOrUpdateOne")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"InsertOrUpdateOne failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["result"] = ctypes.cast(response.result, c_char_p).value.decode('utf-8')
                self.lib.free_insert_or_update_one_response(response_ptr)
            finally:
                event.set()

        cb = InsertOrUpdateOneCallback(callback)

        req = InsertOrUpdateOneRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                              uniqeness=c_char_p(uniqeness.encode('utf-8')),
                                              item=c_char_p(item.encode('utf-8')),
                                              w=w,
                                              j=j)
        
        self.trace("Calling insert_or_update_one_async")
        self.lib.insert_or_update_one_async(self.client, byref(req), cb)
        self.trace("insert_or_update_one_async called")

        event.wait()

        if result["error"]:
            raise result["error"]

        return result["result"]

    def delete_one(self, collectionname = "", id = "", recursive = False):
        self.trace("Inside delete_one")
        event = threading.Event()
        result = {"success": None, "affectedrows": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("DeleteOne callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"DeleteOne failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["affectedrows"] = response.affectedrows
                self.lib.free_delete_one_response(response_ptr)
            finally:
                event.set()

        cb = DeleteOneCallback(callback)

        req = DeleteOneRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                      id=c_char_p(id.encode('utf-8')),
                                      recursive=recursive)
        
        self.trace("Calling delete_one_async")
        self.lib.delete_one_async(self.client, byref(req), cb)
        self.trace("delete_one_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["affectedrows"]

    def delete_many(self, collectionname = "", query = "", recursive = False, ids = []):
        self.trace("Inside delete_many")
        event = threading.Event()
        result = {"success": None, "affectedrows": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("DeleteMany callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"DeleteMany failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["affectedrows"] = response.affectedrows
                self.lib.free_delete_many_response(response_ptr)
            finally:
                event.set()

        cb = DeleteManyCallback(callback)

        cids = (c_char_p * len(ids))()
        cids[:] = [c_char_p(i.encode('utf-8')) for i in ids]

        req = DeleteManyRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                      query=c_char_p(query.encode('utf-8')),
                                      recursive=recursive,
                                      ids=cids)
        
        self.trace("Calling delete_many_async")
        self.lib.delete_many_async(self.client, byref(req), cb)
        self.trace("delete_many_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["affectedrows"]

    def download(self, collectionname = "", id = "", folder = "", filename = ""):
        self.trace("Inside download")
        event = threading.Event()
        result = {"success": None, "filename": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Download callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Download failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["filename"] = ctypes.cast(response.filename, c_char_p).value.decode('utf-8')
                self.lib.free_download_response(response_ptr)
            finally:
                event.set()

        cb = DownloadCallback(callback)


        req = DownloadRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                     id=c_char_p(id.encode('utf-8')),
                                     folder=c_char_p(folder.encode('utf-8')),
                                     filename=c_char_p(filename.encode('utf-8'))
                                     )

        self.trace("Calling download_async")
        self.lib.download_async(self.client, byref(req), cb)
        self.trace("download_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["filename"]
    def upload(self, filepath = "", filename = "", mimetype = "", metadata = "", collectionname = ""):
        self.trace("Inside upload")
        event = threading.Event()
        result = {"success": None, "id": None, "error": None}
        
        self.trace("create callback")
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("Upload callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Upload failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["id"] = ctypes.cast(response.id, c_char_p).value.decode('utf-8')
                self.lib.free_upload_response(response_ptr)
            finally:
                event.set()

        self.trace("create cb")
        cb = UploadCallback(callback)
        
        req = UploadRequestWrapper(filepath=c_char_p(filepath.encode('utf-8')),
                                   filename=c_char_p(filename.encode('utf-8')),
                                   mimetype=c_char_p(mimetype.encode('utf-8')),
                                   metadata=c_char_p(metadata.encode('utf-8')),
                                   collectionname=c_char_p(collectionname.encode('utf-8'))
                                   )
        
        self.trace("Calling upload_async")
        self.lib.upload_async(self.client, byref(req), cb)
        self.trace("upload_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["id"]
    
    def push_workitem(self, wiq = "", wiqid = "", name = "", payload = "", nextrun = 0, success_wiqid = "", failed_wiqid = "", success_wiq = "", failed_wiq = "", priority = 0, files = []):
        self.trace("Inside push_workitem")
        event = threading.Event()
        result = {"success": None, "workitem": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("PushWorkitem callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"PushWorkitem failed: {error_message}")
                else:
                    workitem = decode_workitem(response.workitem)
                    result["success"] = response.success
                    result["workitem"] = workitem

                self.lib.free_push_workitem_response(response_ptr)
            finally:
                event.set()

        cb = PushWorkitemCallback(callback)

        req = PushWorkitemRequestWrapper(wiq=c_char_p(wiq.encode('utf-8')),
                                            wiqid=c_char_p(wiqid.encode('utf-8')),
                                            name=c_char_p(name.encode('utf-8')),
                                            payload=c_char_p(payload.encode('utf-8')),
                                            nextrun=nextrun,
                                            success_wiqid=c_char_p(success_wiqid.encode('utf-8')),
                                            failed_wiqid=c_char_p(failed_wiqid.encode('utf-8')),
                                            success_wiq=c_char_p(success_wiq.encode('utf-8')),
                                            failed_wiq=c_char_p(failed_wiq.encode('utf-8')),
                                            priority=priority,
                                            )
        encode_files(req, files)
        
        self.lib.push_workitem_async.argtypes = [c_void_p, POINTER(PushWorkitemRequestWrapper), PushWorkitemCallback]
        self.trace("Calling push_workitem_async")
        self.lib.push_workitem_async(self.client, byref(req), cb)
        self.trace("push_workitem_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["workitem"]
    def pop_workitem(self, wiq = "", wiqid = "", downloadfolder = "."):
        self.trace("Inside pop_workitem")
        event = threading.Event()
        result = {"success": None, "workitem": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("PopWorkitem callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"PopWorkitem failed: {error_message}")
                else:
                    workitem = decode_workitem(response.workitem)
                    result["success"] = response.success
                    result["workitem"] = workitem
                self.lib.free_pop_workitem_response(response_ptr)
            finally:
                event.set()

        cb = PopWorkitemCallback(callback)

        req = PopWorkitemRequestWrapper(wiq=c_char_p(wiq.encode('utf-8')),
                                        wiqid=c_char_p(wiqid.encode('utf-8')))
        
        self.trace("Calling pop_workitem_async")
        self.lib.pop_workitem_async(self.client, byref(req), downloadfolder, cb)
        self.trace("pop_workitem_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["workitem"]
    
    def update_workitem(self, workitem, files = []):
        self.trace("Inside update_workitem")
        event = threading.Event()
        result = {"success": None, "workitem": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("UpdateWorkitem callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"UpdateWorkitem failed: {error_message}")
                else:
                    workitem = decode_workitem(response.workitem)
                    result["success"] = response.success
                    result["workitem"] = workitem
                self.lib.free_update_workitem_response(response_ptr)
            finally:
                event.set()

        cb = UpdateWorkitemCallback(callback)

        req = UpdateWorkitemRequestWrapper()
        encode_files(req, files)
        workitem_wrapper = encode_workitem(workitem, [])
        workitem_ptr = pointer(workitem_wrapper)

        req.workitem = workitem_ptr
        
        self.trace("Calling update_workitem_async")
        self.lib.update_workitem_async(self.client, byref(req), cb)
        self.trace("update_workitem_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["workitem"]

    def delete_workitem(self, id = ""):
        self.trace("Inside delete_workitem")
        event = threading.Event()
        result = {"success": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                self.trace("DeleteWorkitem callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"DeleteWorkitem failed: {error_message}")
                else:
                    result["success"] = response.success
                self.lib.free_delete_workitem_response(response_ptr)
            finally:
                event.set()

        cb = DeleteWorkitemCallback(callback)

        req = DeleteWorkitemRequestWrapper(id=c_char_p(id.encode('utf-8')))
        
        self.trace("Calling delete_workitem_async")
        self.lib.delete_workitem_async(self.client, byref(req), cb)
        self.trace("delete_workitem_async called")

        event.wait()

        if result["error"]:
            raise result["error"]

    def watch(self, collectionname = "", paths = "", callback = None):
        self.trace("Inside watch")
        event = threading.Event()
        result = {"success": None, "watchid": None, "error": None}
        if not callable(callback):
            raise ValueError("Callback must be a callable function")

        self.counter = 0
        def watch_event_callback(eventref):
            self.trace("Watch Event callback invoked")
            event = eventref.contents
            result = {
                "id": event.id.decode('utf-8'),
                "operation": event.operation.decode('utf-8'),
                "document": event.document.decode('utf-8')
            }
            self.trace("Internal callback invoked", result)
            self.counter += 1
            try:
                callback(result, self.counter)
            except Exception as e:
                self.trace("Error in callback", e)
        def response_callback(response_ptr):
            try:
                self.trace("Watch callback invoked")
                response = response_ptr.contents
                self.trace(response)
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Watch failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["watchid"] = ctypes.cast(response.watchid, c_char_p).value.decode('utf-8')
                self.lib.free_watch_response(response_ptr)
            finally:
                event.set()
        cb = WatchCallback(response_callback, restype=None, argtypes=[POINTER(WatchResponseWrapper)])
        self.callbacks.append(cb)

        c_callback = WatchEventCallback(watch_event_callback)
        self.callbacks.append(c_callback)

        req = WatchRequestWrapper(collectionname=c_char_p(collectionname.encode('utf-8')),
                                  paths=c_char_p(paths.encode('utf-8'))
                                  )
        
        self.trace("Calling watch_async")
        self.lib.watch_async_async(self.client, byref(req), cb, c_callback)
        self.trace("watch_async_async called, now waiting for event")
        
        event.wait()

        self.trace("watch_async_async returned")

        if result["error"]:
            raise result["error"]
        
        return result["watchid"]
    
    def register_queue(self, queuename = "", callback = None):
        self.trace("Inside register_queue")
        result = {"success": None, "queuename": None, "error": None}
        if not callable(callback):
            raise ValueError("Callback must be a callable function")

        self.counter = 0
        def queue_event_callback(eventref):
            self.trace("Queue Event callback invoked")
            event = eventref.contents
            result = {
                "queuename": event.queuename.decode('utf-8'),
                "correlation_id": event.correlation_id.decode('utf-8'),
                "replyto": event.replyto.decode('utf-8'),
                "routingkey": event.routingkey.decode('utf-8'),
                "exchangename": event.exchangename.decode('utf-8'),
                "data": event.data.decode('utf-8')
            }
            self.trace("Internal callback invoked", result)
            self.counter += 1
            try:
                callback(result, self.counter)
            except Exception as e:
                self.trace("Error in callback", e)

        c_callback = QueueEventCallback(queue_event_callback)
        self.callbacks.append(c_callback)

        req = RegisterQueueRequestWrapper(queuename=c_char_p(queuename.encode('utf-8')))
        
        self.trace("Calling register_queue_async")
        #  self.lib.client_count.argtypes = [c_void_p, POINTER(CountRequestWrapper)]
        self.lib.register_queue_async.restype = POINTER(RegisterQueueResponseWrapper)

        reqref = self.lib.register_queue_async(self.client, byref(req), c_callback)
        response = reqref.contents

        if not response.success:
            self.trace("Register Queue failed")
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            self.trace(error_message)
            result["error"] = ClientError(f"Watch failed: {error_message}")
        else:
            self.trace("Register Queue success")
            result["success"] = response.success
            result["queuename"] = ctypes.cast(response.queuename, c_char_p).value.decode('utf-8')

        self.trace("Calling free_register_queue_response")
        self.lib.free_register_queue_response(reqref)

        if result["error"]:
            raise result["error"]
        
        return result["queuename"]

    def register_exchange(self, exchangename = "", algorithm = "", routingkey = "", addqueue = True, callback = None):
        self.trace("Inside register_queue")
        result = {"success": None, "queuename": None, "error": None}
        if not callable(callback):
            raise ValueError("Callback must be a callable function")

        self.counter = 0
        def queue_event_callback(eventref):
            self.trace("Exchange Event callback invoked")
            event = eventref.contents
            result = {
                "queuename": event.queuename.decode('utf-8'),
                "correlation_id": event.correlation_id.decode('utf-8'),
                "replyto": event.replyto.decode('utf-8'),
                "routingkey": event.routingkey.decode('utf-8'),
                "exchangename": event.exchangename.decode('utf-8'),
                "data": event.data.decode('utf-8')
            }
            self.trace("Internal callback invoked", result)
            self.counter += 1
            try:
                callback(result, self.counter)
            except Exception as e:
                self.trace("Error in callback", e)

        c_callback = QueueEventCallback(queue_event_callback)
        self.callbacks.append(c_callback)

        req = RegisterExchangeRequestWrapper(
            exchangename=c_char_p(exchangename.encode('utf-8')),
            algorithm=c_char_p(algorithm.encode('utf-8')),
            routingkey=c_char_p(routingkey.encode('utf-8')),
            addqueue=addqueue)
        
        self.trace("Calling register_exchange_async")
        self.lib.register_exchange_async.argtypes = [c_void_p, POINTER(RegisterExchangeRequestWrapper), QueueEventCallback]
        self.lib.register_exchange_async.restype = POINTER(RegisterExchangeResponseWrapper)

        reqref = self.lib.register_exchange_async(self.client, byref(req), c_callback)
        response = reqref.contents

        if not response.success:
            self.trace("Register Exchange failed")
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            self.trace("error_message is", error_message)
            result["error"] = ClientError(f"Watch failed: {error_message}")
        else:
            self.trace("Register Exchange success")
            result["success"] = response.success
            result["queuename"] = ctypes.cast(response.queuename, c_char_p).value.decode('utf-8')
            self.trace("queuename is", result["queuename"])

        self.trace("Calling free_register_exchange_response")
        self.lib.free_register_exchange_response(reqref)
        self.trace("free_register_exchange_response called")

        if result["error"]:
            raise result["error"]
        
        return result["queuename"]
    
    def unregister_queue(self, queuename):
        self.trace("Inside unregister_queue")
        result = {"success": None, "error": None}
        
        self.lib.unregister_queue.restype = POINTER(UnRegisterQueueResponseWrapper)
        self.trace("Calling unregister_queue")
        ref = self.lib.unregister_queue(self.client, c_char_p(queuename.encode('utf-8')), )
        self.trace("unregister_queue called")
        response = ref.contents
        if not response.success:
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            result["error"] = ClientError(f"Unregister Queue failed: {error_message}")
        else:
            result["success"] = response.success
        self.trace("Calling free_unregister_queue_response")
        self.lib.free_unregister_queue_response(ref)

        if result["error"]:
            raise result["error"]
        
        return result["success"]
    
    def queue_message(self, data, queuename = "", exchangename = "", routingkey = "", replyto = "", correlation_id = "", striptoken = False, expiration = 0):
        self.trace("Inside queue_message")
        result = {"success": None, "error": None}
        if queuename == "" and exchangename == "":
            raise ValueError("Either queuename or exchangename must be provided")
        
        req = QueueMessageRequestWrapper(
            queuename=c_char_p(queuename.encode('utf-8')),
            data=c_char_p(data.encode('utf-8')),
            exchangename=c_char_p(exchangename.encode('utf-8')),
            routingkey=c_char_p(routingkey.encode('utf-8')),
            replyto=c_char_p(replyto.encode('utf-8')),
            correlation_id=c_char_p(correlation_id.encode('utf-8')),
            striptoken=striptoken,
            expiration=expiration
        )
        self.lib.queue_message.restype = POINTER(QueueMessageResponseWrapper)
        self.trace("Calling queue_message")
        ref = self.lib.queue_message(self.client, byref(req))
        self.trace("queue_message called")
        response = ref.contents
        if not response.success:
            error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
            result["error"] = ClientError(f"Queue Message failed: {error_message}")
        else:
            result["success"] = response.success
        self.trace("Calling free_queue_message_response")
        self.lib.free_queue_message_response(ref)

        if result["error"]:
            raise result["error"]
        
        return result["success"]

    def unwatch(self, watchid):
        if not watchid or watchid == "":
            raise ValueError("Watch ID must be provided")
        self.lib.unwatch.argtypes = [c_void_p, c_char_p]
        self.lib.unwatch.restype = POINTER(UnWatchResponseWrapper)
        
        unwatch = self.lib.unwatch(self.client, c_char_p(watchid.encode('utf-8')))
        
        if unwatch:
            unwatchObj = unwatch.contents
            result = {
                'success': unwatchObj.success,
                'error': unwatchObj.error.decode('utf-8') if unwatchObj.error else None
            }
            self.lib.free_unwatch_response(unwatch)
            return result
        else:
            raise ClientError('Unwatch failed or unwatch is null')


    def __del__(self):
        if hasattr(self, 'lib'):
            self.lib.free_client.argtypes = [POINTER(ClientWrapper)]
            self.lib.free_client.restype = None
            if hasattr(self, 'client'):
                self.lib.free_client(self.client)
            print('Client cleaned up')

