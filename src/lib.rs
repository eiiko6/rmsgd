use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub client_address: SocketAddr,
    pub username: String,
    pub content: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
}
