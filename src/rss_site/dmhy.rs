use super::MagnetSite;

pub struct Dmhy;

impl MagnetSite for Dmhy {
    fn get_magnet(&self, item: &rss::Item) -> String {
        let url = &item.enclosure().unwrap().url;
        let idx = url.find("&dn=");
        if let Some(idx) = idx {
            url[..idx].to_string()
        } else {
            url.to_string()
        }
    }
}
