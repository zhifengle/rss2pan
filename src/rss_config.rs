use std::{path::PathBuf, io::BufReader, collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RssConfig {
    pub name: String,
    pub url: String,
    pub cid: Option<String>,
    pub filter: Option<String>,
    pub expiration: Option<u8>,
}

pub fn get_rss_dict(path: Option<PathBuf>) -> anyhow::Result<HashMap<String, Vec<RssConfig>>> {
    let file = if let Some(p) = path {
        File::open(p)?
    } else {
        File::open("rss.json")?
    };
    let reader = BufReader::new(file);
    let rss_dict: HashMap<String, Vec<RssConfig>> = serde_json::from_reader(reader)?;
    Ok(rss_dict)
}