use super::protos::{
    Envelope, PushWorkitemRequest, PopWorkitemRequest
};

impl PushWorkitemRequest {
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
impl PopWorkitemRequest {
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