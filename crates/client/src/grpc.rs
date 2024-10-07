// use std::ops::AddAssign;
use tracing::{trace, error, debug};

use openiap_proto::{errors::OpenIAPError};
use tonic::Request;
use tokio_stream::{wrappers::ReceiverStream};
use futures::{StreamExt };
use crate::{Client, ClientEnum};
use tokio::sync::{mpsc};
impl Client {
    /// internal function, used to setup gRPC stream used for communication with the server.
    /// This function is called by [connect] and should not be called directly.
    /// It will "pre" process stream, watch and queue events, and call future promises, when a response is received.
    #[tracing::instrument(skip_all)]
    pub async fn setup_grpc_stream(&self) -> Result<(), OpenIAPError> {
        let mut client = match self.get_client() {
            ClientEnum::Grpc(ref client) => client.clone(),
            _ => {
                return Err(OpenIAPError::ClientError("Invalid client".to_string()));
            }
        };
        let (_new_stream_tx, stream_rx) = mpsc::channel(60);
        let in_stream = ReceiverStream::new(stream_rx);

        let response = client.setup_stream(Request::new(in_stream)).await;
        let response = match response {
            Ok(response) => response,
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to setup stream: {}",
                    e
                )));
            }
        };

        let envelope_receiver = self.out_envelope_receiver.clone();
        let me = self.clone();
        tokio::spawn( async move {
            // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                // interval.tick().await;
                let envelope = envelope_receiver.recv().await;
                let mut envelope = envelope.unwrap();
                envelope.seq = me.inc_msgcount().clone();
                if envelope.id.is_empty() {
                    envelope.id = envelope.seq.to_string();
                }
                let command = envelope.command.clone();
                if envelope.rid.is_empty() {
                    debug!("Send #{} #{} {} message", envelope.seq, envelope.id, command);
                } else {
                    debug!("Send #{} #{} (reply to #{}) {} message", envelope.seq, envelope.id, envelope.rid, command);
                }

                // trace!("Begin sending a {} message to the server", command);
                match _new_stream_tx.send(envelope).await {
                    Ok(_) => {
                        trace!("Successfully sent a {} message to the server", command);
                    },
                    Err(e) => {
                        error!("Failed to send message to gRPC stream: {:?}", e);
                        me.set_connected(false, Some(&e.to_string()));
                        return;
                    }
                };
            }
        });


        // let envelope_receiver = self.out_envelope_receiver.clone();
        // let me = self.clone();
        // tokio::spawn( async move {
        //     // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        //     loop {
        //         // interval.tick().await;
        //         let envelope = envelope_receiver.recv().await;
        //         println!("Received envelope {:?}", envelope);
        //         let mut envelope = envelope.unwrap();
        //         // let command = envelope.command.clone();
        //         {
        //             envelope.seq = me.inc_msgcount().clone();
        //             if envelope.id.is_empty() {
        //                 envelope.id = envelope.seq.to_string();
        //             }
        //         }
        //     }
        // });
        let mut resp_stream = response.into_inner();
        let me = self.clone();
        tokio::spawn(async move {
            while let Some(received) = resp_stream.next().await {
                if let Ok(received) = received {
                    me.parse_incomming_envelope(received).await;
                }
            }
        });
        self.set_connected(true, None);
        Ok(())
    }
}