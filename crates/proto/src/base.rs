#![warn(missing_docs)]

use super::protos::{
    Envelope, GetDocumentVersionRequest, CustomCommandRequest, ListCollectionsRequest, DropCollectionRequest, CreateCollectionRequest,
    GetIndexesRequest, CreateIndexRequest, DropIndexRequest, EnsureCustomerRequest, InvokeOpenRpaRequest,
    Customer, StripeCustomer
};

impl GetDocumentVersionRequest {
    /// Creates a new `GetDocumentVersionRequest` with the given `collectionname` and `documentid`.
    pub fn byid(collectionname: &str, documentid: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            id: documentid.to_string(),
            decrypt: true,
            version: 0,
        }
    }
    /// Creates a new `GetDocumentVersionRequest` with the given `collectionname`, `documentid` and `version`.
    pub fn byversion(collectionname: &str, documentid: &str, version : i32) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            id: documentid.to_string(),
            decrypt: true,
            version,
        }
    }
    /// Converts the `GetDocumentVersionRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.GetDocumentVersionRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "getdocumentversion".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}

impl CustomCommandRequest {
    /// Creates a new `CustomCommandRequest` with the given `command`.
    pub fn bycommand(command: &str) -> Self {
        Self {
            command: command.to_string(),
            data: "".to_string(),
            id: "".to_string(),
            name: "".to_string(), 
        }
    }
    /// Creates a new `CustomCommandRequest` with the given `command` and `data`.
    pub fn bydata(command: &str, data: &str) -> Self {
        Self {
            command: command.to_string(),
            data: data.to_string(),
            id: "".to_string(),
            name: "".to_string(), 
        }
    }
    /// Converts the `CustomCommandRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.CustomCommandRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "customcommand".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl ListCollectionsRequest {
    /// Creates a new `ListCollectionsRequest` with the given `name`.
    pub fn new(includehist: bool) -> Self {
        Self {
            includehist,
        }
    }
    /// Converts the `ListCollectionsRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.ListCollectionsRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "listcollections".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl DropCollectionRequest {
    /// Creates a new `DropCollectionRequest` with the given `name`.
    pub fn byname(collectionname: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
        }
    }
    /// Converts the `DropCollectionRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DropCollectionRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "dropcollection".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl CreateCollectionRequest {
    /// Creates a new `CreateCollectionRequest` with the given `name`.
    pub fn byname(collectionname: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            ..Default::default()
        }
    }
    /// Converts the `CreateCollectionRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.CreateCollectionRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "createcollection".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl GetIndexesRequest {
    /// Creates a new `GetIndexesRequest` with the given `collectionname`.
    pub fn bycollectionname(collectionname: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
        }
    }
    /// Converts the `GetIndexesRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.GetIndexesRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "getindexes".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl CreateIndexRequest {
    /// Creates a new `CreateIndexRequest` with the given `collectionname` and `index_def`.
    pub fn bycollectionname(collectionname: &str, index_def: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            index: index_def.to_string(),
            options: "".to_string(),
            name: "".to_string(),
        }
    }
    /// Converts the `CreateIndexRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.CreateIndexRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "createindex".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl DropIndexRequest {
    /// Creates a new `DropIndexRequest` with the given `collectionname` and `indexname`.
    pub fn bycollectionname(collectionname: &str, indexname: &str) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            name: indexname.to_string(),
        }
    }
    /// Converts the `DropIndexRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DropIndexRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "dropindex".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl Customer {
    /// Creates a new `Customer` with the given `name`.
    pub fn byname(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    /// Creates a new `Customer` with userid as owner (use if not logged in user)
    pub fn byuserid(name: &str, userid: &str) -> Self {
        Self {
            name: name.to_string(),
            userid: userid.to_string(),
            ..Default::default()
        }
    }
    /// Converts the `Customer` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.Customer".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "customer".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
    
}
impl StripeCustomer {
    /// Creates a new `StripeCustomer` with the given `name`.
    pub fn byname(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            ..Default::default()
        }
    }
    /// Converts the `StripeCustomer` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.StripeCustomer".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "stripecustomer".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
    
}
impl EnsureCustomerRequest {
    /// Creates a new `EnsureCustomerRequest` with the given `customerid`.
    pub fn new(customer: Option<Customer>, ensureas: &str, stripe: Option<StripeCustomer>) -> Self {
        Self {
            customer: customer,
            ensureas: ensureas.to_string(),
            stripe: stripe
        }
    }
    /// Converts the `EnsureCustomerRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.EnsureCustomerRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "ensurecustomer".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
    
}

impl InvokeOpenRpaRequest {
    /// Creates a new `InvokeOpenRpaRequest` with the given `robotid`, `workflowid` and `payload`.
    /// if rpc is true, will not return until workflow has completed
    pub fn new(robotid: &str, workflowid: &str, payload: &str, rpc: bool) -> Self {
        Self {
            robotid: robotid.to_string(),
            workflowid: workflowid.to_string(),
            payload: payload.to_string(),
            rpc
        }
    }
    
}