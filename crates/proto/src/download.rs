#![warn(missing_docs)]
use super::openiap::{Envelope, DownloadRequest};
impl DownloadRequest {
    /// Creates a new `DownloadRequest` with the given `id`.
    #[tracing::instrument(skip_all)]
    pub fn id(id: &str) -> Self {
        Self {
            collectionname: "fs.files".to_string(),
            id: id.to_string(),
            ..Default::default()
        }
    }
    /// Creates a new `DownloadRequest` with the given `filename`.
    #[tracing::instrument(skip_all)]
    pub fn by_id(collectionname: &str, id: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            id: id.to_string(),
            ..Default::default()
        }
    }
    /// Creates a new `DownloadRequest` with the given `filename`.
    #[tracing::instrument(skip_all)]
    pub fn by_filename(collectionname: &str, filename: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            filename: filename.to_string(),
            ..Default::default()
        }
    }
}
impl DownloadRequest {
    /// Converts the `DownloadRequest` to an `Envelope`.
    #[tracing::instrument(skip_all)]
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
