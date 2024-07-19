use super::openiap::{Envelope, QueryRequest};
#[allow(dead_code)]
impl QueryRequest {
    pub fn with_query(collectionname: &str, query: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            query: query.to_string(),
            ..Default::default()
        }
    }
}
impl QueryRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.QueryRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap();
                buf
            },
        };
        Envelope {
            command: "query".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
