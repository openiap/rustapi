use tracing::{error, debug, trace};
use futures_util::{StreamExt};
use openiap_proto::protos::Envelope;
use prost::Message as _;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::sync::Arc;
use tokio::sync::{Mutex};
use futures::SinkExt;
use bytes::{BytesMut, BufMut}; // Correct import for BufMut

use crate::Client;

impl Client {
    /// Setup a websocket connection to the server
    pub async fn setup_ws(&self, strurl: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(strurl).await?;
        trace!("WebSocket handshake has been successfully completed");
        let (mut write, mut read) = ws_stream.split();

        let envelope_receiver = self.out_envelope_receiver.clone();
        let me = self.clone();
        
        // Spawn sending task
        tokio::spawn(async move {
            while let Ok(envelope) = envelope_receiver.recv().await {
                let mut envelope = envelope;
                let command = envelope.command.clone();
                
                envelope.seq = me.inc_msgcount();
                if envelope.id.is_empty() {
                    envelope.id = envelope.seq.to_string();
                }

                if envelope.rid.is_empty() {
                    debug!("Send #{} #{} {} message", envelope.seq, envelope.id, command);
                } else {
                    debug!("Send #{} #{} (reply to #{}) {} message", envelope.seq, envelope.id, envelope.rid, command);
                }

                // Encode envelope and prepend length in little-endian
                let mut message = BytesMut::with_capacity(4 + envelope.encoded_len());
                message.put_u32_le(envelope.encoded_len() as u32);
                envelope.encode(&mut message).unwrap();

                // Send the message
                if let Err(e) = write.send(Message::Binary(message.to_vec())).await {
                    error!("Failed to send {} message to websocket: {:?}", command, e);
                    me.set_connected(false, Some(&e.to_string()));
                    return;
                }
            }
        });

        let buffer = Arc::new(Mutex::new(BytesMut::with_capacity(4096))); // Pre-allocate buffer size
        let me = self.clone();

        // Reading task with backpressure handling
        tokio::spawn({
            let buffer = Arc::clone(&buffer);
            async move {
                while let Some(message) = read.next().await {
                    let data = match message {
                        Ok(msg) => msg.into_data(),
                        Err(e) => {
                            error!("Failed to receive message from websocket: {:?}", e);
                            me.set_connected(false, Some(&e.to_string()));
                            return;
                        }
                    };

                    let mut buffer = buffer.lock().await;
                    buffer.extend_from_slice(&data);

                    while buffer.len() >= 4 {
                        let size = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;

                        if buffer.len() < 4 + size {
                            break; // Wait for more data
                        }

                        let payload = buffer.split_to(4 + size);
                        let payload = &payload[4..]; // Skip the size bytes

                        match Envelope::decode(payload) {
                            Ok(received) => {
                                me.parse_incomming_envelope(received).await;
                            },
                            Err(e) => {
                                error!("Failed to decode protobuf message: {:?}", e);
                            }
                        }
                    }
                }
            }
        });

        self.set_connected(true, None);
        Ok(())
    }
}
