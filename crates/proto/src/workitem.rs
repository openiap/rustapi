#![warn(missing_docs)]
use super::protos::{
    Envelope, PushWorkitemRequest, PushWorkitemsRequest, PopWorkitemRequest, UpdateWorkitemRequest, DeleteWorkitemRequest,
    AddWorkItemQueueRequest, UpdateWorkItemQueueRequest, DeleteWorkItemQueueRequest
};

impl PushWorkitemRequest {
    /// Creates a new `PushWorkitemRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.PushWorkitemRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "pushworkitem".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl PushWorkitemsRequest {
    /// Creates a new `PushWorkitemsRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.PushWorkitemsRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "pushworkitems".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl PopWorkitemRequest {
    /// Creates a new `PopWorkitemRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.PopWorkitemRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "popworkitem".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl UpdateWorkitemRequest {
    /// Creates a new `UpdateWorkitemRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.UpdateWorkitemRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "updateworkitem".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl DeleteWorkitemRequest {
    /// Creates a new `DeleteWorkitemRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DeleteWorkitemRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "deleteworkitem".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl AddWorkItemQueueRequest {
    /// Creates a new `AddWorkItemQueueRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.AddWorkItemQueueRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "addworkitemqueue".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl UpdateWorkItemQueueRequest {
    /// Creates a new `UpdateWorkItemQueueRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.UpdateWorkItemQueueRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "updateworkitemqueue".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl DeleteWorkItemQueueRequest {
    /// Creates a new `DeleteWorkItemQueueRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DeleteWorkItemQueueRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "deleteworkitemqueue".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}