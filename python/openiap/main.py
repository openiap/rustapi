import os
import sys
from ctypes import CDLL, Structure, c_char_p, c_void_p, c_bool, c_int, POINTER, byref
import time

# Define the ctypes types for C types
CString = c_char_p
voidPtr = c_void_p
bool = c_bool
c_int = c_int

# Define the ClientWrapper struct
class ClientWrapper(Structure):
    _fields_ = [("success", bool),
                ("error", CString),
                ("client", voidPtr),
                ("runtime", voidPtr)]

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

class DownloadRequestWrapper(Structure):
    _fields_ = [("collectionname", CString),
                ("id", CString),
                ("folder", CString),
                ("filename", CString)]
class DownloadResponseWrapper(Structure):
    _fields_ = [("success", bool),
                ("filename", CString),
                ("error", CString)]

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
    def __init__(self, url = ""):
        self.lib = self._load_library()
        self.client = self._create_client(url)
    
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
    
    def _create_client(self, url):
        self.lib.client_connect.argtypes = [CString]
        self.lib.client_connect.restype = POINTER(ClientWrapper)
        client = self.lib.client_connect(url.encode('utf-8'))
        if not client.contents.success:
            raise ClientCreationError(f"Failed to create client: {client.contents.error.decode('utf-8')}")
        return client
    
    def signin(self, username = "", password = ""):
        self.lib.client_signin.argtypes = [voidPtr, POINTER(SigninRequestWrapper)]
        self.lib.client_signin.restype = POINTER(SigninResponseWrapper)
        
        req = SigninRequestWrapper(username=CString(username.encode('utf-8')),
                                   password=CString(password.encode('utf-8')),
                                   jwt=CString(b''),
                                   agent=CString(b''),
                                   version=CString(b''),
                                   longtoken=False,
                                   validateonly=False,
                                   ping=False)
        
        user = self.lib.client_signin(self.client, byref(req))
        
        if user:
            userObj = user.contents
            result = {
                'success': userObj.success,
                'jwt': userObj.jwt.decode('utf-8') if userObj.jwt else None,
                'error': userObj.error.decode('utf-8') if userObj.error else None
            }
            self.lib.free_signin_response(user)
            return result
        else:
            raise ClientError('Signin failed or user is null')

    def query(self, collectionname = "", query = "", projection = "", orderby = "", queryas = "", explain = False, skip = 0, top = 0):
        self.lib.client_query.argtypes = [voidPtr, POINTER(QueryRequestWrapper)]
        self.lib.client_query.restype = POINTER(QueryResponseWrapper)
        
        req = QueryRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                  query=CString(query.encode('utf-8')),
                                  projection=CString(projection.encode('utf-8')),
                                  orderby=CString(orderby.encode('utf-8')),
                                  queryas=CString(queryas.encode('utf-8')),
                                  explain=explain,
                                  skip=skip,
                                  top=top)
        
        query = self.lib.client_query(self.client, byref(req))
        
        if query:
            queryObj = query.contents
            result = {
                'success': queryObj.success,
                'results': queryObj.results.decode('utf-8') if queryObj.results else None,
                'error': queryObj.error.decode('utf-8') if queryObj.error else None
            }
            self.lib.free_query_response(query)
            return result
        else:
            raise ClientError('Query failed or query is null')
    
    def download(self, collectionname = "", id = "", folder = "", filename = ""):
        self.lib.client_download.argtypes = [voidPtr, POINTER(DownloadRequestWrapper)]
        self.lib.client_download.restype = POINTER(DownloadResponseWrapper)
        
        req = DownloadRequestWrapper(collectionname=CString(collectionname.encode('utf-8')),
                                     id=CString(id.encode('utf-8')),
                                     folder=CString(folder.encode('utf-8')),
                                     filename=CString(filename.encode('utf-8'))
                                     )
        
        download = self.lib.client_download(self.client, byref(req))
        
        if download:
            downloadObj = download.contents
            result = {
                'success': downloadObj.success,
                'filename': downloadObj.filename.decode('utf-8') if downloadObj.filename else None,
                'error': downloadObj.error.decode('utf-8') if downloadObj.error else None
            }
            self.lib.free_download_response(download)
            return result
        else:
            raise ClientError('Download failed or download is null')

    def upload(self, filepath = "", filename = "", mimetype = "", metadata = "", collectionname = ""):
        self.lib.client_upload.argtypes = [voidPtr, POINTER(UploadRequestWrapper)]
        self.lib.client_upload.restype = POINTER(UploadResponseWrapper)
        
        req = UploadRequestWrapper(filepath=CString(filepath.encode('utf-8')),
                                   filename=CString(filename.encode('utf-8')),
                                   mimetype=CString(mimetype.encode('utf-8')),
                                   metadata=CString(metadata.encode('utf-8')),
                                   collectionname=CString(collectionname.encode('utf-8'))
                                   )
        
        upload = self.lib.client_upload(self.client, byref(req))
        
        if upload:
            uploadObj = upload.contents
            result = {
                'success': uploadObj.success,
                'id': uploadObj.id.decode('utf-8') if uploadObj.id else None,
                'error': uploadObj.error.decode('utf-8') if uploadObj.error else None
            }
            self.lib.free_upload_response(upload)
            return result
        else:
            raise ClientError('Upload failed or upload is null')

    def __del__(self):
        if hasattr(self, 'lib'):
            self.lib.free_client.argtypes = [POINTER(ClientWrapper)]
            self.lib.free_client.restype = None
            if hasattr(self, 'client'):
                self.lib.free_client(self.client)
            print('Client cleaned up')

