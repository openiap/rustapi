<?php
require_once 'lib.php';
use openiap\Client;
try {
    $client = new Client();
    // $client->enable_tracing("openiap=debug", "new");
    $client->connect("grpc://grpc.demo.openiap.io:443");

    $client->createCollection("testphpexpcollection", [ "expire_after_seconds" => 10 ]);
    $client->insertOne("testphpexpcollection", [ "name" => "testphpexpcollection" ]);
    $client->createCollection("testphpcollection");

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
    // $collections = $client->listCollections();
    // foreach ($collections as $collection) {
    //     if ($collection['name'] == "testphpcollection") {
    //         echo "Collection not dropped!\n";
    //         break;
    //     }
    // }

} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>