<?php
// opcache_compile_file(__DIR__ . "/lib.php");
$autoload = __DIR__ . '/../vendor/autoload.php';
if (file_exists($autoload)) {
    require $autoload;
} else {
    echo "vendor missing, please run composer install \n";
    exit(1);
}
use React\EventLoop\Loop;
use React\EventLoop\Factory;

if (!defined('STDIN') || stream_set_blocking(STDIN, false) !== true) {
    fwrite(STDERR, 'ERROR: Unable to set STDIN non-blocking (not CLI or Windows?)' . PHP_EOL);
    exit(1);
}

require_once __DIR__ . '/../src/Client.php';
use openiap\Client;
if (Client::load_dotenv() == false) {
    echo "env missing, please create .env file \n";
    exit(1);
}
try {
    // Example Usage

    $client = new Client();
    // $client->enable_tracing("openiap=debug", "new");
    $client->enable_tracing("openiap=info", "");

    // print("Init events\n");
    // $eventId = $client->on_client_event(function($event) {
    //     // print("EVENT !!!!\n");
    //     echo "Event: " . $event['event'] . ", Reason: " . $event['reason'] . "\n";
    // });
    // print("Event ID: $eventId\n");
    $client->connect("");
    $client->info("Successfully connected to server");

    // Handler state variables
    $f64_handler = null;
    $u64_handler = null;
    $i64_handler = null;

    Loop::addReadStream(STDIN, function ($stream) use ($client, &$f64_handler, &$u64_handler, &$i64_handler) {
        $chunk = \trim(\fread($stream, 64 * 1024));
        switch ($chunk) {
            case 'q':
                $entities = $client->Query("entities", []);
                print_r($entities);
                break;
            case '1':
                $client->enable_tracing("", "");
                break;
            case '2':
                $client->enable_tracing("openiap=new", "");
                break;
            case '3':
                $client->enable_tracing("openiap=debug", "new");
                break;
            case 'r':
                $result = $client->register_queue("test2queue", function($message) {
                    print("Received message: " . json_encode($message) . "\n");
                    return ['payload' => "Bettina"];
                });
                break;
            case 'r2':
                // $result = $client->rpc("test2queue", ['payload' => "Test Message"], ['striptoken' => true]);
                $result = $client->rpc_async("test2queue", ['payload' => "Test Message"], 
                function($message) {
                    print("Received response: " . json_encode($message) . "\n");
                },
                ['striptoken' => true]);
                print_r($result);
                break;
            case 'm':
                $result = $client->queue_message("test2queue", ['payload' => "Test Message"], ['striptoken' => true]);
                print_r($result);
                break;
            case 'i':
                $result = $client->insert_one("entities", (object) ["name" => "testphp", "value" => 123]);
                print_r($result);
                break;
            case 'w':
                $watchid = $client->watch("entities", "[]", function($event, $event_counter)  {
                    echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . ", " . $event['document']['name'] . "\n";
                    // echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . "\n";
                });
                print("Watch ID: $watchid \n");
                break;
            case 'o':
                if ($f64_handler) {
                    $client->disable_observable_gauge("test_f64");
                    $client->info("stopped test_f64");
                    Loop::cancelTimer($f64_handler);
                    $f64_handler = null;
                } else {
                    $client->set_f64_observable_gauge("test_f64", 42.7, "test");
                    $client->info("started test_f64 to 42.7");
                    $f64_handler = Loop::addPeriodicTimer(30.0, function() use ($client) {
                        $value = mt_rand() / mt_getrandmax() * 50;
                        $client->info("Setting test_f64 to " . $value);
                        $client->set_f64_observable_gauge("test_f64", $value, "test");
                    });
                }
                break;
            case 'o2': 
                if ($u64_handler) {
                    $client->disable_observable_gauge("test_u64");
                    $client->info("stopped test_u64");
                    Loop::cancelTimer($u64_handler);
                    $u64_handler = null;
                } else {
                    $client->set_u64_observable_gauge("test_u64", 42, "test");
                    $client->info("started test_u64 to 42");
                    $u64_handler = Loop::addPeriodicTimer(30.0, function() use ($client) {
                        $value = mt_rand(0, 50);
                        $client->info("Setting test_u64 to " . $value);
                        $client->set_u64_observable_gauge("test_u64", $value, "test");
                    });
                }
                break;
            case 'o3':
                if ($i64_handler) {
                    $client->disable_observable_gauge("test_i64");
                    $client->info("stopped test_i64");
                    Loop::cancelTimer($i64_handler);
                    $i64_handler = null;
                } else {
                    $client->set_i64_observable_gauge("test_i64", 42, "test");
                    $client->info("started test_i64 to 42");
                    $i64_handler = Loop::addPeriodicTimer(30.0, function() use ($client) {
                        $value = mt_rand(0, 50);
                        $client->info("Setting test_i64 to " . $value); 
                        $client->set_i64_observable_gauge("test_i64", $value, "test");
                    });
                }
                break;
            case 'cc':
                try {
                    $result = $client->custom_command("getclients");
                    print("Custom command result: " . $result . "\n");
                } catch (Exception $e) {
                    print("Custom command error: " . $e->getMessage() . "\n");
                }
                break;
            case 'quit':
                // Cancel any active gauge handlers
                if ($f64_handler) Loop::cancelTimer($f64_handler);
                if ($u64_handler) Loop::cancelTimer($u64_handler);
                if ($i64_handler) Loop::cancelTimer($i64_handler);
                $client->free();
                unset($client);                
                Loop::removeReadStream($stream);
                stream_set_blocking($stream, true);
                fclose($stream);
                break;
            default:
                echo \strlen($chunk) . ' bytes' . PHP_EOL;
                break;
        }
        
    });

} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>