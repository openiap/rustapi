#![warn(missing_docs)]
use super::openiap::{Envelope, RegisterQueueRequest, UnRegisterQueueRequest, RegisterExchangeRequest, 
    WatchRequest, UnWatchRequest, QueueMessageRequest
};

impl RegisterQueueRequest {
    /// Creates a new `RegisterQueueRequest` with the given `queuename`.
    pub fn byqueuename(queuename: &str) -> Self {
        Self {
            queuename: queuename.to_string()
        }
    }
    /// Converts the `RegisterQueueRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.RegisterQueueRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "registerqueue".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl RegisterExchangeRequest {
    /// Creates a new `RegisterExchangeRequest` with the given `exchangename`.
    pub fn byexchangename(exchangename: &str) -> Self {
        Self {
            exchangename: exchangename.to_string(),
            addqueue: true,
            ..Default::default()
        }
    }
    /// Converts the `RegisterExchangeRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.RegisterExchangeRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "registerexchange".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl  UnRegisterQueueRequest {
    /// Creates a new `UnRegisterQueueRequest` with the given `queuename`.
    pub fn byqueuename(queuename: &str) -> Self {
        Self {
            queuename: queuename.to_string()
        }
    }
    /// Converts the `UnRegisterQueueRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.UnRegisterQueueRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "unregisterqueue".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
    
}
impl WatchRequest {
    /// Creates a new `WatchRequest` with the given `collectionname` and `paths`.
    pub fn new(collectionname: &str, paths: Vec<String>) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            paths
        }
    }
    /// Converts the `WatchRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.WatchRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "watch".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl UnWatchRequest {
    /// Creates a new `UnWatchRequest` with the given `id`.
    pub fn byid(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }
    /// Converts the `UnWatchRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.UnWatchRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "unwatch".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl  QueueMessageRequest {
    /// Creates a new `QueueMessageRequest` with the given `queuename`, `data` and `striptoken`.
    pub fn byqueuename(queuename: &str, data: &str, striptoken: bool) -> Self {
        Self {
            queuename: queuename.to_string(),
            data: data.to_string(),
            striptoken,
            ..Default::default()
        }
    }
    /// Creates a new `QueueMessageRequest` with the given `queuename`, `correlation_id`, `data` and `striptoken`.
    pub fn replyto(queuename: &str, correlation_id: &str, data: &str, striptoken: bool) -> Self {
        Self {
            queuename: queuename.to_string(),
            data: data.to_string(),
            correlation_id: correlation_id.to_string(),
            striptoken,
            ..Default::default()
        }
    }
    /// Creates a new `QueueMessageRequest` with the given `exchangename`, `data` and `striptoken`.
    pub fn byexchangename(exchangename: &str, data: &str, striptoken: bool) -> Self {
        Self {
            exchangename: exchangename.to_string(),
            data: data.to_string(),
            striptoken,
            ..Default::default()
        }
    }
    /// Converts the `QueueMessageRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.QueueMessageRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "queuemessage".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}