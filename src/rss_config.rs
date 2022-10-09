use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RssConfig {
    pub name: String,
    pub url: String,
    pub cid: Option<String>,
    pub filter: Option<String>,
    pub expiration: Option<u8>,
}
