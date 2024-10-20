#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin + Clone>() {}
// cargo test -- --test-threads 1
// cargo test --lib -- --test-threads 1
// cargo test --doc -- --test-threads 1
// cargo test --lib -- --nocapture --test-threads 1
// cargo test --doc -- --nocapture --test-threads 1
#[cfg(test)]
mod tests {
    use errors::OpenIAPError;
    use futures::stream::FuturesUnordered;
    use protos::*;
    // use tokio_tungstenite::tungstenite::handshake::client;
    use std::{env, future::Future, pin::Pin};
    use std::sync::Arc;
    use tokio::sync::{oneshot};

    use crate::{Client, ClientInner};
    use openiap_proto::*;

    use super::*;
    const TEST_URL: &str = "";
    #[test]
    fn normal_type() {
        is_normal::<Pin<Box<Client>>>();
        is_normal::<ClientInner>();
        is_normal::<SigninRequest>();
        is_normal::<SigninResponse>();
        is_normal::<QueryRequest>();
        is_normal::<QueryResponse>();
        is_normal::<DownloadRequest>();
        is_normal::<DownloadResponse>();
        is_normal::<UploadRequest>();
        is_normal::<UploadResponse>();
        is_normal::<BeginStream>();
        is_normal::<Stream>();
        is_normal::<EndStream>();
    }

    #[tokio::test()]
    async fn test_get_document_version() {
        // cargo test test_get_document_version -- --nocapture
        let client = Client::new();
        client.connect_async(TEST_URL).await.unwrap();

        let item = "{\"name\": \"test from rust\", \"_type\": \"test\"}";
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item: item.to_string(),
            j: true,
            w: 2,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        let response = match response {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "insert_one failed with {:?}", e);
                return;
            }
        };
        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();
        let item = format!("{{\"name\":\"updated from rust\", \"_id\": \"{}\"}}", _id);

        let query = UpdateOneRequest {
            collectionname: "entities".to_string(),
            item: item.to_string(),
            ..Default::default()
        };
        let response = client.update_one(query).await;
        _ = match response {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "update_one failed with {:?}", e);
                return;
            }
        };

        let query = GetDocumentVersionRequest {
            collectionname: "entities".to_string(),
            id: _id.to_string(),
            version: 0,
            ..Default::default()
        };
        let response = client.get_document_version(query).await;
        let response = match response {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false, "get_document_version failed with {:?}", e);
                return;
            }
        };
        let _obj = serde_json::from_str(&response);
        let _obj: serde_json::Value = match _obj {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(
                    false,
                    "parse get_document_version result failed with {:?}",
                    e
                );
                return;
            }
        };
        let name = _obj["name"].as_str().unwrap();
        let version = _obj["_version"].as_i64().unwrap();
        println!("version 0 Name: {}, Version: {}", name, version);
        assert_eq!(name, "test from rust");

        let query = GetDocumentVersionRequest {
            collectionname: "entities".to_string(),
            id: _id.to_string(),
            version: 1,
            ..Default::default()
        };
        let response = client.get_document_version(query).await;
        assert!(
            response.is_ok(),
            "test_get_document_version failed with {:?}",
            response.err().unwrap()
        );

        let _obj: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
        let name = _obj["name"].as_str().unwrap();
        let version = _obj["_version"].as_i64().unwrap();
        println!("version 1 Name: {}, Version: {}", name, version);
        assert_eq!(name, "updated from rust");

        let query = GetDocumentVersionRequest {
            collectionname: "entities".to_string(),
            id: _id.to_string(),
            version: -1,
            ..Default::default()
        };
        let response = client.get_document_version(query).await;
        assert!(
            response.is_ok(),
            "test_get_document_version failed with {:?}",
            response.err().unwrap()
        );

        let _obj: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
        let name = _obj["name"].as_str().unwrap();
        let version = _obj["_version"].as_i64().unwrap();
        println!("version -1 Name: {}, Version: {}", name, version);
        assert_eq!(name, "updated from rust");
    }
    #[tokio::test()]
    async fn test_query() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = QueryRequest {
            query: "{}".to_string(),
            projection: "{\"name\": 1}".to_string(),
            ..Default::default()
        };
        let response = client.query(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_multiple_query() {
        // cargo test test_multiple_query -- --nocapture
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let tasks = FuturesUnordered::<
            Pin<Box<dyn Future<Output = Result<QueryResponse, OpenIAPError>>>>,
        >::new();
        // for _ in 1..101 {
        for _ in 1..10 {
            let query = QueryRequest {
                query: "{}".to_string(),
                projection: "{\"name\": 1}".to_string(),
                ..Default::default()
            };
            tasks.push(Box::pin(client.query(query)));
        }
        let result = futures::future::join_all(tasks).await;
        println!("{:?}", result);
    }
    #[tokio::test()]
    async fn test_aggreate() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = AggregateRequest {
            collectionname: "entities".to_string(),
            aggregates: "[]".to_string(),
            ..Default::default()
        };
        let response = client.aggregate(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_aggreate_multiple() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let tasks = FuturesUnordered::<
            Pin<Box<dyn Future<Output = Result<AggregateResponse, OpenIAPError>>>>,
        >::new();
        // for _ in 1..101 {
            for _ in 1..10 {
            let query = AggregateRequest {
                collectionname: "entities".to_string(),
                aggregates: "[]".to_string(),
                ..Default::default()
            };
            tasks.push(Box::pin(client.aggregate(query)));
        }
        let result = futures::future::join_all(tasks).await;
        println!("{:?}", result);
    }
    #[tokio::test()]
    async fn test_count() {
        // cargo test test_count -- --nocapture
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = CountRequest {
            collectionname: "entities".to_string(),
            query: "{}".to_string(),
            ..Default::default()
        };
        let response = client.count(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_distinct() {
        // cargo test test_distinct -- --nocapture
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = DistinctRequest {
            collectionname: "entities".to_string(),
            field: "_type".to_string(),
            ..Default::default()
        };
        let response = client.distinct(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_insert_one() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item: "{\"name\": \"test from rust\", \"_type\": \"test\"}".to_string(),
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_insert_many() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let query = InsertManyRequest {
            collectionname: "entities".to_string(),
            items: "[{\"name\": \"test many from rust 1\", \"_type\": \"test\"}, {\"name\": \"test many from rust 2\", \"_type\": \"test\"}]".to_string(),
            ..Default::default()
        };
        let response = client.insert_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_update_one() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"update test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();
        let item = format!("{{\"name\":\"updated from rust\", \"_id\": \"{}\"}}", _id);

        let query = UpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_insert_or_update_one() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"insert or update one test from rust\", \"_type\": \"test\", \"age\": \"21\"}".to_string();
        let query = InsertOrUpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response).unwrap();
        let _id = _obj["_id"].as_str().unwrap();
        let age = _obj["age"].as_str().unwrap();
        assert!(
            age == "21",
            "Age did not match after first insert or update"
        );

        let item =
            "{\"name\":\"insert or update one test from rust\", \"age\": \"22\"}".to_string();

        let query = InsertOrUpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response2: {:?}", response);
        let _obj: serde_json::Value = serde_json::from_str(&response).unwrap();
        let _id2 = _obj["_id"].as_str().unwrap();
        let age = _obj["age"].as_str().unwrap();
        assert!(
            age == "22",
            "Age did not match after first insert or update"
        );

        assert!(_id == _id2, "ID did not match after update");
    }
    #[tokio::test()]
    async fn test_insert_or_update_many() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item1 = "{\"name\": \"insert or update many test from rust 1\", \"_type\": \"test\", \"age\": \"21\"}".to_string();
        let item2 = "{\"name\": \"insert or update many test from rust 2\", \"_type\": \"test\", \"age\": \"23\"}".to_string();
        let query = InsertOrUpdateManyRequest {
            collectionname: "entities".to_string(),
            items: format!("[{}, {}]", item1, item2),
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_many(query).await;

        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
        let _id1 = _obj[0]["_id"].as_str().unwrap();
        let _id2 = _obj[1]["_id"].as_str().unwrap();
        let age1 = _obj[0]["age"].as_str().unwrap();

        let item1 =
            "{\"name\":\"insert or update many test from rust 1\", \"age\": \"22\"}".to_string();
        let item2 =
            "{\"name\":\"insert or update many test from rust 2\", \"age\": \"24\"}".to_string();

        let query = InsertOrUpdateManyRequest {
            collectionname: "entities".to_string(),
            items: format!("[{}, {}]", item1, item2),
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response2: {:?}", response);
        let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
        let _id1_2 = _obj[0]["_id"].as_str().unwrap();
        let _id2_2 = _obj[1]["_id"].as_str().unwrap();
        let age1_2 = _obj[0]["age"].as_str().unwrap();

        assert!(_id1 == _id1_2, "ID1 did not match after update");
        assert!(_id2 == _id2_2, "ID2 did not match after update");
        assert!(
            age1 == "21",
            "Age1 did not match after first insert or update"
        );
        assert!(
            age1_2 == "22",
            "Age1 did not match after second insert or update"
        );
    }
    #[tokio::test()]
    async fn test_delete_one() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"delete test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteOneRequest {
            collectionname: "entities".to_string(),
            id: _id.to_string(),
            ..Default::default()
        };
        let response = client.delete_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_delete_many_query() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item =
            "{\"name\": \"delete many query test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteManyRequest {
            collectionname: "entities".to_string(),
            query: format!("{{\"_id\": \"{}\"}}", _id),
            ..Default::default()
        };
        let response = client.delete_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_delete_many_ids() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let item =
            "{\"name\": \"delete many ids test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteManyRequest {
            collectionname: "entities".to_string(),
            ids: vec![_id.to_string()],
            ..Default::default()
        };
        let response = client.delete_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_bad_login() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        println!("Client connected: {:?}", client.get_state());
        let response = client
            .signin(SigninRequest::with_userpass("testuser", "badpassword"))
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
                assert!(false, "login with bad password, did not fail");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    #[tokio::test()]
    async fn test_upload() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let response = client
            .upload(
                UploadRequest::filename("rust-test.csv"),
                "../../testfile.csv",
            )
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => {
                assert!(false, "Upload of testfile.csv failed with {:?}", e);
            }
        }
    }
    #[tokio::test()]
    async fn test_upload_as_guest() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        let response = client
            .upload(
                UploadRequest::filename("rust-test-user.csv"),
                "../../testfile.csv",
            )
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
                assert!(false, "Upload of testfile.csv did not fail as guest");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    #[tokio::test()]
    async fn test_download() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let response = client
            .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
            .await;
        println!("Download response: {:?}", response);
        assert!(
            !response.is_err(),
            "Download of file failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_download_as_guest() {
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let response = client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        println!("Signin response: {:?}", response);
        let response = client
            .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
            .await;
        println!("Download response: {:?}", response);
        assert!(
            response.is_err(),
            "Download of file as guest did not failed"
        );
    }
    #[tokio::test]
    async fn test_watch() { // cargo test test_watch -- --nocapture
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let response: std::result::Result<String, OpenIAPError> = client
            .watch(WatchRequest::new("", vec!["".to_string()]), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Watch event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;

        println!("Watch response: {:?}", response);

        assert!(
            response.is_ok(),
            "Watch failed with {:?}",
            response.err().unwrap()
        );

        let id = response.unwrap();

        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item: "{\"name\": \"testing watch from rust\", \"_type\": \"test\"}".to_string(),
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Inserted document: {:?}", response);

        // Await the watch event
        rx.await.unwrap();
        println!("Watch event received");

        client.unwatch(&id).await.unwrap();
    }
    #[tokio::test]
    async fn test_register_queue() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let response: std::result::Result<String, OpenIAPError> = client
            .register_queue(RegisterQueueRequest::byqueuename("secrettestqueue"), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Queue event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;

        println!("RegisterQueue response: {:?}", response);

        assert!(
            response.is_ok(),
            "RegisterQueue failed with {:?}",
            response.err().unwrap()
        );

        let queuename = response.unwrap();

        println!("Send message to queue: {:?}", queuename);
        let query = QueueMessageRequest {
            queuename: queuename.clone(),
            data: "{\"test\": \"message\"}".to_string(),
            striptoken: true,
            ..Default::default()
        };
        let response = client.queue_message(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );

        println!("Await the queue event");
        rx.await.unwrap();
        println!("Queue event received");

        client.unregister_queue(&queuename).await.unwrap();
    }
    #[tokio::test] // cargo test test_register_exchange -- --nocapture
    async fn test_register_exchange() {
        let exchangename = "secrettestexchange";
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let response = client
            .register_exchange(RegisterExchangeRequest::byexchangename(exchangename), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Queue event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;

        println!("RegisterExchange response: {:?}", response);

        assert!(
            response.is_ok(),
            "RegisterExchange failed with {:?}",
            response.err().unwrap()
        );

        let queuename = response.unwrap();

        println!("Send message to exchange: {:?}", exchangename);
        let query = QueueMessageRequest {
            exchangename: exchangename.to_string(),
            data: "{\"test\": \"message\"}".to_string(),
            striptoken: true,
            ..Default::default()
        };
        let response = client.queue_message(query).await;
        assert!(
            response.is_ok(),
            "test_exhange failed with {:?}",
            response.err().unwrap()
        );

        println!("Await the queue event");
        rx.await.unwrap();
        println!("Queue event received");

        client.unregister_queue(&queuename).await.unwrap();
    }
    #[tokio::test] // cargo test test_push_workitem -- --nocapture
    async fn test_push_workitem() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .push_workitem(PushWorkitemRequest {
                wiq: "rustqueue".to_string(),
                name: "test rust workitem".to_string(),
                files: vec![WorkitemFile {
                    filename: "../../testfile.csv".to_string(),
                    ..Default::default()
                }],
                payload: "{\"test\": \"message\"}".to_string(),
                // nextrun: Some(Timestamp::from(std::time::SystemTime::now() + std::time::Duration::from_secs(60))),
                ..Default::default()
            })
            .await;

        assert!(
            response.is_ok(),
            "PushWorkitem failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .pop_workitem(
                PopWorkitemRequest {
                    wiq: "rustqueue".to_string(),
                    ..Default::default()
                },
                Some(""),
            )
            .await;

        assert!(
            response.is_ok(),
            "PopWorkitem failed with {:?}",
            response.err().unwrap()
        );
        let mut workitem = response.unwrap().workitem.unwrap();
        workitem.name = "updated test rust workitem".to_string();
        workitem.payload = "{\"test\": \"updated message\"}".to_string();
        workitem.state = "successful".to_string();
        assert!(workitem.files.len() > 0, "workitem has no files");

        // delete file from workitem by setting id to empty string
        workitem.files[0].id = "".to_string();

        // delete testfile.csv if exsits, so it can be re-download when popping workitem
        if std::path::Path::new("testfile.csv").exists() {
            println!("Deleting testfile.csv");
            std::fs::remove_file("testfile.csv").unwrap();
        }
        let id = workitem.id.clone();

        let response = client
            .update_workitem(UpdateWorkitemRequest {
                workitem: Some(workitem),
                files: vec![
                    WorkitemFile {
                        filename: "../../train.csv".to_string(),
                        ..Default::default()
                    },
                    WorkitemFile {
                        filename: "testfile.csv".to_string(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .await;
        assert!(
            response.is_ok(),
            "UpdateWorkitem failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .delete_workitem(DeleteWorkitemRequest {
                id: id,
                ..Default::default()
            })
            .await;

        assert!(
            response.is_ok(),
            "DeleteWorkitem failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test] // cargo test test_push_workitems -- --nocapture
    async fn test_push_workitems() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .push_workitems(PushWorkitemsRequest {
                wiq: "rustqueue".to_string(),
                items: vec![
                    Workitem {
                        name: "test rust workitem 1".to_string(),
                        payload: "{\"test\": \"message\"}".to_string(),
                        files: vec![WorkitemFile {
                            filename: "../../testfile.csv".to_string(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    Workitem {
                        name: "test rust workitem 2".to_string(),
                        payload: "{\"test\": \"message\"}".to_string(),
                        files: vec![WorkitemFile {
                            filename: "../../testfile.csv".to_string(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .await;

        assert!(
            response.is_ok(),
            "PushWorkitems failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .pop_workitem(
                PopWorkitemRequest {
                    wiq: "rustqueue".to_string(),
                    ..Default::default()
                },
                Some(""),
            )
            .await;

        assert!(
            response.is_ok(),
            "PopWorkitem failed with {:?}",
            response.err().unwrap()
        );
        let mut workitem = response.unwrap().workitem.unwrap();
        workitem.name = "updated test rust workitem".to_string();
        workitem.payload = "{\"test\": \"updated message\"}".to_string();
        workitem.state = "successful".to_string();
        assert!(workitem.files.len() > 0, "workitem has no files");

        // delete file from workitem by setting id to empty string
        workitem.files[0].id = "".to_string();

        // delete testfile.csv if exsits, so it can be re-download when popping workitem
        if std::path::Path::new("testfile.csv").exists() {
            println!("Deleting testfile.csv");
            std::fs::remove_file("testfile.csv").unwrap();
        }
        let id = workitem.id.clone();

        let response = client
            .update_workitem(UpdateWorkitemRequest {
                workitem: Some(workitem),
                files: vec![
                    WorkitemFile {
                        filename: "../../train.csv".to_string(),
                        ..Default::default()
                    },
                    WorkitemFile {
                        filename: "testfile.csv".to_string(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .await;
        assert!(
            response.is_ok(),
            "UpdateWorkitem failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .delete_workitem(DeleteWorkitemRequest {
                id: id,
                ..Default::default()
            })
            .await;

        assert!(
            response.is_ok(),
            "DeleteWorkitem failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .pop_workitem(
                PopWorkitemRequest {
                    wiq: "rustqueue".to_string(),
                    ..Default::default()
                },
                Some(""),
            )
            .await;

        assert!(
            response.is_ok(),
            "PopWorkitem failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .delete_workitem(DeleteWorkitemRequest {
                id: response.unwrap().workitem.unwrap().id,
                ..Default::default()
            })
            .await;

        assert!(
            response.is_ok(),
            "DeleteWorkitem failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test] // cargo test test_custom_command -- --nocapture
    async fn test_custom_command() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .custom_command(CustomCommandRequest::bycommand("getclients"))
            .await;
        println!("CustomCommand response: {:?}", response);

        assert!(
            response.is_ok(),
            "CustomCommand failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test] // cargo test test_list_collections -- --nocapture
    async fn test_list_collections() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client.list_collections(false).await;
        println!("ListCollections response: {:?}", response);

        assert!(
            response.is_ok(),
            "ListCollections failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test] // cargo test test_create_drop_collections -- --nocapture
    async fn test_create_drop_collections() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .create_collection(CreateCollectionRequest::byname("rusttestcollection"))
            .await;
        println!("CreateCollection response: {:?}", response);

        assert!(
            response.is_ok(),
            "CreateCollection failed with {:?}",
            response.err().unwrap()
        );

        let item = "{\"name\": \"test collection\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "rusttestcollection".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .drop_collection(DropCollectionRequest::byname("rusttestcollection"))
            .await;
        println!("DropCollection response: {:?}", response);

        assert!(
            response.is_ok(),
            "DropCollection failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test] // cargo test test_create_drop_tscollections -- --nocapture
    async fn test_create_drop_tscollections() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let collections_json = client.list_collections(false).await.unwrap();
        let collections: serde_json::Value = serde_json::from_str(&collections_json).unwrap();
        let collections = collections.as_array().unwrap();
        for collection in collections {
            let collectionname = collection["name"].as_str().unwrap();
            if collectionname.starts_with("rusttesttscollection") {
                let response = client
                    .drop_collection(DropCollectionRequest::byname(collectionname))
                    .await;
                println!("DropCollection response: {:?}", response);

                assert!(
                    response.is_ok(),
                    "DropCollection failed with {:?}",
                    response.err().unwrap()
                );
            }
        }

        let mut request = CreateCollectionRequest::byname("rusttesttscollection");
        request.timeseries = Some(ColTimeseries {
            time_field: "time".to_string(),
            meta_field: "".to_string(),
            granularity: "minutes".to_string(), // seconds, minutes, hours
        });

        let response = client.create_collection(request).await;
        println!("CreateCollection response: {:?}", response);

        assert!(
            response.is_ok(),
            "CreateCollection failed with {:?}",
            response.err().unwrap()
        );

        let item = "{\"name\": \"test collection\", \"_type\": \"test\", \"time\": \"2024-08-31T07:18:01.395Z\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "rusttesttscollection".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        // let mut item: serde_json::Value = serde_json::from_str(&response.unwrap().result).unwrap();
        // // let id = item["_id"].as_str().unwrap();
        // item["name"] = serde_json::Value::String("test collection 2".to_string());
        // println!("Item: {:?}", item);
        // let query = UpdateOneRequest {
        //     collectionname: "rusttesttscollection".to_string(),
        //     item: item.to_string(),
        //     ..Default::default()
        // };
        // let response = client.update_one(query).await;
        // assert!(
        //     response.is_ok(),
        //     "test_query failed with {:?}",
        //     response.err().unwrap()
        // );

        // let response = client
        //     .drop_collection(DropCollectionRequest::byname("rusttesttscollection"))
        //     .await;
        // println!("DropCollection response: {:?}", response);

        // assert!(
        //     response.is_ok(),
        //     "DropCollection failed with {:?}",
        //     response.err().unwrap()
        // );
    }
    #[tokio::test] // cargo test test_get_create_drop_index -- --nocapture
    async fn test_get_create_drop_index() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .create_collection(CreateCollectionRequest::byname("rustindextestcollection"))
            .await;
        println!("CreateCollection response: {:?}", response);

        assert!(
            response.is_ok(),
            "CreateCollection failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .get_indexes(GetIndexesRequest::bycollectionname(
                "rustindextestcollection",
            ))
            .await;
        assert!(
            response.is_ok(),
            "GetIndexes failed with {:?}",
            response.err().unwrap()
        );
        let indexes: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
        let indexes = indexes.as_array().unwrap();
        for index in indexes {
            let indexname = index["name"].as_str().unwrap();
            if indexname.starts_with("name_1") {
                let response = client
                    .drop_index(DropIndexRequest::bycollectionname(
                        "rustindextestcollection",
                        indexname,
                    ))
                    .await;
                println!("DropIndex response: {:?}", response);

                assert!(
                    response.is_ok(),
                    "DropIndex failed with {:?}",
                    response.err().unwrap()
                );
            } else {
                println!("Index: {:?}", index);
            }
        }

        let response = client
            .create_index(CreateIndexRequest::bycollectionname(
                "rustindextestcollection",
                "{\"name\": 1}",
            ))
            .await;
        println!("CreateIndex response: {:?}", response);

        assert!(
            response.is_ok(),
            "CreateIndex failed with {:?}",
            response.err().unwrap()
        );

        let response = client
            .get_indexes(GetIndexesRequest::bycollectionname(
                "rustindextestcollection",
            ))
            .await;
        assert!(
            response.is_ok(),
            "GetIndexes failed with {:?}",
            response.err().unwrap()
        );
        let indexes: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
        let indexes = indexes.as_array().unwrap();

        let mut found = false;
        for index in indexes {
            let indexname = index["name"].as_str().unwrap();
            if indexname.starts_with("name_1") {
                found = true;
                break;
            }
        }
        assert!(found, "Index name_1 not found");
    }
    #[tokio::test()] // cargo test test_start_getpods_stop_delete_agent -- --nocapture
    async fn test_start_getpods_stop_delete_agent() {
        if TEST_URL.contains("home")  || TEST_URL.contains("") {
            println!("Skipping test_start_getpods_stop_delete_agent");
            return;
        }
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let response = client
            .query(QueryRequest {
                query: "{\"slug\": \"rusttestagent\"}".to_string(),
                collectionname: "agents".to_string(),
                ..Default::default()
            })
            .await;
        let response = match response {
            Ok(response) => {
                let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
                let items = _obj.as_array().unwrap();
                if items.len() == 0 {
                    let agent_json = "{\"name\": \"rusttestagent\", \"_type\": \"agent\", \"image\": \"openiap/nodeagent\", \"slug\": \"rusttestagent\", \"docker\": true }".to_string();
                    let query = InsertOneRequest {
                        collectionname: "agents".to_string(),
                        item: agent_json,
                        ..Default::default()
                    };
                    let response = client.insert_one(query).await;
                    assert!(
                        response.is_ok(),
                        "test_query failed with {:?}",
                        response.err().unwrap()
                    );
                    println!("Created rusttestagent");
                    response.unwrap().result
                } else {
                    println!("rusttestagent already exists");
                    let _obj = items[0].clone();
                    _obj.to_string()
                }
            }
            Err(e) => {
                assert!(false, "Query failed with {:?}", e);
                return;
            }
        };
        let _obj: serde_json::Value = serde_json::from_str(&response).unwrap();
        let id = _obj["_id"].as_str().unwrap();
        println!("Agent ID: {:?}", id);

        let mut podname = "".to_string();
        let response = client.get_agent_pods(id, false).await;
        assert!(
            response.is_ok(),
            "GetAgentPods failed with {:?}",
            response.err().unwrap()
        );
        let pods: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
        let pods = pods.as_array().unwrap();
        if pods.len() > 0 {
            for pod in pods {
                let metadata = pod["metadata"].as_object().unwrap();
                let name = metadata["name"].as_str().unwrap();
                podname = name.to_string();
                println!("Podname: {:?}", podname);
                // let json = pod.to_string();
                // println!("Pod: {:?}", json);
                break;
            }
        }
        if podname.is_empty() {
            let response = client.start_agent(id).await;
            assert!(
                response.is_ok(),
                "StartAgent failed with {:?}",
                response.err().unwrap()
            );
            println!("Started rusttestagent");
            loop {
                let response = client.get_agent_pods(id, false).await;
                assert!(
                    response.is_ok(),
                    "GetAgentPods failed with {:?}",
                    response.err().unwrap()
                );
                let pods: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
                let pods = pods.as_array().unwrap();
                if pods.len() > 0 {
                    for pod in pods {
                        let metadata = pod["metadata"].as_object().unwrap();
                        let name = metadata["name"].as_str().unwrap();
                        podname = name.to_string();
                        println!("Podname: {:?}", podname);
                        // let json = pod.to_string();
                        // println!("Pod: {:?}", json);
                        break;
                    }
                }
                if !podname.is_empty() {
                    println!("Podname: {:?}", podname);
                    break;
                }
            }
        }

        loop {
            let response = client.get_agent_pod_logs(id, &podname).await;
            let is_ok = response.is_ok();
            let message = match response {
                Ok(response) => response,
                Err(e) => e.to_string(),
            };
            if is_ok {
                println!("Logs: {:?}", message);
                if !message.is_empty() {
                    break;
                }
            } else if !message.contains("waiting") {
                assert!(is_ok, "GetAgentLogs failed with {:?}", message);
            }
        }

        let response = client.get_agent_pods(id, false).await;
        assert!(
            response.is_ok(),
            "GetAgentPods failed with {:?}",
            response.err().unwrap()
        );

        let response = client.stop_agent(id).await;
        assert!(
            response.is_ok(),
            "StopAgent failed with {:?}",
            response.err().unwrap()
        );
        println!("Stopped rusttestagent");

        let response = client.delete_agent(id).await;

        assert!(
            response.is_ok(),
            "DeleteAgent failed with {:?}",
            response.err().unwrap()
        );

        println!("Deleted rusttestagent");
    }
    #[tokio::test()] // cargo test test_ensure_customer -- --nocapture
    async fn test_ensure_customer() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let customer = Customer::byname("rusttestcustomer");
        let request = EnsureCustomerRequest::new(Some(customer), "", None);
        let response = client.ensure_customer(request).await;
        let customer = match response {
            Ok(response) => {
                let customer = match response.customer {
                    Some(customer) => customer,
                    None => {
                        assert!(false, "EnsureCustomer failed with no customer");
                        return;
                    }
                };
                customer
            }
            Err(e) => {
                assert!(false, "EnsureCustomer failed with {:?}", e);
                return;
            }
        };
        println!("Customer: {:?}", customer);
    }
    #[tokio::test()] // cargo test test_add_update_delete_workitem_queue -- --nocapture
    async fn test_add_update_delete_workitem_queue() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let signedin = client.get_state();
        let user = client.user().await.unwrap();
        println!("signed in {:?} as: {:?}", signedin, user);

        let response = client
            .query(QueryRequest {
                query: "{\"name\": \"updated rusttestqueue2\"}".to_string(),
                collectionname: "mq".to_string(),
                ..Default::default()
            })
            .await;
        match response {
            Ok(response) => {
                let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
                let items = _obj.as_array().unwrap();
                if items.len() > 0 {
                    let _obj = items[0].clone();
                    let wiqid = _obj["_id"].as_str().unwrap();
                    println!("workitemqueue id: {:?} already exists as updated rusttestqueue2, so delete it", wiqid);
                    client
                        .delete_workitem_queue(DeleteWorkItemQueueRequest {
                            wiqid: wiqid.to_string(),
                            purge: true,
                            ..Default::default()
                        })
                        .await
                        .unwrap();
                }
            }
            Err(e) => {
                assert!(false, "Query failed with {:?}", e);
                return;
            }
        }
        let response = client
            .query(QueryRequest {
                query: "{\"name\": \"rusttestqueue2\"}".to_string(),
                collectionname: "mq".to_string(),
                ..Default::default()
            })
            .await;
        let mut wiq = match response {
            Ok(response) => {
                let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
                let items = _obj.as_array().unwrap();
                if items.len() == 0 {
                    let queue = WorkItemQueue {
                        name: "rusttestqueue2".to_string(),
                        ..Default::default()
                    };
                    let request = AddWorkItemQueueRequest {
                        workitemqueue: Some(queue),
                        skiprole: true,
                    };
                    let response = client.add_workitem_queue(request).await;
                    match response {
                        Ok(response) => {
                            println!("workitem queue: {:?}", response);
                            response
                        }
                        Err(e) => {
                            assert!(false, "AddWorkItemQueue failed with {:?}", e);
                            return;
                        }
                    }
                } else {
                    println!("workitem queue already exists");
                    let _obj = items[0].clone();
                    let vacl = _obj["_acl"].as_array().unwrap();
                    let mut acl = Vec::new();
                    for ace in vacl {
                        let ace = Ace {
                            id: ace["_id"].as_str().unwrap().to_string(),
                            rights: ace["rights"].as_i64().unwrap() as i32,
                            deny: false, // deny: ace["deny"].as_bool().unwrap(),
                        };
                        acl.push(ace);
                    }
                    WorkItemQueue {
                        id: _obj["_id"].as_str().unwrap().to_string(),
                        name: _obj["name"].as_str().unwrap().to_string(),
                        acl,
                        ..Default::default()
                    }
                }
            }
            Err(e) => {
                assert!(false, "Query failed with {:?}", e);
                return;
            }
        };
        println!("workitemqueue id: {:?} name: {:?}", wiq.id, wiq.name);
        wiq.name = "updated rusttestqueue2".to_string();
        let wiqid = wiq.id.clone();

        client
            .update_workitem_queue(UpdateWorkItemQueueRequest {
                workitemqueue: Some(wiq),
                purge: false,
                skiprole: true,
            })
            .await
            .unwrap();

        client
            .delete_workitem_queue(DeleteWorkItemQueueRequest {
                wiqid,
                purge: true,
                ..Default::default()
            })
            .await
            .unwrap();
    }
    #[tokio::test()] // cargo test test_rpc -- --nocapture
    async fn test_rpc() {
        let client = Arc::new(Client::new_connect(TEST_URL).await.unwrap());
        let pingserver = client
            .register_queue(RegisterQueueRequest::byqueuename("pingserver"), {
                let client = client.clone(); // Clone the Arc to move into the closure
                Box::new(move |event| {
                    let client = client.clone(); // Clone here to move it into the spawn block
                    tokio::task::spawn(async move {
                        client
                            .queue_message(QueueMessageRequest {
                                queuename: event.replyto.clone(),
                                data: "{\"command\": \"pong\"}".to_string(),
                                striptoken: true,
                                ..Default::default()
                            })
                            .await
                            .unwrap();
                    });
                })
            })
            .await
            .unwrap();
        let response = client
            .rpc(QueueMessageRequest {
                queuename: "pingserver".to_string(),
                data: "{\"command\": \"ping\"}".to_string(),
                striptoken: true,
                ..Default::default()
            })
            .await
            .unwrap();
        println!("RPC response: {:?}", response);
        let response = client.unregister_queue(&pingserver).await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => {
                assert!(false, "UnregisterQueue failed with {:?}", e);
            }
        }
    }
    #[tokio::test()] // cargo test test_create_workflow_instance -- --nocapture
    async fn test_create_workflow_instance() {
        let client = Client::new_connect(TEST_URL).await.unwrap();

        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let workflow_consumer = client
            .register_queue(RegisterQueueRequest::byqueuename("workflow_consumer"), {
                let tx: Arc<std::sync::Mutex<Option<oneshot::Sender<()>>>> = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Workflow event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await
            .unwrap();

        let response = client
            .create_workflow_instance(CreateWorkflowInstanceRequest {
                workflowid: "66d434b753218675491931c5".to_string(),
                data: "{\"test\": \"message\"}".to_string(),
                initialrun: true,
                name: "Rust initialed workflow".to_string(),
                targetid: "5ce9422d320b9c09742c3ced".to_string(), // 6242d68a73057b27d277be88
                resultqueue: workflow_consumer.clone(),
                ..Default::default()
            })
            .await;
        assert!(
            response.is_ok(),
            "CreateWorkflowInstance failed with {:?}",
            response.err().unwrap()
        );

        // Await the workflow event
        rx.await.unwrap();
        println!("Workflow event received");

        let response = client.unregister_queue(&workflow_consumer).await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => {
                assert!(false, "UnregisterQueue failed with {:?}", e);
            }
        }
    }
    #[tokio::test()] // cargo test test_invoke_openrpa -- --nocapture
    async fn test_invoke_openrpa() {
        if TEST_URL.contains("home") || TEST_URL.contains("") {
            println!("Skipping test_invoke_openrpa");
            return;
        }
        let client = Client::new_connect(TEST_URL).await.unwrap();
        let robot = client.get_one(QueryRequest{
            collectionname: "users".to_string(),
            // query: "{\"username\": \"macuser\"}".to_string(),
            query: "{\"username\": \"allan5\"}".to_string(),
            ..Default::default()
        }).await.unwrap();
        let robotid = robot["_id"].as_str().unwrap();

        let workflow = client.get_one(QueryRequest{
            collectionname: "openrpa".to_string(),
            query: "{\"projectandname\": \"hdrobots/WhoAmI\"}".to_string(),
            ..Default::default()
        }).await.unwrap();
        let workflowid = workflow["_id"].as_str().unwrap();


        let response: std::result::Result<String, OpenIAPError> = client
            .invoke_openrpa(InvokeOpenRpaRequest {
                robotid: robotid.to_string(),
                workflowid: workflowid.to_string(),
                payload: "{\"test\": \"message\"}".to_string(),
                ..Default::default()
            })
            .await;
        assert!(
            response.is_ok(),
            "InvokeOpenRpa failed with {:?}",
            response.err().unwrap()
        );
        println!("InvokeOpenRpa response: {:?}", response.unwrap());
    }
}
