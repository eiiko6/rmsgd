use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: User,
    pub content: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub struct User {
    pub client_address: SocketAddr,
    pub username: String,
}
