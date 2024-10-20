// use std::ops::AddAssign;
use tracing::{trace, error, debug};

use openiap_proto::{errors::OpenIAPError};
use tonic::Request;
use tokio_stream::{wrappers::ReceiverStream};
use futures::{StreamExt };
use crate::{Client, ClientEnum, ClientState};
use tokio::sync::{mpsc};
use tokio::time::{timeout, Duration};
use tonic::transport::Channel;
pub use openiap_proto::*;
pub use protos::flow_service_client::FlowServiceClient;
impl Client {
    /// Connect to the server using gRPC protocol.
    pub async fn connect_grpc(url: String) -> Result<FlowServiceClient<Channel>, Box<dyn std::error::Error>> {
        // let response = FlowServiceClient::connect(strurl.clone()).await;
        // let channel = Channel::from_shared(url)?
        //     .keep_alive_timeout(Duration::from_secs(10))
        //     .keep_alive_while_idle(true)
        //     // .timeout(Duration::from_secs(60 * 24 * 60))
        //     .http2_keep_alive_interval(Duration::from_secs(10))
        //     .connect_timeout(Duration::from_secs(60))
        //     .connect()
        //     .await?;
        // let client = FlowServiceClient::new(channel);
        let client = FlowServiceClient::connect(url).await?;
        Ok(client)
    }
    /// internal function, used to setup gRPC stream used for communication with the server.
    /// This function is called by [connect] and should not be called directly.
    /// It will "pre" process stream, watch and queue events, and call future promises, when a response is received.
    #[tracing::instrument(skip_all)]
    pub async fn setup_grpc_stream(&self) -> Result<(), OpenIAPError> {
        self.set_connected(ClientState::Connecting, None);
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

        self.set_msgcount(-1); // Reset message count

        let envelope_receiver = self.out_envelope_receiver.clone();
        let me = self.clone();
        // let sender = tokio::task::Builder::new().name("GRPC envelope sender").spawn(async move {
        let sender = tokio::task::spawn(async move {
            loop {
                let envelope = envelope_receiver.recv().await;
                let mut envelope = match envelope {
                    Ok(envelope) => envelope,
                    Err(e) => {
                        error!("Failed to receive message from envelope receiver: {:?}", e);
                        me.set_connected(ClientState::Disconnected, Some(&e.to_string()));
                        return;
                    }
                };
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
                        me.set_connected(ClientState::Disconnected, Some(&e.to_string()));
                        return;
                    }
                };
            }
        }); // .map_err(|e| OpenIAPError::ClientError(format!("Failed to spawn GRPC envelope sender task: {:?}", e)))?;
        self.push_handle(sender);
        let mut resp_stream = response.into_inner();
        let me = self.clone();
        // let reader = tokio::task::Builder::new().name("GRPC envelope receiver").spawn(async move {
        let reader = tokio::task::spawn(async move {
            loop {
                let read = timeout(Duration::from_secs(5), resp_stream.next()).await;
                match read {
                    Ok(data) => {
                        match  data {
                            Some(received) => {
                                match received {
                                    Ok(received) => {
                                        me.parse_incomming_envelope(received).await;
                                    }
                                    Err(e) => {
                                        // error!("Received error from stream: {:?}", e);
                                        me.set_connected(ClientState::Disconnected, Some(&e.to_string()));
                                        break;
                                    }                                        
                                }
                            }
                            None => {
                                me.set_connected(ClientState::Disconnected, Some("Server closed the connection"));
                                break;
                            }                                
                        }
                    }
                    Err(_e) => {
                        // timeout elapsed
                    }                        
                }
            }
        }); // .map_err(|e| OpenIAPError::ClientError(format!("Failed to spawn GRPC envelope receiver task: {:?}", e)))?;
        self.push_handle(reader);
        Ok(())
    }
}