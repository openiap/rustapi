#![warn(missing_docs)]
//! Testing client for OpenIAP, will over time transition to a full management client.

use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;
use std::time::Duration;
use tracing::{error, info};

#[allow(unused_imports)]
use openiap_client::{
    self, disable_observable_gauge, disable_tracing, enable_tracing, set_f64_observable_gauge,
    set_i64_observable_gauge, set_otel_url, set_u64_observable_gauge, Client, InsertManyRequest,
    PopWorkitemRequest, RegisterExchangeRequest, RegisterQueueRequest, UpdateWorkitemRequest,
};
use openiap_client::{CustomCommandRequest, InvokeOpenRpaRequest, PushWorkitemRequest, QueueMessageRequest};

use openiap_client::protos::{
    DistinctRequest, DownloadRequest, InsertOneRequest, QueryRequest, SigninRequest, UploadRequest,
    WatchEvent, WatchRequest,
};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};

// #[cfg(not(target_env = "msvc"))]
// use tikv_jemallocator::Jemalloc;
// #[cfg(not(target_env = "msvc"))]
// use tikv_jemalloc_ctl::{profiling, stats};

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

/// Reads a line from the keyboard input.
pub async fn keyboard_input() -> String {
    println!("Enter your message: ");
    let mut inp = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader
        .read_line(&mut inp)
        .await
        .expect("Failed to read line");
    inp.trim().to_string()
}

/// Watch event handler
fn onwatch(event: WatchEvent) {
    let document = event.document;
    let operation = event.operation;
    println!("{} on {}", operation, document);
}
/// Do some calculation, to geneerate some CPU load
pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}
/// Do some calculation, to geneerate some CPU load
fn add_one_loop(&n_loops: &u64) {
    for _in in 0..n_loops {
        let _ = factorial(20);
    }
}
// use memory_stats::memory_stats;
/// Main function
async fn doit() -> Result<(), Box<dyn std::error::Error>> {
    let num_calcs = 100000;

    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // get info how many threads we can use and use half of them
    let iter_per_core: u64 = num_calcs / available_cores;
    let num_iters = 5000;

    let b = Client::new();
    // enable_tracing("openiap=debug", "new");
    enable_tracing("openiap=info", "");
    b.on_event(Box::new(|_event| {
        match _event {
            openiap_client::ClientEvent::Connecting => println!("CLI: Client connecting!"),
            openiap_client::ClientEvent::Connected => println!("CLI: Client connected!"),
            openiap_client::ClientEvent::Disconnected(e) => {
                println!("CLI: Client disconnected! {:?}", e)
            }
            openiap_client::ClientEvent::SignedIn => println!("CLI: Client signed in!"),
            // openiap_client::ClientEvent::SignedOut => println!("CLI: Client signed out!"),
        }
    }))
    .await;
    let res = b.connect_async("").await;
    // let res = b.connect("");
    // let res = Client::new_connect("").await;
    match res {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
            // return Err(e.to_string().into());
            return Ok(());
        }
    };
    // Test disconnect/connect
    // b.disconnect();
    // let res = b.connect_async("").await;
    // match res {
    //     Ok(b) => b,
    //     Err(e) => {
    //         println!("Failed to connect to server: {:?}", e);
    //         return Ok(());
    //     }
    // };
    let user = b.get_user();
    match user {
        Some(user) => {
            println!("signed in as: {:} {:}", user.name, user.username);
        }
        None => {
            println!("not signed in, or signed in as guest");
        }
    }

    let watchid = Arc::new(Mutex::new(String::new()));
    let mut input = String::from("bum");
    println!("? for help");
    let mut sthandle: Option<tokio::task::JoinHandle<()>> = None;
    let mut f64handle: Option<tokio::task::JoinHandle<()>> = None;
    let mut u64handle: Option<tokio::task::JoinHandle<()>> = None;
    let mut i64handle: Option<tokio::task::JoinHandle<()>> = None;
    let mut x: u64 = 0;
    while !input.eq_ignore_ascii_case("quit") {
        if input.eq_ignore_ascii_case("?") {
            println!("? for help");
            println!("quit: to quit");
            println!("q: Query");
            println!("qq: Query all");
            println!("di: Distinct");
            println!("s: Sign in as guest");
            println!("s2: Sign in as testuser");
            println!("i: Insert");
            println!("im: Insert Many");
            println!("d: Download");
            println!("u: Upload train.csv");
            println!("uu: Upload assistant-linux-x86_64.AppImage");
            println!("uuu: Upload virtio-win-0.1.225.iso");
            println!("w: Watch");
            println!("uw: Unwatch");
            println!("r: Register queue");
            println!("m: Queue message");
        }
        // if input.eq_ignore_ascii_case("mon") {
        //     let prof = profiling::prof::read().unwrap();
        //     println!("is memory profiling enabled: {}", prof);
        //     let prof_leak = profiling::prof_leak::read().unwrap();
        //     println!("is leak reporting enabled: {}", prof_leak);
        //     let prof = stats::active::read().unwrap();
        //     println!("is active: {}", prof);
        //     let prof = profiling::prof_final::read().unwrap();
        //     println!("is final: {}", prof);
        //     let prof_final = profiling::prof_final::read().unwrap();
        //     println!("dump final memory usage to file: {}", prof_final);
        // }
        if input.eq_ignore_ascii_case("0") {
            disable_tracing();
        }
        if input.eq_ignore_ascii_case("1") {
            enable_tracing("openiap=info", "");
        }
        if input.eq_ignore_ascii_case("2") {
            enable_tracing("openiap=debug", "new");
        }
        if input.eq_ignore_ascii_case("3") {
            enable_tracing("openiap=trace", "new");
        }
        if input.eq_ignore_ascii_case("4") {
            enable_tracing("trace", "new");
        }
        if input.eq_ignore_ascii_case("o") {
            input = "".to_string();
            if f64handle.is_some() {
                disable_observable_gauge("test_f64");
                info!("Disabled custom metric: test_f64");
                f64handle.unwrap().abort();
                f64handle = None;
            } else {
                f64handle = Some(tokio::task::spawn(async move {
                    info!("Task started, begin loop for test_f64");
                    let randomf: f64 = rand::random::<f64>() * 50.0;
                    match set_f64_observable_gauge(
                        "test_f64",
                        randomf,
                        "My custom metric description",
                    ) {
                        Ok(_) => info!("test_f64 created with inital value: {}", randomf),
                        Err(e) => error!("Failed to register custom metric: {}", e),
                    }
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        let randomf: f64 = rand::random::<f64>() * 50.0;
                        match set_f64_observable_gauge(
                            "test_f64",
                            randomf,
                            "My custom metric description",
                        ) {
                            Ok(_) => info!("test_f64 set to: {}", randomf),
                            Err(e) => error!("Failed to register custom metric: {}", e),
                        }
                    }
                }));
            }
        }
        if input.eq_ignore_ascii_case("o2") {
            input = "".to_string();
            if u64handle.is_some() {
                disable_observable_gauge("test_u64");
                info!("Disabled custom metric: test_u64");
                u64handle.unwrap().abort();
                u64handle = None;
            } else {
                u64handle = Some(tokio::task::spawn(async move {
                    info!("Task started, begin loop for test_u64");
                    let randomf: u64 = (rand::random::<f64>() * 50.0).round() as u64;
                    match set_u64_observable_gauge(
                        "test_u64",
                        randomf,
                        "My custom metric description",
                    ) {
                        Ok(_) => info!("test_u64 created with inital value: {}", randomf),
                        Err(e) => error!("Failed to register custom metric: {}", e),
                    }
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        let randomf: u64 = (rand::random::<f64>() * 50.0).round() as u64;
                        match set_u64_observable_gauge(
                            "test_u64",
                            randomf,
                            "My custom metric description",
                        ) {
                            Ok(_) => info!("test_u64 set to: {}", randomf),
                            Err(e) => error!("Failed to register custom metric: {}", e),
                        }
                    }
                }));
            }
        }
        if input.eq_ignore_ascii_case("o3") {
            input = "".to_string();
            if i64handle.is_some() {
                disable_observable_gauge("test_i64");
                info!("Disabled custom metric: test_i64");
                i64handle.unwrap().abort();
                i64handle = None;
            } else {
                i64handle = Some(tokio::task::spawn(async move {
                    info!("Task started, begin loop for test_i64");
                    let randomf: i64 = (rand::random::<f64>() * 50.0).round() as i64;
                    match set_i64_observable_gauge(
                        "test_i64",
                        randomf,
                        "My custom metric description",
                    ) {
                        Ok(_) => info!("test_i64 created with inital value: {}", randomf),
                        Err(e) => error!("Failed to register custom metric: {}", e),
                    }
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        let randomf: i64 = (rand::random::<f64>() * 50.0).round() as i64;
                        match set_i64_observable_gauge(
                            "test_i64",
                            randomf,
                            "My custom metric description",
                        ) {
                            Ok(_) => info!("test_i64 set to: {}", randomf),
                            Err(e) => error!("Failed to register custom metric: {}", e),
                        }
                    }
                }));
            }
        }
        if input.eq_ignore_ascii_case("st") {
            // || input.eq_ignore_ascii_case("bum")
            input = "".to_string();
            let client = b.clone();
            if sthandle.is_some() {
                println!("Stopping nonstop");
                sthandle.unwrap().abort();
                sthandle = None;
            } else {
                sthandle = Some(
                    // tokio::task::Builder::new().name("NonStop").spawn(async move {
                    tokio::task::spawn(async move {
                        println!("Task started, begin loop...");
                        loop {
                            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                            match client
                                .pop_workitem(PopWorkitemRequest::bywiq("q2"), None)
                                .await
                            {
                                Ok(response) => match response.workitem {
                                    Some(mut workitem) => {
                                        let name = workitem.name.clone();
                                        let id = workitem.id.clone();
                                        println!("popped workitem {:?} {:?}", id, name);
                                        workitem.state = "successful".to_string();
                                        match client
                                            .update_workitem(UpdateWorkitemRequest {
                                                workitem: Some(workitem),
                                                ignoremaxretries: false,
                                                ..Default::default()
                                            })
                                            .await
                                        {
                                            Ok(_response) => {
                                                x = x + 1;
                                                if x % 500 == 0 {
                                                    println!(
                                                        "Updated workitem {:?} {:?}",
                                                        id, name
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                x = x + 1;
                                                if x % 500 == 0 {
                                                    println!("Failed to update workitem: {:?}", e);
                                                }
                                            }
                                        };
                                    }
                                    None => {
                                        let a = std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs();
                                        x = x + 1;
                                        if x % 500 == 0 {
                                            println!("No workitem popped {:?}", a);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to pop workitem: {:?}", e);
                                }
                            }
                        }
                    }),
                );
            }
        }
        if input.eq_ignore_ascii_case("st2") {
            // || input.eq_ignore_ascii_case("bum")
            input = "".to_string();
            let client = b.clone();
            if sthandle.is_some() {
                println!("Stopping nonstop");
                sthandle.unwrap().abort();
                sthandle = None;
            } else {
                sthandle = Some(
                    // tokio::task::Builder::new().name("NonStop").spawn(async move {
                    tokio::task::spawn(async move {
                        println!("Task started, begin loop...");
                        loop {
                            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                            match client
                                .push_workitem(PushWorkitemRequest {
                                    wiq: "rustqueue".to_string(),
                                    name: "test".to_string(),
                                    payload: "{}".to_string(),
                                    ..Default::default()
                                })
                                .await
                            {
                                Ok(response) => {
                                    match response.workitem {
                                        Some(_workitem) => {
                                            // let name = _workitem.name.clone();
                                            // let id = _workitem.id.clone();
                                            // println!("pushed workitem {:?} {:?}", id, name);
                                        }
                                        None => {
                                            let a = std::time::SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .unwrap()
                                                .as_secs();
                                            println!("No workitem pushed {:?}", a);
                                        }
                                    };
                                }
                                Err(e) => {
                                    println!("Failed to push workitem: {:?}", e);
                                }
                            }
                            match client
                                .pop_workitem(PopWorkitemRequest::bywiq("rustqueue"), None)
                                .await
                            {
                                Ok(response) => {
                                    match response.workitem {
                                        Some(mut workitem) => {
                                            let name = workitem.name.clone();
                                            let id = workitem.id.clone();
                                            // println!("popped workitem {:?} {:?}", id, name);
                                            workitem.state = "successful".to_string();
                                            match client
                                                .update_workitem(UpdateWorkitemRequest {
                                                    workitem: Some(workitem),
                                                    ignoremaxretries: false,
                                                    ..Default::default()
                                                })
                                                .await
                                            {
                                                Ok(_response) => {
                                                    x = x + 1;
                                                    if x % 500 == 0 {
                                                        println!(
                                                            "Updated workitem {:?} {:?}",
                                                            id, name
                                                        );
                                                    }
                                                }
                                                Err(e) => {
                                                    x = x + 1;
                                                    if x % 500 == 0 {
                                                        println!(
                                                            "Failed to update workitem: {:?}",
                                                            e
                                                        );
                                                    }
                                                }
                                            };
                                        }
                                        None => {
                                            let a = std::time::SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .unwrap()
                                                .as_secs();
                                            x = x + 1;
                                            if x % 500 == 0 {
                                                println!("No workitem popped {:?}", a);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to pop workitem: {:?}", e);
                                }
                            }
                        }
                    }),
                );
            }
        }
        if input.eq_ignore_ascii_case("st3") {
            input = "".to_string();
            let client = b.clone();
            let num_workers = 3; // Number of concurrent RPCs
            if sthandle.is_some() {
                println!("Stopping nonstop");
                sthandle.unwrap().abort();
                sthandle = None;
            } else {
                sthandle = Some(tokio::task::spawn(async move {
                    println!("Starting {} concurrent RPC workers...", num_workers);
                    let mut handles = Vec::new();
                    for _ in 0..num_workers {
                        let client = client.clone();
                        handles.push(tokio::spawn(async move {
                            let mut x: u64 = 0;
                            loop {
                                tokio::time::sleep(tokio::time::Duration::from_millis(40)).await;
                                let starttime = std::time::SystemTime::now();
                                let result = client
                                    .rpc(
                                        QueueMessageRequest::byqueuename(
                                            "test2queue",
                                            "{\"name\":\"Allan\"}",
                                            true,
                                        ),
                                        tokio::time::Duration::from_secs(1),
                                    )
                                    .await;
                                let endtime = std::time::SystemTime::now();
                                let ms = endtime
                                    .duration_since(starttime)
                                    .unwrap()
                                    .as_millis();
                                match result {
                                    Ok(response) => println!("Worker {:?}: Received RPC response {:?} in {:?}", tokio::task::id(), response, ms),
                                    Err(e) => println!("Worker {:?}: Failed to send RPC message: {:?} in {:?}", tokio::task::id(), e, ms),
                                }
                                x += 1;
                                if x % 500 == 0 {
                                    println!("Worker {:?}: RPC messages sent {:?}", tokio::task::id(), x);
                                }
                            }
                        }));
                    }
                    // Optionally: await all handles if you want to join them (not needed for infinite loop)
                    futures::future::join_all(handles).await;
                }));
            }
        }
        if input.eq_ignore_ascii_case("dis") {
            b.disconnect();
        }
        if input.eq_ignore_ascii_case("c") || input.eq_ignore_ascii_case("cpu") {
            tokio::task::spawn(async move {
                println!("Calculating factorial of 20 {} times", num_calcs);
                // std::thread::spawn(move || add_one_loop(&iter_per_core));
                for _i in 0..num_iters {
                    // let mut results = Vec::new();
                    let mut threads = Vec::new();
                    for _i in 0..available_cores {
                        threads.push(std::thread::spawn(move || add_one_loop(&iter_per_core)));
                    }
                    // for thread in threads {
                    //     results.extend(thread.join());
                    // }
                }
            });
        }
        if input.eq_ignore_ascii_case("q") {
            let client = b.clone();
            // tokio::task::Builder::new().name("projected query").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .query(QueryRequest::with_projection(
                        "entities",
                        "{}",
                        "{\"name\":1}",
                    ))
                    .await;
                match q {
                    Ok(response) => println!("{:?}", response.results),
                    Err(e) => println!("Failed to query: {:?}", e),
                };
            });
        }
        if input.eq_ignore_ascii_case("qq") {
            let client = b.clone();
            // tokio::task::Builder::new().name("query").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .query(QueryRequest::with_query("entities", "{}"))
                    .await;
                match q {
                    Ok(response) => println!("{:?}", response.results),
                    Err(e) => println!("Failed to query: {:?}", e),
                };
            });
        }
        if input.eq_ignore_ascii_case("di") {
            let client = b.clone();
            // tokio::task::Builder::new().name("distinct").spawn(async move {
            tokio::task::spawn(async move {
                let query = DistinctRequest {
                    collectionname: "entities".to_string(),
                    field: "_type".to_string(),
                    ..Default::default()
                };
                let q = client.distinct(query).await;
                match q {
                    Ok(response) => println!("{:?}", response.results),
                    Err(e) => println!("Failed to query: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("s") || input.eq_ignore_ascii_case("s1") {
            let client = b.clone();
            // tokio::task::Builder::new().name("guest signin").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .signin(SigninRequest::with_userpass("guest", "password"))
                    .await;
                if let Err(e) = s {
                    println!("Failed to sign in: {:?}", e);
                } else {
                    println!(
                        "Signed in as {}",
                        s.unwrap().user.as_ref().unwrap().username
                    );
                }
            });
        }
        if input.eq_ignore_ascii_case("s2") || input.eq_ignore_ascii_case("ss") {
            let client = b.clone();
            // tokio::task::Builder::new().name("testuser signin").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .signin(SigninRequest::with_userpass("testuser", "badpassword"))
                    .await;
                if let Err(e) = s {
                    println!("Failed to sign in: {:?}", e);
                } else {
                    println!(
                        "Signed in as {}",
                        s.unwrap().user.as_ref().unwrap().username
                    );
                }
            });
        }
        if input.eq_ignore_ascii_case("i") {
            let client = b.clone();
            //tokio::task::Builder::new().name("insertone").spawn(async move {
            tokio::task::spawn(async move {
                let request = InsertOneRequest {
                    collectionname: "entities".to_string(),
                    item: "{\"name\":\"Allan\", \"_type\":\"test\"}".to_string(),
                    ..Default::default()
                };
                let s = client.insert_one(request).await;
                if let Err(e) = s {
                    println!("Failed to insert: {:?}", e);
                } else {
                    println!("inserted as {}", s.unwrap().result);
                }
            });
        }
        if input.eq_ignore_ascii_case("im") {
            let client = b.clone();
            // tokio::task::Builder::new().name("insertmany").spawn(async move {
            tokio::task::spawn(async move {
                let request = InsertManyRequest {
                    collectionname: "entities".to_string(),
                    items: "[{\"name\":\"Allan\", \"_type\":\"test\"}, {\"name\":\"Allan2\", \"_type\":\"test\"}]".to_string(),
                    ..Default::default()
                };
                let s = client.insert_many(request).await;
                if let Err(e) = s {
                    println!("Failed to insert: {:?}", e);
                } else {
                    println!("inserted as {}", s.unwrap().results);
                }
            });
        }
        if input.eq_ignore_ascii_case("d") {
            let client = b.clone();
            // tokio::task::Builder::new().name("download").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
                    .await;
                if let Err(e) = s {
                    println!("Failed to download: {:?}", e);
                } else {
                    println!("downloaded as {}", s.unwrap().filename);
                }
            });
        }
        if input.eq_ignore_ascii_case("u") || input.eq_ignore_ascii_case("u1") {
            let client = b.clone();
            // tokio::task::Builder::new().name("upload").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .upload(UploadRequest::filename("train.csv"), "train.csv")
                    .await;
                if let Err(e) = s {
                    println!("Failed to upload: {:?}", e);
                } else {
                    println!("uploaded as {}", s.unwrap().filename);
                }
            });
        }
        if input.eq_ignore_ascii_case("uu") || input.eq_ignore_ascii_case("u2") {
            let client = b.clone();
            // tokio::task::Builder::new().name("upload2").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .upload(
                        UploadRequest::filename("assistant-linux-x86_64.AppImage"),
                        "/home/allan/Downloads/assistant-linux-x86_64.AppImage",
                    )
                    .await;
                if let Err(e) = s {
                    println!("Failed to upload: {:?}", e);
                } else {
                    println!("uploaded as {}", s.unwrap().filename);
                }
            });
        }
        if input.eq_ignore_ascii_case("uuu") || input.eq_ignore_ascii_case("u3") {
            let client = b.clone();
            // tokio::task::Builder::new().name("uppload3").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .upload(
                        UploadRequest::filename("virtio-win-0.1.225.iso"),
                        "/home/allan/Downloads/virtio-win-0.1.225.iso",
                    )
                    .await;
                if let Err(e) = s {
                    println!("Failed to upload: {:?}", e);
                } else {
                    println!("uploaded as {}", s.unwrap().filename);
                }
            });
        }
        if input.eq_ignore_ascii_case("w") {
            let client = b.clone();
            let watchid_clone = Arc::clone(&watchid);
            // tokio::task::Builder::new().name("watch").spawn(async move {
            tokio::task::spawn(async move {
                let s = client
                    .watch(
                        WatchRequest::new("entities", vec!["".to_string()]),
                        Box::new(onwatch),
                    )
                    .await;
                if let Err(e) = s {
                    println!("Failed to watch: {:?}", e);
                } else {
                    let new_watchid = s.unwrap();
                    println!("Watch created with id {}", new_watchid);
                    let watchid = watchid_clone.lock();
                    match watchid {
                        Ok(mut watchid) => {
                            *watchid = new_watchid.to_string();
                        }
                        Err(e) => {
                            println!("Failed to lock watchid: {:?}", e);
                        }
                    }
                }
            });
        }
        if input.eq_ignore_ascii_case("uw") {
            let watchid = watchid.lock();
            match watchid {
                Ok(w) => {
                    if w.is_empty() {
                        println!("No watch to unwatch");
                    } else {
                        let client = b.clone();
                        let uw = w.to_string();
                        // tokio::task::Builder::new().name("unwatch").spawn(async move {
                        tokio::task::spawn(async move {
                            let s = client.unwatch(&uw).await;
                            if let Err(e) = s {
                                println!("Failed to watch: {:?}", e);
                            } else {
                                println!("Removed watch for id {}", uw);
                            }
                        });
                    }
                }
                Err(e) => {
                    println!("Failed to lock watchid: {:?}", e);
                }
            }
        }
        if input.eq_ignore_ascii_case("r2") {
            let client = b.clone();
            tokio::task::spawn(async move {
                let result = client
                    .rpc(QueueMessageRequest::byqueuename(
                        "test2queue",
                        "{\"name\":\"Allan\"}",
                        true,
                    ), tokio::time::Duration::from_secs(1))
                    .await;
                match result {
                    Ok(response) => println!("Received RPC response {:?}", response),
                    Err(e) => println!("Failed to send RPC message: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("r") {
            let client = b.clone();
            // tokio::task::Builder::new().name("registerqueue").spawn(async move {
            tokio::task::spawn(async move {
                let q: Result<String, openiap_client::OpenIAPError> = client
                    .register_queue(
                        RegisterQueueRequest::byqueuename("test2queue"),
                        std::sync::Arc::new(|_client, event| {
                            println!(
                                "Received message queue from {:?} with reply to {:?}: {:?}",
                                event.queuename, event.replyto, event.data
                            );
                            // let replyto = event.replyto.clone();
                            // let correlation_id = event.correlation_id.clone();
                            // let client = _client.clone(); // This requires `client` to implement Clone.
                            // tokio::spawn(async move {
                            //     let result = client
                            //         .queue_message(openiap_client::QueueMessageRequest::replyto(
                            //             &replyto,
                            //             &correlation_id,
                            //             "{\"payload\":\"Bettina\"}",
                            //             true,
                            //         ))
                            //         .await;
                            //     match result {
                            //         Ok(_response) => (),
                            //         Err(e) => println!("Failed to queue message: {:?}", e),
                            //     }
                            // });
                            // None
                            Box::pin(async { Some("{\"payload\":\"Bettina\"}".to_string()) })
                        }),
                    )
                    .await;
                match q {
                    Ok(response) => println!("Registered queue as {:?}", response),
                    Err(e) => println!("Failed to register queue: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("m") {
            let client = b.clone();
            // tokio::task::Builder::new().name("queue message").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .queue_message(openiap_client::QueueMessageRequest::byqueuename(
                        "test2queue",
                        "{\"name\":\"Allan\"}",
                        true,
                    ))
                    .await;
                match q {
                    Ok(response) => println!(
                        "Queued message to {:?} with reply to {:?}",
                        response.queuename, response.replyto
                    ),
                    Err(e) => println!("Failed to queue message: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("m20") {
            let client = b.clone();
            // tokio::task::Builder::new().name("queue 20 messages").spawn(async move {
            tokio::task::spawn(async move {
                let mut count = 0;
                loop {
                    count = count.add(1);
                    let q = client
                        .queue_message(openiap_client::QueueMessageRequest::byqueuename(
                            "test2queue",
                            format!("{{\"name\":\"Allan {}\"}}", count).as_str(),
                            true,
                        ))
                        .await;
                    match q {
                        Ok(response) => println!(
                            "Queued message to {:?} with reply to {:?}",
                            response.queuename, response.replyto
                        ),
                        Err(e) => println!("Failed to queue message: {:?}", e),
                    }
                    if count >= 20 {
                        break;
                    }
                }
            });
        }
        if input.eq_ignore_ascii_case("re") {
            let client = b.clone();
            // tokio::task::Builder::new().name("register exchance").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .register_exchange(
                        RegisterExchangeRequest::byexchangename("test2exchange"),
                        Arc::new(|_client, event| {
                            println!(
                                "Received exchange message to queue  {:?} with reply to {:?}: {:?}",
                                event.queuename, event.replyto, event.data
                            );
                            Box::pin(async { None })
                        }),
                    )
                    .await;
                match q {
                    Ok(response) => println!("Registered exchange as {:?}", response),
                    Err(e) => println!("Failed to register exchange: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("cc") {
            let client = b.clone();
            //tokio::task::Builder::new().name("queue message to exchance").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .custom_command(CustomCommandRequest {
                        command: "getclients".to_string(),
                        data: "".to_string(),
                        id: "".to_string(),
                        name: "".to_string(),
                    }, None)
                    .await;
                match q {
                    Ok(response) => println!("Result: {:?}", response),
                    Err(e) => println!("Failed to run Custom Command: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("rpa") {
            let config = InvokeOpenRpaRequest {
                robotid: "5ce94386320b9ce0bc2c3d07".to_string(),
                workflowid: "5e0b52194f910e30ce9e3e49".to_string(),
                payload: "{\"test\":\"test\"}".to_string(),
                rpc: true,
            };
            let result = b.invoke_openrpa(config, Some(Duration::from_secs(10))).await;
            match result {
                Ok(response) => println!("Result: {:?}", response),
                Err(e) => println!("Failed to run Custom Command: {:?}", e),
            }
        }
        if input.eq_ignore_ascii_case("me") {
            let client = b.clone();
            //tokio::task::Builder::new().name("queue message to exchance").spawn(async move {
            tokio::task::spawn(async move {
                let q = client
                    .queue_message(openiap_client::QueueMessageRequest::byexchangename(
                        "test2exchange",
                        "{\"name\":\"Allan\"}",
                        true,
                    ))
                    .await;
                match q {
                    Ok(response) => println!(
                        "Queued message to {:?} with reply to {:?}",
                        response.exchangename, response.replyto
                    ),
                    Err(e) => println!("Failed to queue message: {:?}", e),
                }
            });
        }

        input = keyboard_input().await;
    }
    Ok(())
}

/// Main function
#[tokio::main]
async fn main() {
    println!("Main function started.");
    doit().await.expect("Failed to run doit");
    println!("Main function completed.");
}
