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

    $document = [
        "name" => "John Doe",
        "age" => 30
    ];
    
    $result = $client->insertOrUpdateOne("testphpcollection", $document, 'name');
    $result = $client->insertOrUpdateOne("testphpcollection", $document, 'name');
    
    $collections = $client->listCollections();
    foreach ($collections as $collection) {
        if ($collection['name'] == "testphpcollection") {
            echo "Collection created successfully!\n";
            break;
        }
    }
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