use super::MagnetSite;

pub struct Nyaa;

impl MagnetSite for Nyaa {
    fn get_magnet(&self, item: &rss::Item) -> String {
        // let hash: &str = item.extensions.get("nyaa:infoHash").unwrap();
        todo!()
    }
}
