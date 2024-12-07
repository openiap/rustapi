<?php
// lib.php

// Load the shared library and define FFI functionality
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
    function enable_tracing($rust_log, $tracing) {
        print_r("enabling tracing with rust_log=$rust_log, tracing=$tracing\n");
        // void enable_tracing(const char *rust_log, const char *tracing);
        $this->ffi->enable_tracing($rust_log, $tracing);
    }
    function connect($url) {
        print_r("connecting to $url\n");
        $response = $this->ffi->client_connect($this->client, $url);
        if ($response->success) {
            echo "Connected successfully!\n";
        } else {
            echo "Error: " . FFI::string($response->error) . "\n";
        }
        $this->ffi->free_connect_response($response);
    }
    function listCollections() {
        $response = $this->ffi->list_collections($this->client, false);
        $collections = [];
        if ($response->success) {
            $collections = explode(",", FFI::string($response->results));
            echo "Collections: " . $collections . "\n";
        } else {
            echo "Error: " . FFI::string($response->error) . "\n";
        }
        $this->ffi->free_list_collections_response($response);
        return $collections;
    }
    function createCollection($collection) {
        /*
typedef struct CreateCollectionRequestWrapper {
  const char *collectionname;
  struct ColCollationWrapper *collation;
  struct ColTimeseriesWrapper *timeseries;
  int32_t expire_after_seconds;
  bool change_stream_pre_and_post_images;
  bool capped;
  int32_t max;
  int32_t size;
  int32_t request_id;
} CreateCollectionRequestWrapper;
typedef struct CreateCollectionResponseWrapper {
  bool success;
  const char *error;
  int32_t request_id;
} CreateCollectionResponseWrapper;
typedef void (*CreateCollectionCallback)(struct CreateCollectionResponseWrapper *wrapper);
        */
        $request = FFI::new("CreateCollectionRequestWrapper");
        $request->collection = FFI::string($collection);
        $response = $this->ffi->create_collection($this->client, $request);
        if ($response->success) {
            echo "Collection created successfully!\n";
        } else {
            echo "Error: " . FFI::string($response->error) . "\n";
        }
    }

}

