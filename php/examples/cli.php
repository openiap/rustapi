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
    $client->enable_tracing("openiap=info", "new");

    // print("Init events\n");
    // $eventId = $client->on_client_event(function($event) {
    //     // print("EVENT !!!!\n");
    //     echo "Event: " . $event['event'] . ", Reason: " . $event['reason'] . "\n";
    // });
    // print("Event ID: $eventId\n");
    $client->connect("");

    Loop::addReadStream(STDIN, function ($stream) use ($client) {
        $chunk = \trim(\fread($stream, 64 * 1024));
        switch ($chunk) {
            case 'q':
                $entities = $client->Query("entities", []);
                print_r($entities);
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
            case 'quit':
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