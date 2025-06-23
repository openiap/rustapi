#![warn(missing_docs)]
use super::openiap::{
    Envelope, DeletePackageRequest, 
    StartAgentRequest, StopAgentRequest, GetAgentLogRequest, GetAgentPodsRequest, DeleteAgentPodRequest, DeleteAgentRequest
};

impl DeletePackageRequest {
    /// Creates a new `DeletePackageRequest` with the given `packageid`.
    pub fn byid(packageid: &str) -> Self {
        Self {
            packageid: packageid.to_string(),
        }
    }
    /// Converts the `DeletePackageRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DeletePackageRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "deletepackage".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
impl StartAgentRequest {
    /// Creates a new `StartAgentRequest` with the given `agentid`.
    pub fn byid(agentid: &str) -> Self {
        Self {
            agentid: agentid.to_string(),
        }
    }
    /// Converts the `StartAgentRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.StartAgentRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "startagent".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl StopAgentRequest {
    /// Creates a new `StopAgentRequest` with the given `agentid`.
    pub fn byid(agentid: &str) -> Self {
        Self {
            agentid: agentid.to_string(),
        }
    }
    /// Converts the `StopAgentRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.StopAgentRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "stopagent".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl GetAgentLogRequest {
    /// Creates a new `GetAgentLogRequest` with the given `agentid`.
    pub fn new(agentid: &str, podname: &str) -> Self {
        Self {
            agentid: agentid.to_string(),
            podname: podname.to_string()
        }
    }
    /// Converts the `GetAgentLogRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.GetAgentLogRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "getagentlog".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl GetAgentPodsRequest {
    /// Creates a new `GetAgentPodsRequest` with the given `agentid`. Use include_stats to get memory and cpu usage per pod, this is expensive, avoid misuse.
    pub fn byid(agentid: &str, include_stats: bool) -> Self {
        Self {
            agentid: agentid.to_string(),
            stats: include_stats,
        }
    }
    /// Converts the `GetAgentPodsRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.GetAgentPodsRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "getagentpods".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl DeleteAgentPodRequest {
    /// Creates a new `DeleteAgentPodRequest` with the given `agentid` and `podname`.
    pub fn byid(agentid: &str, podname: &str) -> Self {
        Self {
            agentid: agentid.to_string(),
            podname: podname.to_string(),
        }
    }
    /// Converts the `DeleteAgentPodRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DeleteAgentPodRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "deleteagentpod".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}
impl DeleteAgentRequest {
    /// Creates a new `DeleteAgentRequest` with the given `agentid`.
    pub fn byid(agentid: &str) -> Self {
        Self {
            agentid: agentid.to_string(),
        }
    }
    /// Converts the `DeleteAgentRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.DeleteAgentRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "deleteagent".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }    
}