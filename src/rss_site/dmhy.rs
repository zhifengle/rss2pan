use super::MagnetSite;

pub struct Dmhy;

impl MagnetSite for Dmhy {
    fn get_magnet(&self, item: &rss::Item) -> String {
        let url = &item.enclosure().unwrap().url;
        let idx = url.find("&dn=").unwrap();
        url[..idx].to_string()
    }
}
