#![warn(missing_docs)]
use super::protos::{Envelope, SigninRequest};

impl SigninRequest {
    /// Creates a new `SigninRequest` with the given `username` and `password`.
    pub fn with_userpass(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            ping: true,
            ..Default::default()
        }
    }
    /// Creates a new `SigninRequest` with the given `jwt`.
    pub fn with_jwt(jwt: &str) -> Self {
        Self {
            jwt: jwt.to_string(),
            ..Default::default()
        }
    }
}
impl SigninRequest {
    /// Converts the `SigninRequest` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.SigninRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "signin".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}
