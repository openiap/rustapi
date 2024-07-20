"use strict";

function _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } }

function _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); return Constructor; }

function _typeof(obj) { if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj; }; } return _typeof(obj); }

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _possibleConstructorReturn(self, call) { if (call && (_typeof(call) === "object" || typeof call === "function")) { return call; } return _assertThisInitialized(self); }

function _assertThisInitialized(self) { if (self === void 0) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return self; }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function"); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, writable: true, configurable: true } }); if (superClass) _setPrototypeOf(subClass, superClass); }

function _wrapNativeSuper(Class) { var _cache = typeof Map === "function" ? new Map() : undefined; _wrapNativeSuper = function _wrapNativeSuper(Class) { if (Class === null || !_isNativeFunction(Class)) return Class; if (typeof Class !== "function") { throw new TypeError("Super expression must either be null or a function"); } if (typeof _cache !== "undefined") { if (_cache.has(Class)) return _cache.get(Class); _cache.set(Class, Wrapper); } function Wrapper() { return _construct(Class, arguments, _getPrototypeOf(this).constructor); } Wrapper.prototype = Object.create(Class.prototype, { constructor: { value: Wrapper, enumerable: false, writable: true, configurable: true } }); return _setPrototypeOf(Wrapper, Class); }; return _wrapNativeSuper(Class); }

function isNativeReflectConstruct() { if (typeof Reflect === "undefined" || !Reflect.construct) return false; if (Reflect.construct.sham) return false; if (typeof Proxy === "function") return true; try { Date.prototype.toString.call(Reflect.construct(Date, [], function () {})); return true; } catch (e) { return false; } }

function _construct(Parent, args, Class) { if (isNativeReflectConstruct()) { _construct = Reflect.construct; } else { _construct = function _construct(Parent, args, Class) { var a = [null]; a.push.apply(a, args); var Constructor = Function.bind.apply(Parent, a); var instance = new Constructor(); if (Class) _setPrototypeOf(instance, Class.prototype); return instance; }; } return _construct.apply(null, arguments); }

function _isNativeFunction(fn) { return Function.toString.call(fn).indexOf("[native code]") !== -1; }

function _setPrototypeOf(o, p) { _setPrototypeOf = Object.setPrototypeOf || function _setPrototypeOf(o, p) { o.__proto__ = p; return o; }; return _setPrototypeOf(o, p); }

function _getPrototypeOf(o) { _getPrototypeOf = Object.setPrototypeOf ? Object.getPrototypeOf : function _getPrototypeOf(o) { return o.__proto__ || Object.getPrototypeOf(o); }; return _getPrototypeOf(o); }

var StructType = require('ref-struct-napi');

var ffi = require('ffi-napi');

var ref = require('ref-napi');

var path = require('path');

var CString = ref.types.CString;
var voidPtr = ref.refType(ref.types["void"]);
var bool = ref.types.bool;
var _int = ref.types["int"]; // Define the ClientWrapper struct

var ClientWrapper = StructType({
  success: bool,
  error: CString,
  client: voidPtr,
  runtime: voidPtr
});
var ClientWrapperPtr = ref.refType(ClientWrapper); // Define the SigninRequestWrapper struct

var SigninRequestWrapper = StructType({
  username: CString,
  password: CString,
  jwt: CString,
  agent: CString,
  version: CString,
  longtoken: bool,
  validateonly: bool,
  ping: bool
});
var SigninRequestWrapperPtr = ref.refType(SigninRequestWrapper); // Define the SigninResponseWrapper struct

var SigninResponseWrapper = StructType({
  success: bool,
  jwt: CString,
  error: CString
});
var SigninResponseWrapperPtr = ref.refType(SigninResponseWrapper); // Define the SigninRequestWrapper struct

var QueryRequestWrapper = StructType({
  collectionname: CString,
  query: CString,
  projection: CString,
  orderby: CString,
  queryas: CString,
  explain: bool,
  skip: _int,
  top: _int
});
var QueryRequestWrapperPtr = ref.refType(QueryRequestWrapper); // Define the SigninResponseWrapper struct

var QueryResponseWrapper = StructType({
  success: bool,
  results: CString,
  error: CString
});
var QueryResponseWrapperPtr = ref.refType(QueryResponseWrapper); // Function to load the correct library file based on the operating system

function loadLibrary() {
  var libDir = path.join(__dirname, 'lib');
  var libPath;

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
      'free_signin_response': ['void', [SigninResponseWrapperPtr]],
      'client_query': [QueryRequestWrapperPtr, [voidPtr, QueryResponseWrapperPtr]],
      'free_query_response': ['void', [QueryResponseWrapperPtr]]
    });
  } catch (e) {
    throw new LibraryLoadError("Failed to load library: ".concat(e.message));
  }
} // Custom error classes


var ClientError =
/*#__PURE__*/
function (_Error) {
  _inherits(ClientError, _Error);

  function ClientError(message) {
    var _this;

    _classCallCheck(this, ClientError);

    _this = _possibleConstructorReturn(this, _getPrototypeOf(ClientError).call(this, message));
    _this.name = "ClientError";
    return _this;
  }

  return ClientError;
}(_wrapNativeSuper(Error));

var LibraryLoadError =
/*#__PURE__*/
function (_ClientError) {
  _inherits(LibraryLoadError, _ClientError);

  function LibraryLoadError(message) {
    var _this2;

    _classCallCheck(this, LibraryLoadError);

    _this2 = _possibleConstructorReturn(this, _getPrototypeOf(LibraryLoadError).call(this, message));
    _this2.name = "LibraryLoadError";
    return _this2;
  }

  return LibraryLoadError;
}(ClientError);

var ClientCreationError =
/*#__PURE__*/
function (_ClientError2) {
  _inherits(ClientCreationError, _ClientError2);

  function ClientCreationError(message) {
    var _this3;

    _classCallCheck(this, ClientCreationError);

    _this3 = _possibleConstructorReturn(this, _getPrototypeOf(ClientCreationError).call(this, message));
    _this3.name = "ClientCreationError";
    return _this3;
  }

  return ClientCreationError;
}(ClientError); // Client class


var Client =
/*#__PURE__*/
function () {
  function Client(url) {
    _classCallCheck(this, Client);

    this.lib = loadLibrary();
    this.client = this.createClient(url);
  }

  _createClass(Client, [{
    key: "createClient",
    value: function createClient(url) {
      var client = this.lib.client_connect(url);
      var clientres = client.deref();

      if (!clientres.success) {
        throw new ClientCreationError(clientres.error);
      }

      return client;
    }
  }, {
    key: "signin",
    value: function signin(username, password) {
      var jwt = "";
      if (username == null) username = '';
      if (password == null) password = '';

      if (username != "" && password == "") {
        jwt = username;
        username = "";
      } // Allocate C strings for the SigninRequestWrapper fields


      var req = new SigninRequestWrapper({
        username: ref.allocCString(username),
        password: ref.allocCString(password),
        jwt: ref.allocCString(jwt),
        agent: ref.allocCString('node'),
        version: ref.allocCString(''),
        longtoken: false,
        validateonly: false,
        ping: false
      }); // Call the client_signin function

      var user = this.lib.client_signin(this.client, req.ref());

      if (ref.isNull(user)) {
        throw new ClientError('Signin failed or user is null');
      }

      var userObj = user.deref();
      var result = {
        success: userObj.success,
        jwt: userObj.jwt,
        error: userObj.error
      };
      this.lib.free_signin_response(user);
      return result;
    }
  }, {
    key: "query",
    value: function query(_ref) {
      var collectionname = _ref.collectionname,
          _query = _ref.query,
          projection = _ref.projection,
          orderby = _ref.orderby,
          queryas = _ref.queryas,
          explain = _ref.explain,
          skip = _ref.skip,
          top = _ref.top;
      // Allocate C strings for the QueryRequestWrapper fields
      var req = new QueryRequestWrapper({
        collectionname: ref.allocCString(collectionname),
        query: ref.allocCString(_query),
        projection: ref.allocCString(projection),
        orderby: ref.allocCString(orderby),
        queryas: ref.allocCString(queryas),
        explain: explain,
        skip: skip,
        top: top
      });
      var response = this.lib.client_query(this.client, req.ref());

      if (ref.isNull(response)) {
        throw new ClientError('Signin failed or user is null');
      }

      var Obj = response.deref();
      console.log("Obj", Obj);
      var result = {
        success: Obj.success,
        results: JSON.parse(Obj.results),
        error: Obj.error
      };
      this.lib.free_query_response(Obj);
      return result;
    }
  }, {
    key: "free",
    value: function free() {
      if (this.client) {
        this.lib.free_client(this.client);
      }
    }
  }]);

  return Client;
}();

module.exports = {
  Client: Client,
  ClientError: ClientError,
  LibraryLoadError: LibraryLoadError,
  ClientCreationError: ClientCreationError
};