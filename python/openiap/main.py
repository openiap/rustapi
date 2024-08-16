import ctypes
import json
import os
import sys
from ctypes import CDLL, Structure, c_char_p, c_void_p, c_bool, c_int, c_size_t, CFUNCTYPE, POINTER, byref
import threading
import time

# Define the ctypes types for C types
CString = c_char_p
voidPtr = c_void_p
bool = c_bool
c_int = c_int
CALLBACK = CFUNCTYPE(None, c_char_p)

# Define the ClientWrapper struct
class ClientWrapper(Structure):
    _fields_ = [("success", bool),
                ("error", CString),
                ("client", voidPtr),
                ("runtime", voidPtr)]

ConnectCallback = CFUNCTYPE(None, POINTER(ClientWrapper))

# Define the SigninRequestWrapper struct
class SigninRequestWrapper(Structure):
    _fields_ = [("username", CString),
                ("password", CString),
                ("jwt", CString),
                ("agent", CString),
                ("version", CString),
                ("longtoken", bool),
                ("validateonly", bool),
                ("ping", bool)]

# Define the SigninResponseWrapper struct
class SigninResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("jwt", CString),
                ("error", CString)]
SigninCallback = CFUNCTYPE(None, POINTER(SigninResponseWrapper))
    
class QueryRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("query", CString),
                ("projection", CString),
                ("orderby", CString),
                ("queryas", CString),
                ("explain", bool),
                ("skip", c_int),
                ("top", c_int)]
class QueryResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("results", CString),
                ("error", CString)]
QueryCallback = CFUNCTYPE(None, POINTER(QueryResponseWrapper))

class AggregateRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("aggregates", CString),
                ("queryas", CString),
                ("hint", CString),
                ("explain", bool)]
class AggregateResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("results", CString),
                ("error", CString)]
AggregateCallback = CFUNCTYPE(None, POINTER(AggregateResponseWrapper))

class CountRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("query", CString),
                ("queryas", CString),
                ("explain", bool)]
class CountResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("result", c_int),
                ("error", CString)]
CountCallback = CFUNCTYPE(None, POINTER(CountResponseWrapper))

class DistinctRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("field", CString),
                ("query", CString),
                ("queryas", CString),
                ("explain", bool)]
class DistinctResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ('results', POINTER(c_char_p)),
                ('results_count', c_size_t),
                ("error", CString)]
DistinctCallback = CFUNCTYPE(None, POINTER(DistinctResponseWrapper))

class InsertOneRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("item", CString),
                ("w", c_int),
                ("j", bool)]
class InsertOneResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("result", CString),
                ("error", CString)]
InsertOneCallback = CFUNCTYPE(None, POINTER(InsertOneResponseWrapper))

class InsertManyRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("items", CString),
                ("w", c_int),
                ("j", bool),
                ("skipresults", bool)]
class InsertManyResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("result", CString),
                ("error", CString)]
InsertManyCallback = CFUNCTYPE(None, POINTER(InsertManyResponseWrapper))

class UpdateOneRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("item", CString),
                ("w", c_int),
                ("j", bool)]
class UpdateOneResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("result", CString),
                ("error", CString)]
UpdateOneCallback = CFUNCTYPE(None, POINTER(UpdateOneResponseWrapper))
    
class InsertOrUpdateOneRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("uniqeness", CString),
                ("item", CString),
                ("w", c_int),
                ("j", bool)]
class InsertOrUpdateOneResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("result", CString),
                ("error", CString)]
InsertOrUpdateOneCallback = CFUNCTYPE(None, POINTER(InsertOrUpdateOneResponseWrapper))
    
class DownloadRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("id", CString),
                ("folder", CString),
                ("filename", CString)]
class DownloadResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("filename", CString),
                ("error", CString)]
DownloadCallback = CFUNCTYPE(None, POINTER(DownloadResponseWrapper))

class UploadRequestWrapper(Structure):
    _fields_ = [("filepath", CString),
                ("filename", CString),
                ("mimetype", CString),
                ("metadata", CString),
                ("collectionname", CString)]
class UploadResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("id", CString),
                ("error", CString)]
UploadCallback = CFUNCTYPE(None, POINTER(UploadResponseWrapper))

class WorkitemFileWrapper(Structure):
    _fields_ = [("filename", CString),
                ("id", CString),
                ("compressed", bool),
                #("file", CString)
                ]
class WorkitemWrapper(Structure):
    _fields_ = [("id", CString),
                ("name", CString),
                ("payload", CString),
                ("priority", c_int),
                ("nextrun", c_int),
                ("lastrun", c_int),
                ("files", ctypes.POINTER(WorkitemFileWrapper)),
                ("files_len", c_size_t),
                ("state", CString),
                ("wiq", CString),
                ("wiqid", CString),
                ("retries", c_int),
                ("username", CString),
                ("success_wiqid", CString),
                ("failed_wiqid", CString),
                ("success_wiq", CString),
                ("failed_wiq", CString),
                ("errormessage", CString),
                ("errorsource", CString),
                ("errortype", CString),
                ]
class PushWorkitemRequestWrapper(Structure):
    _fields_ = [("wiq", CString),
                ("wiqid", CString),
                ("name",  CString),
                ("payload",  CString),
                ("nextrun", c_int),
                ("success_wiqid",  CString),
                ("failed_wiqid",  CString),
                ("success_wiq",  CString),
                ("failed_wiq",  CString),
                ("priority", c_int),
                ("files", POINTER(WorkitemFileWrapper)),
                ("files_len", c_size_t)
    ]
class PushWorkitemResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("error", CString),
                # ("workitem", WorkitemWrapper),
                ("workitem", POINTER(WorkitemWrapper)),
                # ("workitem", ctypes.POINTER(WorkitemWrapper)),
                ]
PushWorkitemCallback = CFUNCTYPE(None, POINTER(PushWorkitemResponseWrapper))

class PopWorkitemRequestWrapper(Structure):
    _fields_ = [("wiq", CString),
                ("wiqid", CString)]
class PopWorkitemResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("error", CString),
                ("workitem", POINTER(WorkitemWrapper))]
PopWorkitemCallback = CFUNCTYPE(None, POINTER(PopWorkitemResponseWrapper))

class DeleteOneRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("id", CString),
                ("recursive", bool)]
class DeleteOneResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("affectedrows", c_int),
                ("error", CString)]
DeleteOneCallback = CFUNCTYPE(None, POINTER(DeleteOneResponseWrapper))

class DeleteManyRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("query", CString),
                ("recursive", bool),
                # ids is a list of strings
                ("ids", POINTER(CString))]
class DeleteManyResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("affectedrows", c_int),
                ("error", CString)]
DeleteManyCallback = CFUNCTYPE(None, POINTER(DeleteManyResponseWrapper))

class WatchEventWrapper(Structure):
    _fields_ = [("id", CString),
                ("operation", CString),
                ("document", CString)]
WatchEventCallback = CFUNCTYPE(None, POINTER(WatchEventWrapper))
class WatchRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("paths", CString)]
class WatchResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("watchid", CString),
                ("error", CString)]
WatchCallback = CFUNCTYPE(None, POINTER(WatchResponseWrapper))

class UnWatchResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("error", CString)]

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

class Client:
    def __init__(self):
        self.lib = self._load_library()
        self.tracing = False
        self.informing = False
        self.verbosing = False
        # self.client = self._create_client(url)
        self.callbacks = []  # Keep a reference to the callbacks
    
    def _load_library(self):
        # Determine the path to the shared library
        lib_dir = os.path.join(os.path.dirname(__file__), 'lib')
        architecture = os.uname().machine
        if sys.platform == 'win32':
            if architecture == 'x86_64':
                lib_file = 'openiap-windows-x64.dll'
            elif architecture == 'AMD64':
                lib_file = 'openiap-windows-x86.dll'
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
                self.client = client_ptr;
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
        self.lib.connect_async(url.encode('utf-8'), cb)
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

        req = QueryRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                  query=CString(query.encode('utf-8')),
                                  projection=CString(projection.encode('utf-8')),
                                  orderby=CString(orderby.encode('utf-8')),
                                  queryas=CString(queryas.encode('utf-8')),
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

        req = AggregateRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                      aggregates=CString(aggregates.encode('utf-8')),
                                      queryas=CString(queryas.encode('utf-8')),
                                      hint=CString(hint.encode('utf-8')),
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

        req = CountRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                  query=CString(query.encode('utf-8')),
                                  queryas=CString(queryas.encode('utf-8')),
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
                    results_count = response.results_count
                    results = []

                    for i in range(results_count):
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

        req = InsertOneRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                      item=CString(item.encode('utf-8')),
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

        req = InsertManyRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                      items=CString(items.encode('utf-8')),
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

        req = UpdateOneRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                        item=CString(item.encode('utf-8')),
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

        req = InsertOrUpdateOneRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                              uniqeness=CString(uniqeness.encode('utf-8')),
                                              item=CString(item.encode('utf-8')),
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

        req = DeleteOneRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                      id=CString(id.encode('utf-8')),
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

        cids = (CString * len(ids))()
        cids[:] = [CString(i.encode('utf-8')) for i in ids]

        req = DeleteManyRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                      query=CString(query.encode('utf-8')),
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


        req = DownloadRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                     id=CString(id.encode('utf-8')),
                                     folder=CString(folder.encode('utf-8')),
                                     filename=CString(filename.encode('utf-8'))
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
        
        req = UploadRequestWrapper(filepath=CString(filepath.encode('utf-8')),
                                   filename=CString(filename.encode('utf-8')),
                                   mimetype=CString(mimetype.encode('utf-8')),
                                   metadata=CString(metadata.encode('utf-8')),
                                   collectionname=CString(collectionname.encode('utf-8'))
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
                print("response.success", response.success)
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"PushWorkitem failed: {error_message}")
                else:
                    # cb = PushWorkitemCallback(callback)
                    print("*****************")
                    print("workitem_ptr", response.workitem)
                    workitem = response.workitem.contents

                    # print("workitem", workitem)
                    # result["success"] = response.success
                    result["workitem"] = {
                        "id": workitem.id.decode('utf-8'),
                        "name": workitem.name.decode('utf-8'),
                        "payload": workitem.payload.decode('utf-8'),
                        "priority": workitem.priority,
                        "nextrun": workitem.nextrun,
                        "lastrun": workitem.lastrun,
                        "files": [],
                        "files_len": workitem.files_len,
                        #"state": workitem.state.decode('utf-8'),
                        "wiq": workitem.wiq.decode('utf-8'),
                        "wiqid": workitem.wiqid.decode('utf-8'),
                        "retries": workitem.retries,
                        "username": workitem.username.decode('utf-8'),
                        "success_wiqid": workitem.success_wiqid.decode('utf-8'),
                        "failed_wiqid": workitem.failed_wiqid.decode('utf-8'),
                        "success_wiq": workitem.success_wiq.decode('utf-8'),
                        "failed_wiq": workitem.failed_wiq.decode('utf-8'),
                        "errormessage": workitem.errormessage.decode('utf-8'),
                        "errorsource": workitem.errorsource.decode('utf-8'),
                        "errortype": workitem.errortype.decode('utf-8')
                    }
                    print("*****************")
                    print("workitem", result["workitem"])
                    print("*****************")
                    for i in range(workitem.files_len):
                        file = workitem.files[i]
                        result["workitem"]["files"].append({
                            "filename": file.filename.decode('utf-8'),
                            "id": file.id.decode('utf-8'),
                            "compressed": file.compressed
                        })
                self.lib.free_push_workitem_response(response_ptr)
            finally:
                event.set()

        cb = PushWorkitemCallback(callback)

        cfiles = (WorkitemFileWrapper * len(files))()
        cfiles[:] = [WorkitemFileWrapper(filename=CString(f["filename"].encode('utf-8')),
                                            id=CString(f["id"].encode('utf-8')),
                                            compressed=f["compressed"]) for f in files]
        
        req = PushWorkitemRequestWrapper(wiq=CString(wiq.encode('utf-8')),
                                            wiqid=CString(wiqid.encode('utf-8')),
                                            name=CString(name.encode('utf-8')),
                                            payload=CString(payload.encode('utf-8')),
                                            nextrun=nextrun,
                                            success_wiqid=CString(success_wiqid.encode('utf-8')),
                                            failed_wiqid=CString(failed_wiqid.encode('utf-8')),
                                            success_wiq=CString(success_wiq.encode('utf-8')),
                                            failed_wiq=CString(failed_wiq.encode('utf-8')),
                                            priority=priority,
                                            files=cfiles,
                                            files_len=len(files))
        
        self.trace("Calling push_workitem_async")
        self.lib.push_workitem_async(self.client, byref(req), cb)
        self.trace("push_workitem_async called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["workitem"]
        

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
            callback(result, self.counter)

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

        req = WatchRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                  paths=CString(paths.encode('utf-8'))
                                  )
        
        self.trace("Calling watch_async")
        self.lib.watch_async_async(self.client, byref(req), cb, c_callback)
        self.trace("watch_async_async called, now waiting for event")
        
        event.wait()

        self.trace("watch_async_async returned")

        if result["error"]:
            raise result["error"]
        
        return result["watchid"]

    def unwatch(self, watchid):
        if not watchid or watchid == "":
            raise ValueError("Watch ID must be provided")
        self.lib.unwatch.argtypes = [voidPtr, CString]
        self.lib.unwatch.restype = POINTER(UnWatchResponseWrapper)
        
        unwatch = self.lib.unwatch(self.client, CString(watchid.encode('utf-8')))
        
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

