<?php
require_once 'lib.php';

// Call the FFI functions from lib.php
try {
    $client = new Client();
    $client->enable_tracing("openiap=info", "new");
    $client->connect("grpc://grpc.demo.openiap.io:443");

    // List a collections
    // $collections = $client->listCollections();
    // print_r($collections);

    $client->createCollection("phptestcollection");


} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>