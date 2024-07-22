use super::openiap::{Envelope, RegisterQueueRequest, WatchRequest, UnWatchRequest,
    // RegisterExchangeRequest, UnRegisterQueueRequest,
};
#[allow(dead_code)]
impl RegisterQueueRequest {
    pub fn queuename(queuename: &str) -> Self {
        Self {
            queuename: queuename.to_string(),
            ..Default::default()
        }
    }
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
// impl RegisterExchangeRequest {
//     pub fn exchangename(exchangename: &str) -> Self {
//         Self {
//             exchangename: exchangename.to_string(),
//             ..Default::default()
//         }
//     }
//     pub fn to_envelope(&self) -> Envelope {
//         let any_message = prost_types::Any {
//             type_url: "type.googleapis.com/openiap.RegisterExchangeRequest".to_string(),
//             value: {
//                 let mut buf = Vec::new();
//                 prost::Message::encode(self, &mut buf).unwrap_or(());
//                 buf
//             },
//         };
//         Envelope {
//             command: "registerexchange".into(),
//             data: Some(any_message.clone()),
//             ..Default::default() 
//         }
//     }
// }
// impl UnRegisterQueueRequest {
//     pub fn queuename(queuename: &str) -> Self {
//         Self {
//             queuename: queuename.to_string(),
//             ..Default::default()
//         }
//     }
//     pub fn to_envelope(&self) -> Envelope {
//         let any_message = prost_types::Any {
//             type_url: "type.googleapis.com/openiap.UnRegisterQueue".to_string(),
//             value: {
//                 let mut buf = Vec::new();
//                 prost::Message::encode(self, &mut buf).unwrap_or(());
//                 buf
//             },
//         };
//         Envelope {
//             command: "unregisterqueue".into(),
//             data: Some(any_message.clone()),
//             ..Default::default() 
//         }
//     }
// }
impl WatchRequest {
    pub fn new(collectionname: &str, paths: Vec<String>) -> Self {
        Self {
            collectionname: collectionname.to_string(),
            paths: paths,
            ..Default::default()
        }
    }
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
    pub fn byid(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }
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