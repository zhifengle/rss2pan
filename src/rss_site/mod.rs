mod dmhy;
mod mikanani;
mod nyaa;

pub use dmhy::*;
pub use mikanani::*;
pub use nyaa::*;
use rss::Item;

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
