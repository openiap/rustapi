import ctypes
import json
import os
import sys
from ctypes import CDLL, Structure, c_char_p, c_void_p, c_bool, c_int, CFUNCTYPE, POINTER, byref
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
        # self.client = self._create_client(url)
        self.callbacks = []  # Keep a reference to the callbacks
    
    def _load_library(self):
        # Determine the path to the shared library
        lib_dir = os.path.join(os.path.dirname(__file__), 'lib')
        lib_path = os.path.join(lib_dir, 'libopeniap.so' if sys.platform != 'win32' else 'libopeniap.dll')
        if sys.platform == 'darwin':
            lib_path = os.path.join(lib_dir, 'libopeniap.dylib')

        if not os.path.exists(lib_path):
            lib_dir = os.path.join(os.path.dirname(__file__), '../../target/debug/')
        lib_path = os.path.join(lib_dir, 'libopeniap.so' if sys.platform != 'win32' else 'libopeniap.dll')
        if sys.platform == 'darwin':
            lib_path = os.path.join(lib_dir, 'libopeniap.dylib')
        
        # Load the Rust library
        try:
            return CDLL(lib_path)
        except OSError as e:
            raise LibraryLoadError(f"Failed to load library: {e}")
    
    def connect(self, url=""):
        # Event to wait for the callback
        event = threading.Event()
        result = {"client": None, "error": None}

        def callback(client_ptr):
            try:
                self.client = client_ptr;
                client = client_ptr.contents
                print("Python: Callback invoked")
                if not client.success:
                    error_message = ctypes.cast(client.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientCreationError(f"Failed to create client: {error_message}")
                else:
                    result["client"] = client_ptr
            finally:
                event.set()

        cb = ConnectCallback(callback)

        print("Python: Calling client_connect")
        self.lib.client_connect(url.encode('utf-8'), cb)
        print("Python: client_connect called")

        # Wait for the callback to be invoked
        event.wait()

        if result["error"]:
            raise result["error"]
        return result["client"]
    
    def signin(self, username="", password=""):
        print("Python: Inside signin")
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}

        def callback(response_ptr):
            try:
                response = response_ptr.contents
                print("Python: Signin callback invoked")
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

        print("Python: Calling client_signin")
        self.lib.client_signin(self.client, byref(req), cb)
        print("Python: client_signin called")

        event.wait()

        if result["error"]:
            raise result["error"]
        return {"success": result["success"], "jwt": result["jwt"]}

    def query(self, collectionname = "", query = "", projection = "", orderby = "", queryas = "", explain = False, skip = 0, top = 0):
        print("Python: Inside query")
        # self.lib.client_query.argtypes = [voidPtr, POINTER(QueryRequestWrapper)]
        # self.lib.client_query.restype = POINTER(QueryResponseWrapper)
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                print("Python: Query callback invoked")
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
        
        print("Python: Calling client_query")
        self.lib.client_query(self.client, byref(req), cb)
        print("Python: client_query called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["results"]
    
    def aggregate(self, collectionname = "", aggregates = "", queryas = "", hint = "", explain = False):
        print("Python: Inside aggregate")
        # self.lib.client_aggregate.argtypes = [voidPtr, POINTER(AggregateRequestWrapper)]
        # self.lib.client_aggregate.restype = POINTER(AggregateResponseWrapper)
        event = threading.Event()
        result = {"success": None, "jwt": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                print("Python: Aggregate callback invoked")
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
        
        print("Python: Calling client_aggregate")
        self.lib.client_aggregate(self.client, byref(req), cb)
        print("Python: client_aggregate called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["results"]
    
    def download(self, collectionname = "", id = "", folder = "", filename = ""):
        print("Python: Inside download")
        # self.lib.client_download.argtypes = [voidPtr, POINTER(DownloadRequestWrapper)]
        # self.lib.client_download.restype = POINTER(DownloadResponseWrapper)
        event = threading.Event()
        result = {"success": None, "filename": None, "error": None}
        
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                print("Python: Download callback invoked")
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

        print("Python: Calling client_download")
        self.lib.client_download(self.client, byref(req), cb)
        print("Python: client_download called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["filename"]
    def upload(self, filepath = "", filename = "", mimetype = "", metadata = "", collectionname = ""):
        print("Python: Inside upload")
        # self.lib.client_upload.argtypes = [voidPtr, POINTER(UploadRequestWrapper)]
        # self.lib.client_upload.restype = POINTER(UploadResponseWrapper)
        event = threading.Event()
        result = {"success": None, "id": None, "error": None}
        
        print("Python: create callback")
        def callback(response_ptr):
            try:
                response = response_ptr.contents
                print("Python: Upload callback invoked")
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Upload failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["id"] = ctypes.cast(response.id, c_char_p).value.decode('utf-8')
                self.lib.free_upload_response(response_ptr)
            finally:
                event.set()

        print("Python: create cb")
        cb = UploadCallback(callback)
        
        req = UploadRequestWrapper(filepath=CString(filepath.encode('utf-8')),
                                   filename=CString(filename.encode('utf-8')),
                                   mimetype=CString(mimetype.encode('utf-8')),
                                   metadata=CString(metadata.encode('utf-8')),
                                   collectionname=CString(collectionname.encode('utf-8'))
                                   )
        
        print("Python: Calling client_upload")
        self.lib.client_upload(self.client, byref(req), cb)
        print("Python: client_upload called")

        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["id"]
    
    def watch(self, collectionname = "", paths = "", callback = None):
        print("Python: Inside watch")
        event = threading.Event()
        result = {"success": None, "watchid": None, "error": None}
        if not callable(callback):
            raise ValueError("Callback must be a callable function")

        def internal_callback(event_str):
            print("Python: Internal callback invoked")
            event_json = event_str.decode('utf-8')
            event_obj = json.loads(event_json)
            callback(event_obj)

        def event_callback(response_ptr):
            try:
                print("Python: Watch callback invoked")
                response = response_ptr.contents
                if not response.success:
                    error_message = ctypes.cast(response.error, c_char_p).value.decode('utf-8')
                    result["error"] = ClientError(f"Watch failed: {error_message}")
                else:
                    result["success"] = response.success
                    result["watchid"] = ctypes.cast(response.watchid, c_char_p).value.decode('utf-8')
                self.lib.free_watch_response(response_ptr)
            finally:
                event.set()
        cb = WatchCallback(event_callback)
        self.callbacks.append(cb)
        
        c_callback = CALLBACK(internal_callback)
        self.callbacks.append(c_callback)  # Keep a reference to the callback to prevent garbage collection

        # self.lib.client_watch.argtypes = [voidPtr, POINTER(WatchRequestWrapper)]
        # self.lib.client_watch.restype = POINTER(WatchResponseWrapper)
        
        req = WatchRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                  paths=CString(paths.encode('utf-8'))
                                  )
        
        print("Python: Calling client_watch")
        watch = self.lib.client_watch(self.client, byref(req), cb, c_callback)
        print("Python: client_watch called")
        
        event.wait()

        if result["error"]:
            raise result["error"]
        
        return result["watchid"]

    def unwatch(self, watchid):
        if not watchid or watchid == "":
            raise ValueError("Watch ID must be provided")
        self.lib.client_unwatch.argtypes = [voidPtr, CString]
        self.lib.client_unwatch.restype = POINTER(UnWatchResponseWrapper)
        
        unwatch = self.lib.client_unwatch(self.client, CString(watchid.encode('utf-8')))
        
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

