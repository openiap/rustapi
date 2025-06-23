// #![warn(missing_docs)]
//! The `proto` crate provides the `protos` used by the OpenIAP client(s).

// Instead of using tonic::include_proto!("openiap"), directly include the generated file.
pub mod openiap;
/// The `base` module provides the `CustomCommandRequest` struct and its methods.
pub mod base;
/// The `download` module provides the `Download` struct and its methods.
pub mod download;
/// The `errors` module provides the `Error` struct and its methods.
pub mod errors;
/// The `query` module provides the `Query` struct and its methods.
pub mod query;
/// The `queue` module provides the `RegisterQueueRequest`, `UnRegisterQueueRequest`, `RegisterExchangeRequest`, `WatchRequest`, `UnWatchRequest`, and `QueueMessageRequest` structs and their methods.
pub mod queue;
/// The `signin` module provides the `SigninRequest` struct and its methods.
pub mod signin;
/// The `upload` module provides the `Upload` struct and its methods.
pub mod upload;
/// The `workitem` module provides the `WorkItem` struct and its methods.
pub mod workitem;
/// The `agent` module provides the `Agent` struct and its methods.
pub mod agent;
