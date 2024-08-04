mod client;
use client::openiap::InsertOneRequest;
use client::Client;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
pub mod openiap {
    tonic::include_proto!("openiap");
}

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

fn onwatch(event: client::openiap::WatchEvent) {
    let document = event.document;
    let operation = event.operation;
    let document = document;
    println!("{} on {}", operation, document);
}

async fn doit() -> Result<(), Box<dyn std::error::Error>> {
    // let res = Client::connect("http://localhost:50051").await;
    // let res = Client::connect("grpc://grpc.app.openiap.io:443").await;
    // let res = Client::connect("grpc://grpc.home.openiap.io:443").await;
    // let res = Client::connect("grpc://grpc.home.openiap.io:443").await;
    let res = Client::connect("").await;
    if res.is_err() == true {
        println!("Failed to connect to server: {:?}", res.err().unwrap());
        return Ok(());
    }
    let b = res.unwrap();

    let watchid = "";

    let mut input = String::from("bum");
    while !input.eq_ignore_ascii_case("") {
        if input.eq_ignore_ascii_case("q") {
            let client = b.clone();
            tokio::task::spawn(async move {
                let q = client
                    .query(client::openiap::QueryRequest::with_projection("entities", "{}", "{\"name\":1}"))
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
                    .query(client::openiap::QueryRequest::with_query("entities", "{}"))
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
                let query = client::openiap::DistinctRequest {
                    collectionname: "entities".to_string(),
                    field: "_type".to_string(),
                    ..Default::default()
                };
                let q = client
                    .distinct(query)
                    .await;
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
                    .signin(client::openiap::SigninRequest::with_userpass(
                        "guest", "password",
                    ))
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
                    .signin(client::openiap::SigninRequest::with_userpass(
                        "testuser",
                        "badpassword",
                    ))
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
                let s = client
                    .insert_one(request)                       
                    .await;
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
                    .download(
                        client::openiap::DownloadRequest::id("65a3aaf66d52b8c15131aebd"),
                        None,
                        None,
                    )
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
                    .upload(
                        client::openiap::UploadRequest::filename("train.csv"),
                        "train.csv",
                    )
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
                        client::openiap::UploadRequest::filename("assistant-linux-x86_64.AppImage"),
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
                        client::openiap::UploadRequest::filename("virtio-win-0.1.225.iso"),
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
            tokio::task::spawn(async move {
                let s = client
                    .watch(
                        client::openiap::WatchRequest::new("", vec!["".to_string()]), 
                        Box::new(onwatch)
                )
                    .await;
                if let Err(e) = s {
                    println!("Failed to watch: {:?}", e);
                } else {
                    let watchid = s.unwrap();
                    println!("Watch created with id {}", watchid);
                }
            });
        }
        if input.eq_ignore_ascii_case("uw") {
            if !watchid.is_empty() {
                let client = b.clone();
                tokio::task::spawn(async move {
                    let watchid = "".to_string();
                    let s = client
                        .unwatch(&watchid
                    )
                        .await;
                    if let Err(e) = s {
                        println!("Failed to watch: {:?}", e);
                    } else {
                        println!("Removed watch for id {}", watchid);
                    }
                });
            } else {
                println!("No watch to unwatch");
            }
        }
        input = keyboard_input().await;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::fmt()
    // .with_max_level(tracing::Level::DEBUG)
    // .init();

    println!("Main function started.");
    doit().await.expect("Failed to run doit");
    println!("Main function completed.");
}
