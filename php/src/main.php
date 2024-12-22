<?php
// opcache_compile_file(__DIR__ . "/lib.php");
require __DIR__ . './../vendor/autoload.php';


require_once 'lib.php';
use openiap\Client;
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

    $events_triggered = 0;
    $watchid = $client->watch("testphpcollection", "[]", function($event, $event_counter) {
        // echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . ", " . $event['document']['name'] . "\n";
        echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . "\n";
        $events_triggered = $events_triggered + 1;

    });
    print("Watch ID: $watchid \n");
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "Jane Doe", "age" => 30, "now" => time() ], 'name');
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    $i = 0; $seconds = 60;
    while ($i < $seconds && $events_triggered < 3) {
        $i++;
        print("sleeping for 1 second, $i of $seconds\n");
        sleep(1);
    }

    // print("create collections\n");
    // $client->createCollection("testphpexpcollection", [ "expire_after_seconds" => 10 ]);
    // $client->insertOne("testphpexpcollection", [ "name" => "testphpexpcollection" ]);
    // $client->createCollection("testphpcollection");

    // print("insert or update 3 times\n");
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "Jane Doe", "age" => 30, "now" => time() ], 'name');
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    
    // $collections = $client->listCollections();
    // foreach ($collections as $collection) {
    //     if ($collection['name'] == "testphpcollection") {
    //         echo "Collection created successfully!\n";
    //         break;
    //     }
    // }

    // $items = [
    //     ["name" => "Alice Smith", "age" => 25],
    //     ["name" => "Bob Johnson", "age" => 35],
    //     ["name" => "Carol White", "age" => 28]
    // ];
    // $result = $client->insertMany("testphpcollection", $items);

    // if (!empty($result[0]['_id'])) {
    //     $affected = $client->deleteOne("testphpcollection", $result[0]['_id']);
    //     echo "Deleted document count: $affected\n";
    // }
    

    // // Fix the MongoDB query operator syntax
    // $query = ['age' => ['$gt' => 30]]; // Note the quotes around '$gt'
    // $affected = $client->deleteMany("testphpcollection", $query);
    // echo "Deleted documents count: $affected\n";

    // $client->dropCollection("testphpcollection");
    // $client->dropCollection("testphpexpcollection");

} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>