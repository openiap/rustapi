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
    private $clientevents = array();
    // private $next_watch_interval = 1000000; // microseconds (1 second)
    private $next_watch_interval = 1; // microseconds (1 second)
    private $eventProcesses = array();
    private $watches = array();

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
        // Terminate all child processes before destroying the client
        foreach ($this->eventProcesses as $pid) {
            posix_kill($pid, SIGTERM);
            pcntl_waitpid($pid, $status);
        }
        foreach ($this->watches as $pid) {
            posix_kill($pid, SIGTERM);
            pcntl_waitpid($pid, $status);
        }
        $this->ffi->free_client($this->client);
    }
    public function  enable_tracing($rust_log, $tracing) {
        print_r("enabling tracing with rust_log=$rust_log, tracing=$tracing\n");
        $this->ffi->enable_tracing($rust_log, $tracing);
    }
    public function  connect($url) {
        print_r("connecting...\n");
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

    // so pparently, php does not like callbacks, according to
    // https://www.php.net/manual/en/ffi.examples-callback.php
    // so we have to use pcntl_fork to handle events in a separate process
    // and poll for events in the child process
    public function on_client_event($callback) {
        if (!extension_loaded('pcntl')) {
            throw new Exception("pcntl extension is not loaded. Required for non-blocking event handling.");
        }

        $response = $this->ffi->on_client_event($this->client);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_event_response($response);
            throw new Exception($error_message);
        }
        $eventid = FFI::string($response->eventid);
        $this->ffi->free_event_response($response);

        $this->clientevents[$eventid] = true;
        
        // Fork a new process to handle events
        $pid = pcntl_fork();
        if ($pid == -1) {
            throw new Exception("Could not fork process");
        } else if ($pid) {
            // Parent process
            $this->eventProcesses[$eventid] = $pid;
            return $eventid;
        } else {
            // Child process
            $event_counter = 0;
            while ($this->clientevents[$eventid]) {
                $hadone = false;
                do {
                    // print("next_client_event\n");
                    $responsePtr = $this->ffi->next_client_event($eventid);
                    if ($responsePtr === null) {
                        print("responsePtr is null\n");
                        $hadone = false;
                        continue;
                    }

                    if ($responsePtr->event === null) {
                        // print("responsePtr->event is null\n");
                        $hadone = false;
                    } else if (strlen(FFI::string($responsePtr->event)) == 0) {
                        print("responsePtr->event is empty string\n");
                        $hadone = false;
                    } else if ($responsePtr->event !== null && strlen(FFI::string($responsePtr->event)) > 0) {
                        print("************************************\n");
                        print("Event\n");
                        print("************************************\n");
                        $hadone = true;
                        $event_counter++;
                        $event = array(
                            'event' => FFI::string($responsePtr->event),
                            'reason' => ""
                            // 'reason' => FFI::string($responsePtr->reason)
                        );
                        try {
                            $callback($event, $event_counter);
                        } catch (Exception $error) {
                            print_r("Error in client event callback: " . $error->getMessage() . "\n");
                        }
                    } else {
                        print("ELSE!!!! responsePtr->event is null\n");
                        $hadone = false;
                    }
                    $this->ffi->free_client_event($responsePtr);
                } while ($hadone);
                // usleep($this->next_watch_interval);
                sleep($this->next_watch_interval);
            }
            print("***************************\n");
            print("Exiting child process\n");
            print("***************************\n");
            exit(0); // End child process
        }
    }

    public function off_client_event($eventid) {
        print_r("off_client_event\n");
        $response = $this->ffi->off_client_event($eventid);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_off_event_response($response);
            throw new Exception($error_message);
        }
        $this->ffi->free_off_event_response($response);
        
        // Stop the event loop in child process
        if (isset($this->clientevents[$eventid])) {
            $this->clientevents[$eventid] = false;
            unset($this->clientevents[$eventid]);
        }

        // Terminate the child process
        if (isset($this->eventProcesses[$eventid])) {
            posix_kill($this->eventProcesses[$eventid], SIGTERM);
            pcntl_waitpid($this->eventProcesses[$eventid], $status);
            unset($this->eventProcesses[$eventid]);
        }
    }

    public function watch($collectionname, $paths, $callback) {
        $request = $this->ffi->new('struct WatchRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Set paths
        if (is_array($paths)) {
            $paths = json_encode($paths);
        }
        $paths_str = $this->ffi->new("char[" . strlen($paths) + 1 . "]", false);
        FFI::memcpy($paths_str, $paths, strlen($paths));
        $request->paths = FFI::cast("char *", FFI::addr($paths_str));

        // Make the call
        $response = $this->ffi->watch($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($paths_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_watch_response($response);
            throw new Exception($error_message);
        }

        $watchid = FFI::string($response->watchid);
        $this->ffi->free_watch_response($response);

        // Fork a new process to handle watch events
        $pid = pcntl_fork();
        if ($pid == -1) {
            throw new Exception("Could not fork process");
        } else if ($pid) {
            // Parent process
            $this->watches[$watchid] = $pid;
            return $watchid;
        } else {
            // Child process
            $event_counter = 0;
            while (true) {
                $hadone = false;
                do {
                    $responsePtr = $this->ffi->next_watch_event($watchid);
                    if ($responsePtr === null) {
                        print_r("responsePtr is null\n");
                        $hadone = false;
                        continue;
                    }

                    try {
                        $id = "";
                        $operation = "";
                        $document = "{}";
                        if ($responsePtr->id !== null) { $id = FFI::string($responsePtr->id); }
                        if ($responsePtr->operation !== null) { $operation = FFI::string($responsePtr->operation); }
                        if ($responsePtr->document !== null) { $document = FFI::string($responsePtr->document); }

                        print_r("************************************\n");
                        print_r("Watch Event\n");
                        print_r("************************************\n");
                        $hadone = true;
                        $event_counter++;
                        $event = array(
                            'id' => $id,
                            'operation' => $operation,
                            'document' => json_decode($document, true)
                        );
                        print_r("event: " . json_encode($event) . "\n");
                        try {
                            $callback($event, $event_counter);
                        } catch (Exception $error) {
                            print_r("Error in watch callback: " . $error->getMessage() . "\n");
                        }
                    // }
                    } catch (\Throwable $th) {
                        print_r("Error in watch callback: " . $th->getMessage() . "\n");
                    }

                    $this->ffi->free_watch_event($responsePtr);
                } while ($hadone);
                sleep($this->next_watch_interval);
            }
        }
    }

    public function unwatch($watchid) {
        print_r("unwatch\n");
        $response = $this->ffi->unwatch($watchid);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_unwatch_response($response);
            throw new Exception($error_message);
        }
        $this->ffi->free_unwatch_response($response);
        
        // Terminate the child process
        if (isset($this->watches[$watchid])) {
            posix_kill($this->watches[$watchid], SIGTERM);
            pcntl_waitpid($this->watches[$watchid], $status);
            unset($this->watches[$watchid]);
        }
    }

}

