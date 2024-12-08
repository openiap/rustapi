<?php
namespace openiap;

use \Exception;
use \FFI;

if (!extension_loaded('FFI')) {
    throw new Exception("FFI extension is not loaded");
}

class Client {
    private $client;
    private $ffi;

    private function loadLibrary() {
        $platform = PHP_OS_FAMILY;
        $arch = php_uname('m');
        $libDir = __DIR__ . '/lib';
        $libPath = null;

        print_r("Platform: $platform, Arch: $arch\n");

        switch (strtolower($platform)) {
            case 'windows':
                switch ($arch) {
                    case 'x86_64':
                        $libPath = $libDir . '/openiap-windows-x64.dll';
                        break;
                    case 'x86':
                        $libPath = $libDir . '/openiap-windows-i686.dll';
                        break;
                    case 'aarch64':
                        $libPath = $libDir . '/openiap-windows-arm64.dll';
                        break;
                    default:
                        throw new Exception("Unsupported architecture on Windows: $arch");
                }
                break;
            case 'darwin':
                switch ($arch) {
                    case 'x86_64':
                        $libPath = $libDir . '/libopeniap-macos-x64.dylib';
                        break;
                    case 'arm64':
                        $libPath = $libDir . '/libopeniap-macos-arm64.dylib';
                        break;
                    default:
                        throw new Exception("Unsupported architecture on Darwin: $arch");
                }
                break;
            case 'linux':
                switch ($arch) {
                    case 'x86_64':
                        // Note: PHP doesn't have a direct way to detect musl vs glibc
                        // You might want to add additional detection logic if needed
                        $libPath = $libDir . '/libopeniap-linux-x64.so';
                        break;
                    case 'aarch64':
                        $libPath = $libDir . '/libopeniap-linux-arm64.so';
                        break;
                    default:
                        throw new Exception("Unsupported architecture on Linux: $arch");
                }
                break;
            default:
                throw new Exception("Unsupported platform: $platform");
        }

        if (!file_exists($libPath)) {
            $libDir = __DIR__ . '/../../target/debug/';
            switch (strtolower($platform)) {
                case 'windows':
                    $libPath = $libDir . 'openiap_clib.dll';
                    break;
                case 'darwin':
                    $libPath = $libDir . 'libopeniap_clib.dylib';
                    break;
                default:
                    $libPath = $libDir . 'libopeniap_clib.so';
                    break;
            }
        }

        print_r("Using library: $libPath\n");
        $this->ffi = FFI::cdef(
            file_get_contents(__DIR__ . "/clib_openiap.h"),
            $libPath
        );
    }
    private function createClient() {
        print_r("creating client\n");
        $client = $this->ffi->create_client();
        if ($client === null) {
            throw new Exception("Failed to create client");
        }
        return $client;
    }
    
    public function __construct() {
        $this->loadLibrary();
        $this->client = $this->createClient();
    }

    public function __destruct() {
        $this->ffi->free_client($this->client);
    }
    public function  enable_tracing($rust_log, $tracing) {
        print_r("enabling tracing with rust_log=$rust_log, tracing=$tracing\n");
        $this->ffi->enable_tracing($rust_log, $tracing);
    }
    public function  connect($url) {
        print_r("connecting to $url\n");
        $response = $this->ffi->client_connect($this->client, $url);
        if ($response->success) {
            echo "Connected successfully!\n";
        } else {
            echo "Error: " . FFI::string($response->error) . "\n";
        }
        $this->ffi->free_connect_response($response);
    }
    public function  listCollections() {
        $response = $this->ffi->list_collections($this->client, false);
        $collections = [];
        if ($response->success) {
            $collections = json_decode(FFI::string($response->results), true);
        } else {
            echo "Error: " . FFI::string($response->error) . "\n";
        }
        $this->ffi->free_list_collections_response($response);
        return $collections;
    }

    public function createCollection($collectionname, $options = array()) {
        $request = $this->ffi->new('struct CreateCollectionRequestWrapper');
        
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));
        $request->request_id = 0;

        // Set optional parameters if provided
        if (isset($options['expire_after_seconds'])) {
            $request->expire_after_seconds = $options['expire_after_seconds'];
        }
        if (isset($options['change_stream_pre_and_post_images'])) {
            $request->change_stream_pre_and_post_images = $options['change_stream_pre_and_post_images'];
        }
        if (isset($options['capped'])) {
            $request->capped = $options['capped'];
        }
        if (isset($options['max'])) {
            $request->max = $options['max'];
        }
        if (isset($options['size'])) {
            $request->size = $options['size'];
        }

        $response = $this->ffi->create_collection($this->client, FFI::addr($request));
        
        // Free allocated memory
        FFI::free($str);
        
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_create_collection_response($response);
            throw new Exception($error_message);
        }
        
        $this->ffi->free_create_collection_response($response);
    }

    public function dropCollection($collectionname) {
        $response = $this->ffi->drop_collection($this->client, $collectionname);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_drop_collection_response($response);
            throw new Exception($error_message);
        }
        $this->ffi->free_drop_collection_response($response);
    }

    public function insertOne($collectionname, $item) {
        $request = $this->ffi->new('struct InsertOneRequestWrapper');

        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Convert item to JSON string
        $json_item = json_encode($item);
        $item_str = $this->ffi->new("char[" . strlen($json_item) + 1 . "]", false);
        FFI::memcpy($item_str, $json_item, strlen($json_item));
        $request->item = FFI::cast("char *", FFI::addr($item_str));

        // Set default values
        $request->w = 0;
        $request->j = false;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->insert_one($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($item_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_insert_one_response($response);
            throw new Exception($error_message);
        }

        $result = null;
        if ($response->result) {
            $result = json_decode(FFI::string($response->result), true);
        }

        $this->ffi->free_insert_one_response($response);
        return $result;
    }

    public function insertOrUpdateOne($collectionname, $item, $uniqeness) {
        $request = $this->ffi->new('struct InsertOrUpdateOneRequestWrapper');

        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Convert item to JSON string
        $json_item = json_encode($item);
        $item_str = $this->ffi->new("char[" . strlen($json_item) + 1 . "]", false);
        FFI::memcpy($item_str, $json_item, strlen($json_item));
        $request->item = FFI::cast("char *", FFI::addr($item_str));

        // Set uniqeness
        $uniq_str = $this->ffi->new("char[" . strlen($uniqeness) + 1 . "]", false);
        FFI::memcpy($uniq_str, $uniqeness, strlen($uniqeness));
        $request->uniqeness = FFI::cast("char *", FFI::addr($uniq_str));

        // Set default values
        $request->w = 0;
        $request->j = false;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->insert_or_update_one($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($item_str);
        FFI::free($uniq_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_insert_or_update_one_response($response);
            throw new Exception($error_message);
        }

        $result = null;
        if ($response->result) {
            $result = json_decode(FFI::string($response->result), true);
        }

        $this->ffi->free_insert_or_update_one_response($response);
        return $result;
    }

    public function insertMany($collectionname, $items, $options = array()) {
        $request = $this->ffi->new('struct InsertManyRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Convert items array to JSON string
        $json_items = json_encode($items);
        $items_str = $this->ffi->new("char[" . strlen($json_items) + 1 . "]", false);
        FFI::memcpy($items_str, $json_items, strlen($json_items));
        $request->items = FFI::cast("char *", FFI::addr($items_str));

        // Set default values and options
        $request->w = isset($options['w']) ? $options['w'] : 0;
        $request->j = isset($options['j']) ? $options['j'] : false;
        $request->skipresults = isset($options['skipresults']) ? $options['skipresults'] : false;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->insert_many($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($items_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_insert_many_response($response);
            throw new Exception($error_message);
        }

        $result = null;
        if ($response->results) {
            $result = json_decode(FFI::string($response->results), true);
        }

        $this->ffi->free_insert_many_response($response);
        return $result;
    }

    public function deleteOne($collectionname, $id, $recursive = false) {
        $request = $this->ffi->new('struct DeleteOneRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Set id
        $id_str = $this->ffi->new("char[" . strlen($id) + 1 . "]", false);
        FFI::memcpy($id_str, $id, strlen($id));
        $request->id = FFI::cast("char *", FFI::addr($id_str));

        $request->recursive = $recursive;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->delete_one($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($id_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_delete_one_response($response);
            throw new Exception($error_message);
        }

        $affected_rows = $response->affectedrows;
        $this->ffi->free_delete_one_response($response);
        return $affected_rows;
    }

    public function deleteMany($collectionname, $query, $recursive = false) {
        $request = $this->ffi->new('struct DeleteManyRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Initialize ids as a pointer to an array with a single null element
        // Create a persistent CData array that won't be freed
        $ids_array = FFI::new("char*[1]", false);
        $ids_array[0] = null;
        $request->ids = $ids_array;
        
        // Set query if provided
        if ($query !== null) {
            $json_query = json_encode($query);
            $query_str = $this->ffi->new("char[" . strlen($json_query) + 1 . "]", false);
            FFI::memcpy($query_str, $json_query, strlen($json_query));
            $request->query = FFI::cast("char *", FFI::addr($query_str));
        } else {
            $request->query = null;
        }

        $request->recursive = $recursive;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->delete_many($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        // Remove FFI::free($ids_array) since it's not needed and causes the error
        if ($query !== null && isset($query_str)) {
            FFI::free($query_str);
        }

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_delete_many_response($response);
            throw new Exception($error_message);
        }

        $affected_rows = $response->affectedrows;
        $this->ffi->free_delete_many_response($response);
        return $affected_rows;
    }
}

