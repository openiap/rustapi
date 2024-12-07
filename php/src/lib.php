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
        print_r("loading library header from " . __DIR__ . "/../../clib_openiap.h\n");
        print_r("loading library from /home/allan/code/rust/openiap/target/debug/libopeniap_clib.so\n");
        $this->ffi = FFI::cdef(
            file_get_contents(__DIR__ . "/../../clib_openiap.h"),
            // __DIR__ . "/lib.so"
            "/home/allan/code/rust/openiap/target/debug/libopeniap_clib.so"
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
}

