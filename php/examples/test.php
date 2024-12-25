<?php
// opcache_compile_file(__DIR__ . "/lib.php");
if (file_exists("./../vendor/autoload.php")) {
    // include __DIR__ . './../vendor/autoload.php';
    require __DIR__ . './../vendor/autoload.php';
} else {
    echo "vendor missing, cannot use watch/on_event \n";
}

require_once __DIR__ . '/../src/lib.php';
use openiap\Client;
if (Client::load_dotenv() == false) {
    echo "env missing, please create .env file \n";
    exit(1);
}
try {
    // Example Usage

    $client = new Client();
    
    // Enable tracing for debugging
    $client->enable_tracing("openiap=info", "new");
    
    // Connect and then signin
    $client->connect("");
    
    // // Example of signin with username/password
    // $jwt = $client->signin([
    //     'username' => 'testuser',
    //     'password' => 'testuser'
    // ]);
    // if ($jwt) {
    //     echo "Signed in successfully, received JWT\n";
    // }

    // // Example of signin with JWT
    // $jwt = $client->signin([
    //     'jwt' => $jwt,
    //     'validateonly' => true
    // ]);
    // if ($jwt) {
    //     echo "JWT validation successful\n";
        
    //     // Get current user info
    // }
    $user = $client->client_user();
    if ($user) {
        echo "Logged in as: " . $user['name'] . " (" . $user['username'] . ")\n";
        echo "User ID: " . $user['id'] . "\n";
        echo "Email: " . $user['email'] . "\n";
    }

    // Disable tracing when done debugging
    $client->disable_tracing();
    // $result = $client->push_workitem("q2", (object) ["testkey" => "hasvalue"], "php without file");
    $workitemfile = [__DIR__ . "/../../testfile.csv"];
    $result = $client->push_workitem("q2", (object) ["testkey" => "hasvalue"], "php with file", null, $workitemfile);
    $downloadfolder = __DIR__ . "/downloads";
    if(!file_exists($downloadfolder)) { mkdir($downloadfolder, 0777, true); }
    $result = $client->pop_workitem("q2", $downloadfolder);
    $result['name'] = "test workitem updated";
    $result['state'] = "successful";
    $result = $client->update_workitem($result);
    $client->delete_workitem($result['id']);


    $result = $client->register_queue("testqueue");
    print("Registered queue as: " . $result . "\n");

    $queuename = $client->register_exchange("test2exchange", "fanout");
    print("Registered exchange with queue: " . $result . "\n");

    $client->queue_message("testqueue", ['test' => "test message"], ['striptoken' => true]);
    $client->queue_message("", ['test' => "test message"], ['exchangename' => 'test2exchange', 'striptoken' => true]);

    $client->unregister_queue("queuename");
    $client->unregister_queue("testqueue");

    // Test count function
    print("\nTesting Count function:\n");
    $count = $client->count("testphpcollection", ['age' => ['$gt' => 25]]);
    print("Number of documents with age > 25: $count\n");

    // Test distinct function
    print("\nTesting Distinct function:\n");
    $distinctAges = $client->distinct("testphpcollection", "_type");
    print("Distinct ages in collection: " . implode(", ", $distinctAges) . "\n");
    // print("Distinct ages in collection: " . $distinctAges . "\n");

    // Test index operations
    print("\nTesting Index Operations:\n");
    
    // Create an index
    print("Creating index on 'name' field...\n");
    $indexDefinition = ['name' => 1];  // 1 for ascending
    $options = ['unique' => true];
    try {
        $client->createIndex("testphpcollection", $indexDefinition, $options, "name_index");
    } catch (\Throwable $th) {
        print("Error creating index: " . $th->getMessage() . "\n");
        $options = ['unique' => false];
        $client->createIndex("testphpcollection", $indexDefinition, $options, "name_index");
    }

    // List all indexes
    print("Listing all indexes:\n");
    $indexes = $client->getIndexes("testphpcollection");
    print_r($indexes);

    // Drop the index
    print("Dropping index 'name_index'...\n");
    $client->dropIndex("testphpcollection", "name_index");

    // Verify index was dropped
    print("Indexes after dropping:\n");
    $indexes = $client->getIndexes("testphpcollection");
    print_r($indexes);

    $entities = $client->Query("testphpcollection", []);
    print_r("Query returned: " . count($entities) . " documents\n");

    // Test aggregation pipeline
    print("\nTesting Aggregate function:\n");
    $pipeline = [
        ['$match' => ['age' => ['$exists' => true]]],
        ['$group' => [
            '_id' => null,
            'avgAge' => ['$avg' => '$age'],
            'count' => ['$sum' => 1]
        ]]
    ];
    $aggregateResults = $client->aggregate("testphpcollection", $pipeline);
    print("Aggregate Results:\n");
    print_r($aggregateResults);
    $aggregateResults = $client->aggregate("testphpcollection", []);
    print_r("Aggregate returned: " . count($aggregateResults) . " documents\n");

    $events_triggered = 0;
    $watchid = $client->watch("testphpcollection", "[]", function($event, $event_counter) use ($events_triggered) {
        // echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . ", " . $event['document']['name'] . "\n";
        echo "Watch: " . $event['id'] . ", Operation: " . $event['operation'] . "\n";
        $events_triggered = $events_triggered + 1;

    });
    print("Watch ID: $watchid \n");
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "Jane Doe", "age" => 30, "now" => time() ], 'name');
    // $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    // $i = 0; $seconds = 60;
    // while ($i < $seconds && $events_triggered < 3) {
    //     $i++;
    //     print("sleeping for 1 second, $i of $seconds\n");
    //     sleep(1);
    // }
    $uploadfilename = __DIR__ . "/../../testfile.csv";
    $downloadfolder = __DIR__ . "/downloads";
    if(!file_exists($downloadfolder)) { mkdir($downloadfolder, 0777, true); }
    $result = $client->upload($uploadfilename, "phptestfile.csv");
    print("Upload result: " . $result . "\n");
    $result = $client->download($result, "", $downloadfolder, "downloadedfile.csv");

    print("create collections\n");
    $client->createCollection("testphpexpcollection", [ "expire_after_seconds" => 10 ]);
    $result = $client->insertOne("testphpexpcollection", [ "name" => "testphpexpcollection" ]);
    // update name on $result
    $result['name'] = "testphpexpcollection updated";
    $client->updateOne("testphpexpcollection", $result);

    $client->createCollection("testphpcollection");

    print("insert or update 3 times\n");
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "Jane Doe", "age" => 30, "now" => time() ], 'name');
    $result = $client->insertOrUpdateOne("testphpcollection", [ "name" => "John Doe", "age" => 30, "now" => time() ], 'name');
    
    $collections = $client->listCollections();
    foreach ($collections as $collection) {
        if ($collection['name'] == "testphpcollection") {
            echo "Collection created successfully!\n";
            break;
        }
    }

    $items = [
        ["name" => "Alice Smith", "age" => 25],
        ["name" => "Bob Johnson", "age" => 35],
        ["name" => "Carol White", "age" => 28]
    ];
    $result = $client->insertMany("testphpcollection", $items);

    if (!empty($result[0]['_id'])) {
        $affected = $client->deleteOne("testphpcollection", $result[0]['_id']);
        echo "Deleted document count: $affected\n";
    }
    

    // Fix the MongoDB query operator syntax
    $query = ['age' => ['$gt' => 30]]; // Note the quotes around '$gt'
    $affected = $client->deleteMany("testphpcollection", $query);
    echo "Deleted documents count: $affected\n";

    // $client->dropCollection("testphpcollection");
    // $client->dropCollection("testphpexpcollection");

} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
$client->free();
unset($client);
?>