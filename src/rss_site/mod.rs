mod dmhy;
mod mikanani;
mod nyaa;

use regex::Regex;
use reqwest::Method;
use rss::{Channel, Item};
use std::io::BufReader;
use std::{fs::File, path::PathBuf};

pub use dmhy::*;
pub use mikanani::*;
pub use nyaa::*;

use crate::rss_config::RssConfig;
use crate::AJAX_INSTANCE;

pub trait MagnetSite {
    fn get_magnet(&self, item: &Item) -> String;
    fn get_magnet_item(&self, item: &Item) -> MagnetItem {
        MagnetItem {
            title: item.title().map_or("".to_string(), |s| s.to_string()),
            link: item.link().unwrap().to_string(),
            magnet: self.get_magnet(item),
            description: item.description().map(|s| s.to_string()),
            content: item.content().map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct MagnetItem {
    pub title: String,
    pub link: String,
    pub magnet: String,
    pub description: Option<String>,
    pub content: Option<String>,
}

pub fn get_site(name: &str) -> Box<dyn MagnetSite> {
    let site: String = if name.starts_with("http") {
        let url_obj = url::Url::parse(name).unwrap();
        url_obj.host_str().unwrap().to_string()
    } else {
        name.to_string()
    };
    let name = site.as_str();

    match name {
        "mikanani.me" => Box::new(Mikanani),
        "nyaa.si" => Box::new(Nyaa),
        "sukebei.nyaa.si" => Box::new(Nyaa),
        "share.dmhy.org" => Box::new(Dmhy),
        _ => panic!("invalid name"),
    }
}

pub async fn get_feed(url: &str) -> anyhow::Result<Channel> {
    let content = AJAX_INSTANCE
        .get()
        .unwrap()
        .gen_req(Method::GET, url)
        .send()
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn get_magnetitem_list(config: &RssConfig) -> Vec<MagnetItem> {
    let site = get_site(&config.url);
    if let Ok(channel) = get_feed(&config.url).await {
        channel
            .items()
            .iter()
            .map(|item| site.get_magnet_item(item))
            .filter(|m| {
                let mut flag = true;
                if let Some(pat) = &config.filter {
                    if pat.starts_with("/") && pat.ends_with("/") {
                        let re = Regex::new(&pat[1..pat.len() - 1]);
                        match re {
                            Ok(re) => {
                                flag = re.is_match(&m.title);
                            }
                            Err(_) => {}
                        }
                    } else {
                        flag = m.title.contains(pat);
                    }
                }
                flag
            })
            .collect()
    } else {
        vec![]
    }
}

#[allow(dead_code)]
pub fn get_feed_by_file(path: PathBuf) -> anyhow::Result<Channel> {
    let file = File::open(path).expect("no such file");
    let buf_reader = BufReader::new(file);
    let channel = Channel::read_from(buf_reader)?;
    Ok(channel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::RssService;

    #[test]
    fn test_db_save_items() {
        let channel = get_feed_by_file("tests/Bangumi.rss".into());
        assert!(channel.is_ok());
        let channel = channel.unwrap();

        let service = RssService::new();
        let site = get_site("mikanani.me");
        let items: Vec<MagnetItem> = channel
            .items()
            .iter()
            .map(|item| site.get_magnet_item(item))
            .collect();
        let res = service.save_items(&items, true);
        assert!(res.is_ok());
    }
    #[test]
    fn test_re() {
        let str_list = [
            "[7月新番][传颂之物 二人的白皇][Utawarerumono - Futari no Hakuoro][09][1080P][MP4][GB][简中] [241.72 MB]",
            "【幻樱字幕组】【7月新番】【传颂之物 二人白皇 Utawarerumono-Futari no Hakuoro-】【16】【BIG5_MP4】【1920X1080】 [321.13 MB]",
            "[动漫国字幕组&澄空学园&LoliHouse] 传颂之物 二人的白皇 / Utawarerumono Futari no Hakuoro - 16 [WebRip 1080p HEVC-10bit AAC][简繁外挂字幕] [485.4 MB]"
        ];
        let pat = "/澄空学园|幻樱|\\d{4}[p]/";
        let re = Regex::new(&pat[1..pat.len() - 1]).unwrap();
        assert_eq!(str_list.map(|s| re.is_match(s)), [true, true, true]);
    }
}
