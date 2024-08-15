#![warn(missing_docs)]
use super::protos::{AggregateRequest, CountRequest, Envelope, InsertManyRequest, InsertOneRequest, UpdateOneRequest, InsertOrUpdateOneRequest, QueryRequest, DistinctRequest,
    DeleteOneRequest, DeleteManyRequest, InsertOrUpdateManyRequest, UpdateDocumentRequest
    };
    impl QueryRequest {
        /// Creates a new `QueryRequest` with the given `collectionname` and `query`.
        #[tracing::instrument(skip_all)]
        pub fn with_query(collectionname: &str, query: &str) -> Self {
            Self {
                collectionname: collectionname.to_string(),
                query: query.to_string(),
                ..Default::default()
            }
        }
        /// Creates a new `QueryRequest` with the given `collectionname`, `query` and `projection`.
        #[tracing::instrument(skip_all)]
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
        /// Converts the `QueryRequest` to an `Envelope`.
        #[tracing::instrument(skip_all)]
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
        /// Creates a new `AggregateRequest` with the given `collectionname` and `pipeline`.
        #[tracing::instrument(skip_all)]
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
        /// Creates a new `CountRequest` with the given `collectionname` and `query`.
        #[tracing::instrument(skip_all)]
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
        /// Creates a new `DistinctRequest` with the given `collectionname`, `field` and `query`.
        #[tracing::instrument(skip_all)]
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
        /// Creates a new `InsertOneRequest` with the given `collectionname` and `document`.
        #[tracing::instrument(skip_all)]
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
        /// Creates a new `InsertManyRequest` with the given `collectionname` and `documents`.
        #[tracing::instrument(skip_all)]
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
    impl UpdateOneRequest {
        /// Creates a new `UpdateOneRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.UpdateOneRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "updateone".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }    
    }
    impl InsertOrUpdateOneRequest {
        /// Creates a new `InsertOrUpdateOneRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.InsertOrUpdateOneRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "insertorupdateone".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }
    }
    impl  InsertOrUpdateManyRequest {
        /// Creates a new `InsertOrUpdateManyRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.InsertOrUpdateManyRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "insertorupdatemany".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }    
    }
    impl  UpdateDocumentRequest {
        /// Creates a new `UpdateDocumentRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.UpdateDocumentRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "updatedocument".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }        
    }    
    impl DeleteOneRequest {
        /// Creates a new `DeleteOneRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.DeleteOneRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "deleteone".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }        
    }
    impl DeleteManyRequest {
        /// Creates a new `DeleteManyRequest` with the given `collectionname`, `filter` and `update`.
        #[tracing::instrument(skip_all)]
        pub fn to_envelope(&self) -> Envelope {
            let any_message = prost_types::Any {
                type_url: "type.googleapis.com/openiap.DeleteManyRequest".to_string(),
                value: {
                    let mut buf = Vec::new();
                    prost::Message::encode(self, &mut buf).unwrap_or(());
                    buf
                },
            };
            Envelope {
                command: "deletemany".into(),
                data: Some(any_message.clone()),
                ..Default::default() 
            }
        }        
    }