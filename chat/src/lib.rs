use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

// An enum which represents the types of messages that can be sent from Client -> Server.
// The parameters are wrapped in an Atomic-Reference-Count smart pointer which is thread-safe
// and enables the reference to the data inside to be shared by multiple owners. 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    }
}

// An enum which represents the types of messages that can be sent from Server -> Client.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}