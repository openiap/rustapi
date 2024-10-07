use tracing::{error, debug, trace};
use futures_util::{StreamExt};
use openiap_proto::protos::Envelope;
use prost::Message as _;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::sync::{Arc};
use tokio::sync::{Mutex};
use futures::SinkExt;
// use std::ops::AddAssign;

use crate::Client;
impl Client {
    /// Setup a websocket connection to the server
    pub async fn setup_ws(&self, strurl:& str) -> Result<(), Box<dyn std::error::Error>>
    {
        let (ws_stream, _) = connect_async(strurl).await?;
        trace!("WebSocket handshake has been successfully completed");
        let (mut write, read) = ws_stream.split();

        let envelope_receiver = self.out_envelope_receiver.clone();
        let me = self.clone();
        tokio::spawn( async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                let envelope = envelope_receiver.recv().await;
                let mut envelope = envelope.unwrap();
                let command = envelope.command.clone();
                {
                    envelope.seq = me.inc_msgcount().clone();
                    if envelope.id.is_empty() {
                        envelope.id = envelope.seq.to_string();
                    }
                }
                if envelope.rid.is_empty() {
                    debug!("Send #{} #{} {} message", envelope.seq, envelope.id, command);
                } else {
                    debug!("Send #{} #{} (reply to #{}) {} message", envelope.seq, envelope.id, envelope.rid, command);
                }
                // get envelope length, then add it as the first 4 bytes of the message
                let envelope = envelope.encode_to_vec();
                let size = envelope.len() as u32;

                // Write the size in little-endian format (like writeUInt32LE in Node.js)
                let mut size_bytes = size.to_le_bytes().to_vec(); 

                // Append the actual envelope data
                size_bytes.extend_from_slice(&envelope);

                // Now size_bytes contains the 4-byte length followed by the envelope
                let size_bytes = size_bytes;
                match write.send(Message::Binary(size_bytes)).await {
                    Ok(_) => {
                        trace!("Sent a {} message to the server", command);
                    },
                    Err(e) => {
                        error!("Failed to send {} message to websocket: {:?}", command, e);
                        me.set_connected(false, Some(&e.to_string()));
                        return;
                    }
                };
            }
        });

        let buffer: Vec<u8> = vec![];
        let buffer = Arc::new(Mutex::new(buffer));
        let me = self.clone();
        let ws_to_stdout = {
            read.for_each(move |message| {
                let buffer = Arc::clone(&buffer);
                let me = me.clone();
                async move {
                    if message.is_err() {
                        let errmsg = message.err().unwrap().to_string();
                        error!("Failed to receive message from websocket: {:?}", errmsg);
                        me.set_connected(false, Some(&errmsg));
                        return;
                    }
                    let data = message.unwrap().into_data();
                    let mut buffer = buffer.lock().await;
                    buffer.extend(&data);

                    if buffer.len() < 4 {
                        // trace!("Buffer length is less than 4");
                        return;
                    }

                    let size: u64 = ((buffer[3] as u64) << 24) +
                    ((buffer[2] as u64) << 16) +
                    ((buffer[1] as u64) << 8) +
                    (buffer[0] as u64);

                    // Make sure we have the full message (4 bytes for the size + payload)
                    if buffer.len() < (4 + size as usize) {
                        // trace!("Buffer length is less than 4 + size: {}", 4 + size as usize);
                        return; // Wait for more data
                    }

                    let payload = &buffer[4..(4 + size as usize)].to_vec();
                    buffer.drain(0..(4 + size as usize));
                    // trace!("Decoding payload: {:?}", payload);
                    let received = match Envelope::decode(&payload[..]) {
                        Ok(received) => {
                            received
                        },
                        Err(e) => {
                            error!("Failed to decode protobuf message: {:?}", e);
                            return;
                        }
                    };
                    me.parse_incomming_envelope(received).await;
                }
            })
        };
        tokio::spawn(async {
            ws_to_stdout.await;
        });
        self.set_connected(true, None);
        Ok(())
    }
}