#![warn(missing_docs)]
//! Testing client for OpenIAP, will over time transition to a full management client.

use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;

use openiap_client::{self, enable_tracing, Client, RegisterExchangeRequest, RegisterQueueRequest};

use openiap_client::protos::{
    DistinctRequest, DownloadRequest, InsertOneRequest, QueryRequest, SigninRequest, UploadRequest,
    WatchEvent, WatchRequest,
};
use sysinfo::System;
use sysinfo::SystemExt;

use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};

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

/// Main function
async fn doit() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
    // Number of CPUs:
    println!("Number of available threads: {}", sys.cpus().len());


    // let pid = get_current_pid()?;
    // let sys = System::new_all();
    // println!("System name:             {:?}", sys.name());
    // println!("System kernel version:   {:?}", sys.kernel_version());
    // println!("System OS version:       {:?}", sys.os_version());
    // println!("System host name:        {:?}", sys.host_name());
    // let sys = Arc::new(Mutex::new(sys));
    // std::thread::spawn(move || {
    //     loop {
    //         {
    //             let mut file = std::fs::File::create("testfile.txt").unwrap();
    //             let _ = file.write_all("Hello world".as_bytes());
    //             file.sync_all().unwrap();
    //         }
    //         {
    //             let mut file = std::fs::File::open("testfile.txt").unwrap();
    //             let mut contents = String::new();
    //             file.read_to_string(&mut contents).unwrap();
    //         }

    //         std::thread::sleep(std::time::Duration::from_secs(5));
    //         let mut sys = sys.lock().unwrap();
    //         sys.refresh_process(pid);
    //         sys.refresh_all();
    //         if let Some(process) = sys.process(pid) {
    //             let disk_io = process.disk_usage();
    //             println!("read: {:?} write: {:?} total read: {:?} total write: {:?}", disk_io.read_bytes, disk_io.written_bytes, disk_io.total_read_bytes, disk_io.total_written_bytes);
    //         }
    //     }
    // });

    // let num_calcs = 1000000;
    let num_calcs = 100000;

    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // get info how many threads we can use and use half of them
    let iter_per_core: u64 = num_calcs / available_cores;
    let num_iters = 5000;

    enable_tracing("openiap=debug", "");
    let res = Client::connect("").await;
    let b = match res {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
            return Ok(());
        }        
    };
    let watchid = Arc::new(Mutex::new(String::new()));
    let mut input = String::from("bum");
    println!("? for help");
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
            println!("d: Download");
            println!("u: Upload train.csv");
            println!("uu: Upload assistant-linux-x86_64.AppImage");
            println!("uuu: Upload virtio-win-0.1.225.iso");
            println!("w: Watch");
            println!("uw: Unwatch");
            println!("r: Register queue");
            println!("m: Queue message");
        }
        if input.eq_ignore_ascii_case("c") || input.eq_ignore_ascii_case("cpu") {
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
        }
        if input.eq_ignore_ascii_case("q") {
            let client = b.clone();
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
                }
            });
        }
        if input.eq_ignore_ascii_case("qq") {
            let client = b.clone();
            tokio::task::spawn(async move {
                let q = client
                    .query(QueryRequest::with_query("entities", "{}"))
                    .await;
                match q {
                    Ok(response) => println!("{:?}", response.results),
                    Err(e) => println!("Failed to query: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("di") {
            let client = b.clone();
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
            tokio::task::spawn(async move {
                let request = InsertOneRequest {
                    collectionname: "entities".to_string(),
                    item: "{\"name\":\"Allan\", \"_type\":\"Allan\"}".to_string(),
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
        if input.eq_ignore_ascii_case("d") {
            let client = b.clone();
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

            tokio::task::spawn(async move {
                let s = client
                    .watch(
                        WatchRequest::new("", vec!["".to_string()]),
                        Box::new(onwatch),
                    )
                    .await;
                if let Err(e) = s {
                    println!("Failed to watch: {:?}", e);
                } else {
                    let new_watchid = s.unwrap();
                    println!("Watch created with id {}", new_watchid);
                    let watchid = watchid_clone.lock();
                    match  watchid {
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
        if input.eq_ignore_ascii_case("r") {
            let client = b.clone();
            tokio::task::spawn(async move {
                let q = client
                    .register_queue(
                        RegisterQueueRequest::byqueuename("test2queue"),
                        Box::new(|event| {
                            println!(
                                "Received message queue from {:?} with reply to {:?}: {:?}",
                                event.queuename, event.replyto, event.data
                            );
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
            tokio::task::spawn(async move {
                let q = client
                    .queue_message(openiap_client::QueueMessageRequest::byqueuename(
                        "test2queue",
                        "{\"name\":\"Allan\"}",
                        true
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
            tokio::task::spawn(async move {
                let mut count = 0;
                loop {
                    count = count.add(1);
                    let q = client
                        .queue_message(openiap_client::QueueMessageRequest::byqueuename(
                            "test2queue",
                            format!("{{\"name\":\"Allan {}\"}}", count).as_str(),
                            true
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
            tokio::task::spawn(async move {
                let q = client
                    .register_exchange(
                        RegisterExchangeRequest::byexchangename("test2exchange"),
                        Box::new(|event| {
                            println!(
                                "Received exchange message to queue  {:?} with reply to {:?}: {:?}",
                                event.queuename, event.replyto, event.data
                            );
                        }),
                    )
                    .await;
                match q {
                    Ok(response) => println!("Registered exchange as {:?}", response),
                    Err(e) => println!("Failed to register exchange: {:?}", e),
                }
            });
        }
        if input.eq_ignore_ascii_case("me") {
            let client = b.clone();
            tokio::task::spawn(async move {
                let q = client
                    .queue_message(openiap_client::QueueMessageRequest::byexchangename(
                        "test2exchange",
                        "{\"name\":\"Allan\"}",
                        true
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
