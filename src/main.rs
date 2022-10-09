mod app;
mod db;
mod request;
mod rss_config;
mod rss_site;
mod yiyiwu;

use db::RssService;
use reqwest::Client;
use rss::Channel;
use rss_site::MagnetItem;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::request::build_proxy_client;
use crate::rss_config::RssConfig;
use crate::rss_site::get_site;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = build_proxy_client();
    let service = RssService::new();

    let items = get_items(&client, &service).await?;

    let r = service.save_items(&items);
    println!("{:?} {}", r, items.len());

    Ok(())
}

async fn get_feed(client: &Client, url: &str) -> anyhow::Result<Channel> {
    let content = client.get(url).send().await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
fn get_feed_by_file(path: PathBuf) -> anyhow::Result<Channel> {
    let file = File::open(path).expect("no such file");
    let buf_reader = BufReader::new(file);
    let channel = Channel::read_from(buf_reader)?;
    Ok(channel)
}

fn get_rss_dict(path: Option<PathBuf>) -> anyhow::Result<HashMap<String, Vec<RssConfig>>> {
    let file = if let Some(p) = path {
        File::open(p)?
    } else {
        File::open("rss.json")?
    };
    let reader = BufReader::new(file);
    let rss_dict: HashMap<String, Vec<RssConfig>> = serde_json::from_reader(reader)?;
    Ok(rss_dict)
}

async fn get_items(client: &Client, service: &RssService) -> anyhow::Result<Vec<MagnetItem>> {
    let mut item_list: Vec<MagnetItem> = vec![];
    let rss_dict = get_rss_dict(None)?;
    for (k, v) in rss_dict.iter() {
        let site = get_site(k);
        for config in v.iter() {
            if let Ok(channel) = get_feed(&client, &config.url).await {
                for item in channel.items() {
                    let m = site.get_magnet_item(item);
                    let mut flag = true;
                    if let Some(pat) = &config.filter {
                        flag = m.title.contains(pat);
                    }
                    if flag && !service.has_item(&m.magnet) {
                        item_list.push(m)
                    }
                }
            }
        }
    }
    Ok(item_list)
}
