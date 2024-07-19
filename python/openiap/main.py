import os
import sys
from ctypes import CDLL, Structure, c_char_p, c_void_p, c_bool, POINTER, byref
import time

# Define the ctypes types for C types
CString = c_char_p
voidPtr = c_void_p
bool = c_bool

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
    def __init__(self, url):
        self.lib = self._load_library()
        self.client = self._create_client(url)
    
    def _load_library(self):
        # Determine the path to the shared library
        lib_dir = os.path.join(os.path.dirname(__file__), 'lib')
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
    
    def __del__(self):
        self.lib.free_client.argtypes = [POINTER(ClientWrapper)]
        self.lib.free_client.restype = None
        self.lib.free_client(self.client)
        print('Client cleaned up')

