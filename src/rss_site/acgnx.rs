use super::MagnetSite;

pub struct Acgnx;

impl MagnetSite for Acgnx {
    fn get_magnet(&self, item: &rss::Item) -> String {
        let url = &item.enclosure().unwrap().url;
        let idx = url.find("&tr=");
        if let Some(idx) = idx {
            url[..idx].to_string()
        } else {
            url.to_string()
        }
    }
}
