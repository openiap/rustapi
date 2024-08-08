use super::protos::{Envelope, DownloadRequest};
#[allow(dead_code)]
impl DownloadRequest {
    pub fn id(id: &str) -> Self {
        Self {
            collectionname: "fs.files".to_string(),
            id: id.to_string(),
            ..Default::default()
        }
    }
    pub fn by_id(collectionname: &str, id: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            id: id.to_string(),
            ..Default::default()
        }
    }
    pub fn by_filename(collectionname: &str, filename: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            filename: filename.to_string(),
            ..Default::default()
        }
    }
}
impl DownloadRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DownloadRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "download".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
