mod mikanani;
mod nyaa;

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
    match name {
        "mikanani.me" => Box::new(Mikanani),
        "nyaa.si" => Box::new(Nyaa),
        _ => panic!("invalid name"),
    }
}
