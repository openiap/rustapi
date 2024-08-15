#![warn(missing_docs)]
use super::protos::{Envelope, UploadRequest, BeginStream, EndStream, Stream};
impl UploadRequest {
    /// Creates a new `UploadRequest` with the given `workitem`.
    pub fn filename(filename: &str) -> Self {
        Self {
            collectionname: "fs.files".to_string(),
            filename: filename.to_string(),
            ..Default::default()
        }
    }
    /// Creates a new `UploadRequest` with the given `workitem`.
    pub fn by_filename(collectionname: &str, filename: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            filename: filename.to_string(),
            ..Default::default()
        }
    }
}
impl UploadRequest {
    /// Creates a new `UploadRequest` with the given `workitem`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.UploadRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "upload".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl BeginStream {
    /// Creates a new `BeginStream` with the given `workitem`.
    pub fn to_envelope(&self, rid:String) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.BeginStream".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "beginstream".into(),
            data: Some(any_message.clone()),
            rid,
            ..Default::default() 
        }
    }
    /// Creates a new `BeginStream` with the given `workitem`.
    pub fn from_rid(rid:String) -> Envelope {
        let req = BeginStream {
            checksum: "".to_string(),
            ..Default::default()
        };
        let envelope = req.to_envelope(rid);
        let mut buf = Vec::new();
        prost::Message::encode(&envelope, &mut buf).unwrap_or(());
        envelope
    }   
}
impl EndStream {
    /// Creates a new `EndStream` with the given `workitem`.
    pub fn to_envelope(&self, rid:String) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.EndStream".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "endstream".into(),
            data: Some(any_message.clone()),
            rid,
            ..Default::default() 
        }
    }
    /// Creates a new `EndStream` with the given `workitem`.
    pub fn from_rid(rid:String) -> Envelope {
        let req = EndStream {            
        };
        let envelope = req.to_envelope(rid);
        let mut buf = Vec::new();
        prost::Message::encode(&envelope, &mut buf).unwrap_or(());
        envelope
    }   
}
impl Stream {
    /// Creates a new `Stream` with the given `workitem`.
    pub fn to_envelope(&self, rid:String) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.Stream".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "stream".into(),
            data: Some(any_message.clone()),
            rid,
            ..Default::default() 
        }
    }
    /// Creates a new `Stream` with the given `workitem`.
    pub fn from_rid(data: Vec<u8>, rid:String) -> Envelope {
        let req = Stream {
            data
        };
        let envelope = req.to_envelope(rid);
        let mut buf = Vec::new();
        prost::Message::encode(&envelope, &mut buf).unwrap_or(());
        envelope
    }    
}
