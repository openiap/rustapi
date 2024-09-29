use futures_util::{ StreamExt};
use openiap_proto::protos::Envelope;

use prost::Message as _;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use std::sync::{Arc};
use tokio::sync::{Mutex};

use crate::Client;
impl Client {
    /// Setup a websocket connection to the server
    pub async fn setup_ws(&self, strurl:& str,
        stream_tx: tokio::sync::mpsc::Sender<Envelope>,
    ) -> Result<futures_channel::mpsc::UnboundedSender<Message>, Box<dyn std::error::Error>>
    {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(strurl).await?;
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);

        let buffer: Vec<u8> = vec![];
        let buffer = Arc::new(Mutex::new(buffer));
        let inner_stream_tx = Arc::new(Mutex::new(stream_tx.clone()));
        let me = self.clone();
        let ws_to_stdout = {
            read.for_each(move |message| {
                let buffer = Arc::clone(&buffer);
                let stream_tx: Arc<Mutex<tokio::sync::mpsc::Sender<Envelope>>> = Arc::clone(&inner_stream_tx);
                let me = me.clone();
                async move {
                    if message.is_err() {
                        let errmsg = message.err().unwrap().to_string();
                        eprintln!("Failed to receive message from websocket: {:?}", errmsg);
                        me.set_connected(false, Some(&errmsg));
                        return;
                    }
                    // println!("Received a message from the server");
                    let data = message.unwrap().into_data();
                    let mut buffer = buffer.lock().await;
                    buffer.extend(&data);

                    if buffer.len() < 4 {
                        return;
                    }

                    let size: u64 = ((buffer[3] as u64) << 24) +
                    ((buffer[2] as u64) << 16) +
                    ((buffer[1] as u64) << 8) +
                    (buffer[0] as u64);

                    // Make sure we have the full message (4 bytes for the size + payload)
                    if buffer.len() < (4 + size as usize) {
                        return; // Wait for more data
                    }

                    let payload = &buffer[4..(4 + size as usize)].to_vec();
                    buffer.drain(0..(4 + size as usize));
                    let received = match Envelope::decode(&payload[..]) {
                        Ok(received) => {
                            received
                        },
                        Err(e) => {
                            eprintln!("Failed to decode protobuf message: {:?}", e);
                            return;
                        }
                    };
                    let stream_tx = stream_tx.lock().await;
                    match stream_tx.send(received).await {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Failed to send message to stream: {:?}", e);
                        }                    
                    };
                    // stream_tx.send(received).await.unwrap();
                    // let inner = inner.lock().await;
                    // parse_incomming_envelope(inner.clone(), received).await;
                    // continue;
                    // tokio::io::stdout().write_all(&data).await.unwrap();
                }
            })
        };
        let me = self.clone();
        tokio::spawn( async move {
            let res = stdin_to_ws.await;
            let me = me.clone();
            if res.is_err() {
                let errmsg = res.err().unwrap().to_string(); 
                eprintln!("Failed to receive message from websocket: {:?}", errmsg);
                me.set_connected(false, Some(&errmsg));
                return;
            }
        });
        
        tokio::spawn(async {
            ws_to_stdout.await;
        });
        self.set_connected(true, None);
        Ok(stdin_tx)
    }
}