use super::openiap::{AggregateRequest, CountRequest, Envelope, InsertManyRequest, InsertOneRequest, QueryRequest, DistinctRequest};
#[allow(dead_code)]
impl QueryRequest {
    pub fn with_query(collectionname: &str, query: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            query: query.to_string(),
            ..Default::default()
        }
    }
    pub fn with_projection(collectionname: &str, query: &str, projection: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            query: query.to_string(),
            projection: projection.to_string(),
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
                prost::Message::encode(self, &mut buf).unwrap_or(());
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
impl AggregateRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.AggregateRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "aggregate".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl CountRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.CountRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "count".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl DistinctRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DistinctRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "distinct".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}

impl InsertOneRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.InsertOneRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "insertone".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl InsertManyRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.InsertManyRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "insertmany".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
