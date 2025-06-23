#![warn(missing_docs)]
use std::fmt;
use super::openiap::Envelope;
impl super::openiap::ErrorResponse {
    /// Creates a new `ErrorResponse` with the given `message` and `code`.
    pub fn new(message: &str, code: i32) -> Self {
        Self {
            code,
            message: message.to_string(),
            ..Default::default()
        }
    }
    /// Converts the `ErrorResponse` to an `Envelope`.
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.ErrorResponse".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap_or(());
                buf
            },
        };
        Envelope {
            command: "error".into(),
            data: Some(any_message.clone()),
            ..Default::default()
        }
    }
}


/// Error type for OpenIAP
#[derive(Debug)]
pub enum OpenIAPError {
    /// Client error
    ClientError(String),
    /// Server error
    ServerError(String),
    /// Custom error
    CustomError(String),
}
impl fmt::Display for OpenIAPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenIAPError::ClientError(e) => write!(f, "Client Error {}", e),
            OpenIAPError::ServerError(e) => write!(f, "Server Error {}", e),
            OpenIAPError::CustomError(e) => write!(f, "Custom Error {}", e),
        }
    }
}
impl std::error::Error for OpenIAPError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // match self {
        //     // OpenIAPError::NestedError(e) => Some(&**e),
        //     _ => None,
        // }
        None
    }
}
