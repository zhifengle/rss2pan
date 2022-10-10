use super::MagnetSite;

pub struct Nyaa;

impl MagnetSite for Nyaa {
    fn get_magnet(&self, item: &rss::Item) -> String {
        let hash = item
            .extensions()
            .get("nyaa")
            .unwrap()
            .get("infoHash")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .unwrap();
        format!("magnet:?xt=urn:btih:{}", hash)
    }
}
