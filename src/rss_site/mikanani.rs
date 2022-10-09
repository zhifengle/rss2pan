use rss::Item;

use super::MagnetSite;

pub struct Mikanani;

impl MagnetSite for Mikanani {
    fn get_magnet(&self, item: &Item) -> String {
        let link = item.link().unwrap();
        let idx = link.find("Episode/").unwrap();
        format!("magnet:?xt=urn:btih:{}", &link[idx + 8..])
    }
}
