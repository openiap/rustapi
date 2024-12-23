<?php
namespace openiap;

use \Exception;
use \FFI;
use React\EventLoop\Loop;

if (!extension_loaded('FFI')) {
    throw new Exception("FFI extension is not loaded");
}
  
class Client {
    private $client;
    public $ffi;
    private $clientevents = array();
    // private $next_watch_interval = 1000000; // microseconds (1 second)
    private $next_watch_interval = 1; // microseconds (1 second)
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
        $client = $this->ffi->create_client();
        if ($client === null) {
            throw new Exception("Failed to create client");
        }
        return $client;
    }
    
    public function __construct() {
        $this->loadLibrary();
        $this->client = $this->createClient();
        $this->set_agent_name("php");
    }

    public function free() {
        $this->disconnect();
        $this->__destruct();
    }

    public function __destruct() {
        foreach ($this->watches as $watch) {
            // print_r("unwatching $watchid\n");
            // Loop::cancelTimer($this->watches[$watchid]);
            Loop::cancelTimer($watch);
        }
        $this->ffi->free_client($this->client);
    }
    public function set_next_watch_interval($interval) {
        $this->next_watch_interval = $interval;
    }
    public static function load_dotenv(string $envfile = null) {
        if($envfile == null) {
            $envfile = __DIR__ . '/.env';
            if (file_exists($envfile) == false) $envfile = null;
        }
        if($envfile == null) {
            $envfile = __DIR__ . '/../.env';
            if (file_exists($envfile) == false) $envfile = null;
        }
        if($envfile == null) {
            $envfile = __DIR__ . '/../../.env';
        }
        if (file_exists($envfile) == false) return false;
        $content = file_get_contents($envfile);
        $lines = explode("\n", $content);
        foreach ($lines as $line) {
            $line = trim($line);
            if (empty($line)) {
                continue;
            }
            $parts = explode("=", $line);
            if (count($parts) == 2) {
                $key = $parts[0];
                $value = $parts[1];
                $t = strpos($key, "#");
                if($t == false && $t !== 0) {
                    print ("Setting env: $key\n");
                    putenv("$key=$value");
                }
            }
        }
        $apiurl = getenv('apiurl');
        print ("****************************************************\n");
        print ("apiurl: $apiurl\n");
        print ("****************************************************\n");
        return true;
    }
    public function enable_tracing($rust_log, $tracing) {
        print_r("enabling tracing with rust_log=$rust_log, tracing=$tracing\n");
        $this->ffi->enable_tracing($rust_log, $tracing);
    }
    public function disable_tracing() {
        $this->ffi->disable_tracing();
    }

    private function set_agent_name($agent_name) {
        $this->ffi->client_set_agent_name($this->client, $agent_name);
    }

    private function set_agent_version($agent_version) {
        $this->ffi->client_set_agent_version($this->client, $agent_version);
    }

    private function strToCharP($str) {
        if ($str === null) return null;
        $len = strlen($str);
        $tmp = $this->ffi->new("char[$len + 1]", false);
        FFI::memcpy($tmp, $str, $len);
        return FFI::cast("char *", FFI::addr($tmp));
    }
    
    public function signin($options = array()) {
        $request = $this->ffi->new('struct SigninRequestWrapper');
        
        // Helper function to convert string to char*
        
        // Set optional parameters with proper casting
        $request->username = $this->strToCharP($options['username'] ?? null);
        $request->password = $this->strToCharP($options['password'] ?? null);
        $request->jwt = $this->strToCharP($options['jwt'] ?? null);
        $request->agent = $this->strToCharP($options['agent'] ?? null);
        $request->version = $this->strToCharP($options['version'] ?? null);
        
        // Set boolean flags
        $request->longtoken = isset($options['longtoken']) ? $options['longtoken'] : false;
        $request->validateonly = isset($options['validateonly']) ? $options['validateonly'] : false;
        $request->ping = isset($options['ping']) ? $options['ping'] : false;
        $request->request_id = 0;

        $response = $this->ffi->signin($this->client, FFI::addr($request));
        
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_signin_response($response);
            throw new Exception($error_message);
        }

        $jwt = null;
        if ($response->jwt) {
            $jwt = FFI::string($response->jwt);
        }

        $this->ffi->free_signin_response($response);
        return $jwt;
    }

    public function connect($url) {
        $response = $this->ffi->client_connect($this->client, $url);
        if ($response->success) {
            // echo "Connected successfully!\n";
        } else {
            throw new Exception(FFI::string($response->error));
            // echo "Error: " . FFI::string($response->error) . "\n";
        }
        $this->ffi->free_connect_response($response);
    }

    public function disconnect() {
        $this->ffi->client_disconnect($this->client);
    }

    public function listCollections() {
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

    public function getIndexes($collectionname) {
        $response = $this->ffi->get_indexes($this->client, $collectionname);
        $indexes = [];
        if ($response->success) {
            $indexes = json_decode(FFI::string($response->results), true);
        } else {
            throw new Exception(FFI::string($response->error));
        }
        $this->ffi->free_get_indexes_response($response);
        return $indexes;
    }

    public function createIndex($collectionname, $index, $options = null, $name = null) {
        $request = $this->ffi->new('struct CreateIndexRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Convert index to JSON string
        $json_index = json_encode($index);
        $index_str = $this->ffi->new("char[" . strlen($json_index) + 1 . "]", false);
        FFI::memcpy($index_str, $json_index, strlen($json_index));
        $request->index = FFI::cast("char *", FFI::addr($index_str));

        // Set options if provided
        if ($options !== null) {
            $json_options = json_encode($options);
            $options_str = $this->ffi->new("char[" . strlen($json_options) + 1 . "]", false);
            FFI::memcpy($options_str, $json_options, strlen($json_options));
            $request->options = FFI::cast("char *", FFI::addr($options_str));
        }

        // Set name if provided
        if ($name !== null) {
            $name_str = $this->ffi->new("char[" . strlen($name) + 1 . "]", false);
            FFI::memcpy($name_str, $name, strlen($name));
            $request->name = FFI::cast("char *", FFI::addr($name_str));
        }

        $request->request_id = 0;

        $response = $this->ffi->create_index($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($index_str);
        if ($options !== null) FFI::free($options_str);
        if ($name !== null) FFI::free($name_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_create_index_response($response);
            throw new Exception($error_message);
        }

        $this->ffi->free_create_index_response($response);
    }

    public function dropIndex($collectionname, $name) {
        $response = $this->ffi->drop_index($this->client, $collectionname, $name);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_drop_index_response($response);
            throw new Exception($error_message);
        }
        $this->ffi->free_drop_index_response($response);
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
            $json_query = json_encode((object)$query);
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
    private static function watch_worker(Client $me, string $watchid, \Closure $callback){
        $event_counter = 0;
        $hadone = true;
        while ($hadone) {
            $hadone = false;
            do {
                $responsePtr = $me->ffi->next_watch_event($watchid);
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

                    if($id == "") {
                        $hadone = false;
                        continue;
                    }

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
                        if(is_callable($callback)) {
                            $callback($event, $event_counter);
                        }
                    } catch (Exception $error) {
                        print_r("Error in watch callback: " . $error->getMessage() . "\n");
                    }
                } catch (\Throwable $th) {
                    print_r("Error in watch callback: " . $th->getMessage() . "\n");
                }

                $me->ffi->free_watch_event($responsePtr);
            } while ($hadone);
            sleep($me->next_watch_interval);
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

        $timer = Loop::addPeriodicTimer(0.1, function () use ($watchid, $callback) {
            Client::watch_worker($this, $watchid, $callback);
        });
        $this->watches[$watchid] = $timer;
        return $watchid;
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
        if (isset($this->watches[$watchid])) {
            Loop::cancelTimer($this->watches[$watchid]);
            unset($this->watches[$watchid]);
        }
    }

    public function query($collectionname, $query = null, $projection = null, $orderby = null, $top = 0, $skip = 0, $explain = false, $queryas = null) {
        $request = $this->ffi->new('struct QueryRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        if ($query === null) {
            $query = array();
        }
        $json_query = json_encode((object)$query);
        $query_str = $this->ffi->new("char[" . strlen($json_query) + 1 . "]", false);
        FFI::memcpy($query_str, $json_query, strlen($json_query));
        $request->query = FFI::cast("char *", FFI::addr($query_str));

        // Set projection if provided
        if ($projection !== null) {
            $json_proj = json_encode($projection);
            $proj_str = $this->ffi->new("char[" . strlen($json_proj) + 1 . "]", false);
            FFI::memcpy($proj_str, $json_proj, strlen($json_proj));
            $request->projection = FFI::cast("char *", FFI::addr($proj_str));
        } else {
            $request->projection = null;
        }

        // Set orderby if provided
        if ($orderby !== null) {
            $json_order = json_encode($orderby);
            $order_str = $this->ffi->new("char[" . strlen($json_order) + 1 . "]", false);
            FFI::memcpy($order_str, $json_order, strlen($json_order));
            $request->orderby = FFI::cast("char *", FFI::addr($order_str));
        } else {
            $request->orderby = null;
        }

        // Set queryas if provided
        if ($queryas !== null) {
            $queryas_str = $this->ffi->new("char[" . strlen($queryas) + 1 . "]", false);
            FFI::memcpy($queryas_str, $queryas, strlen($queryas));
            $request->queryas = FFI::cast("char *", FFI::addr($queryas_str));
        } else {
            $request->queryas = null;
        }

        $request->top = $top;
        $request->skip = $skip;
        $request->explain = $explain;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->query($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        if ($query !== null && isset($query_str)) {
            FFI::free($query_str);
        }
        if ($projection !== null && isset($proj_str)) {
            FFI::free($proj_str);
        }
        if ($orderby !== null && isset($order_str)) {
            FFI::free($order_str);
        }
        if ($queryas !== null && isset($queryas_str)) {
            FFI::free($queryas_str);
        }

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_query_response($response);
            throw new Exception($error_message);
        }

        $results = null;
        if ($response->results) {
            $results = json_decode(FFI::string($response->results), true);
        }

        $this->ffi->free_query_response($response);
        return $results;
    }

    public function aggregate($collectionname, $aggregates, $hint = null, $queryas = null, $explain = false) {
        $request = $this->ffi->new('struct AggregateRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Convert aggregates to JSON string
        $json_agg = json_encode($aggregates);
        $agg_str = $this->ffi->new("char[" . strlen($json_agg) + 1 . "]", false);
        FFI::memcpy($agg_str, $json_agg, strlen($json_agg));
        $request->aggregates = FFI::cast("char *", FFI::addr($agg_str));

        // Set hint if provided
        if ($hint !== null) {
            $json_hint = json_encode($hint);
            $hint_str = $this->ffi->new("char[" . strlen($json_hint) + 1 . "]", false);
            FFI::memcpy($hint_str, $json_hint, strlen($json_hint));
            $request->hint = FFI::cast("char *", FFI::addr($hint_str));
        } else {
            $request->hint = null;
        }

        // Set queryas if provided
        if ($queryas !== null) {
            $queryas_str = $this->ffi->new("char[" . strlen($queryas) + 1 . "]", false);
            FFI::memcpy($queryas_str, $queryas, strlen($queryas));
            $request->queryas = FFI::cast("char *", FFI::addr($queryas_str));
        } else {
            $request->queryas = null;
        }

        $request->explain = $explain;
        $request->request_id = 0;

        // Make the call
        $response = $this->ffi->aggregate($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($agg_str);
        if ($hint !== null && isset($hint_str)) {
            FFI::free($hint_str);
        }
        if ($queryas !== null && isset($queryas_str)) {
            FFI::free($queryas_str);
        }

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_aggregate_response($response);
            throw new Exception($error_message);
        }

        $results = null;
        if ($response->results) {
            $results = json_decode(FFI::string($response->results), true);
        }

        $this->ffi->free_aggregate_response($response);
        return $results;
    }

    public function count($collectionname, $query = null, $queryas = null, $explain = false) {
        $request = $this->ffi->new('struct CountRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        if ($query === null) {
            $query = array();
        }
        $json_query = json_encode((object)$query);
        $query_str = $this->ffi->new("char[" . strlen($json_query) + 1 . "]", false);
        FFI::memcpy($query_str, $json_query, strlen($json_query));
        $request->query = FFI::cast("char *", FFI::addr($query_str));

        // Set queryas if provided
        if ($queryas !== null) {
            $queryas_str = $this->ffi->new("char[" . strlen($queryas) + 1 . "]", false);
            FFI::memcpy($queryas_str, $queryas, strlen($queryas));
            $request->queryas = FFI::cast("char *", FFI::addr($queryas_str));
        }

        $request->explain = $explain;
        $request->request_id = 0;

        $response = $this->ffi->count($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        if ($query !== null && isset($query_str)) FFI::free($query_str);
        if ($queryas !== null && isset($queryas_str)) FFI::free($queryas_str);

        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_count_response($response);
            throw new Exception($error_message);
        }

        $result = $response->result;
        $this->ffi->free_count_response($response);
        return $result;
    }

    public function distinct($collectionname, $field, $query = null, $queryas = null, $explain = false) {
        $request = $this->ffi->new('struct DistinctRequestWrapper');
        
        // Set collectionname
        $str = $this->ffi->new("char[" . strlen($collectionname) + 1 . "]", false);
        FFI::memcpy($str, $collectionname, strlen($collectionname));
        $request->collectionname = FFI::cast("char *", FFI::addr($str));

        // Set field
        $field_str = $this->ffi->new("char[" . strlen($field) + 1 . "]", false);
        FFI::memcpy($field_str, $field, strlen($field));
        $request->field = FFI::cast("char *", FFI::addr($field_str));

        if ($query === null) {
            $query = array();
        }
        $json_query = json_encode((object)$query);
        $query_str = $this->ffi->new("char[" . strlen($json_query) + 1 . "]", false);
        FFI::memcpy($query_str, $json_query, strlen($json_query));
        $request->query = FFI::cast("char *", FFI::addr($query_str));

        // Set queryas if provided
        if ($queryas !== null) {
            $queryas_str = $this->ffi->new("char[" . strlen($queryas) + 1 . "]", false);
            FFI::memcpy($queryas_str, $queryas, strlen($queryas));
            $request->queryas = FFI::cast("char *", FFI::addr($queryas_str));
        }

        $request->explain = $explain;
        $request->request_id = 0;

        $response = $this->ffi->distinct($this->client, FFI::addr($request));

        // Free allocated memory
        FFI::free($str);
        FFI::free($field_str);
        // if ($query !== null && isset($query_str)) FFI::free($query_str);
        // if ($queryas !== null && isset($queryas_str)) FFI::free($queryas_str);
        if (!$response->success) {
            $error_message = FFI::string($response->error);
            $this->ffi->free_distinct_response($response);
            throw new Exception($error_message);
        }
        $results = array();
        $len = $response->results_len;
        for ($i = 0; $i < $len; $i++) {
            $value = $response->results[$i];
            if ($value !== null) {
                print("test6 $value\n");
                $results[] = FFI::string($response->results[$i]);
            }
        }
        print("test7\n");
        print_r($results);

        $this->ffi->free_distinct_response($response);
        return $results;
    }

    public function client_user() {
        $userPtr = $this->ffi->client_user($this->client);
        if ($userPtr === null) {
            return null;
        }

        $user = array(
            'id' => $userPtr->id ? FFI::string($userPtr->id) : null,
            'name' => $userPtr->name ? FFI::string($userPtr->name) : null,
            'username' => $userPtr->username ? FFI::string($userPtr->username) : null,
            'email' => $userPtr->email ? FFI::string($userPtr->email) : null
        );

        # $this->ffi->free_user($userPtr);
        return $user;
    }

}

