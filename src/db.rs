use chrono::prelude::*;

use anyhow::Result;
use rusqlite::{Connection, Error};

use crate::rss_site::MagnetItem;

pub struct RssService {
    conn: Box<Connection>,
}

impl RssService {
    // pub fn new<P: AsRef<Path>>(path: P) -> Self {
    pub fn new() -> Self {
        let path = "db.sqlite";
        let conn = Box::new(Connection::open(path).expect("invalid db path"));
        conn.execute(
            "CREATE TABLE if not exists `rss_items` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `link` VARCHAR(255), `title` VARCHAR(255), `guid` VARCHAR(255), `pubDate` DATETIME, `creator` VARCHAR(255), `summary` TEXT, `content` VARCHAR(255), `isoDate` DATETIME, `categories` VARCHAR(255), `contentSnippet` VARCHAR(255), `done` TINYINT(1) DEFAULT 0, `magnet` VARCHAR(255) NOT NULL, `createdAt` DATETIME NOT NULL, `updatedAt` DATETIME NOT NULL)",
            (), // empty list of parameters.
        ).unwrap();
        conn.execute(
            "CREATE TABLE if not exists `sites_status` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `needLogin` TINYINT(1), `abnormalOp` TINYINT(1), `createdAt` DATETIME NOT NULL, `updatedAt` DATETIME NOT NULL)",
            (), // empty list of parameters.
        ).unwrap();
        // let conn = Box::new(Connection::open_in_memory().expect("invalid db path"));
        Self { conn }
    }
    pub fn save_items(&self, items: &[MagnetItem], done: bool) -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        // let now: DateTime<Utc> = Utc::now() + FixedOffset::east(8 * 3600);
        let done = (done as u8).to_string();
        for item in items {
            self.conn.execute("INSERT INTO rss_items (`link`,`title`,`content`,`magnet`,`done`,`createdAt`,`updatedAt`) VALUES (?,?,?,?,?,?,?)", [
                &item.link,
                &item.title,
                item.content.as_deref().unwrap_or(""),
                &item.magnet,
                &done,
                &now.to_string(),
                &now.to_string(),
            ])?;
        }
        Ok(())
    }
    pub fn has_item(&self, magnet: &str) -> bool {
        let (item,) = self
            .conn
            .query_row(
                "SELECT count(*) AS num FROM rss_items WHERE magnet = ?1",
                [magnet],
                |row| <(u8,)>::try_from(row),
            )
            .unwrap();
        item > 0
    }
    pub fn get_item_by_magnet(&self, magnet: &str) -> Result<MagnetItem> {
        // 本质上是调用的 next 取第一个; LIMIT 1 不需要
        let item = self.conn.query_row(
            "SELECT link,title,magnet FROM rss_items WHERE magnet = ?1",
            [magnet],
            |row| {
                Ok(MagnetItem {
                    link: row.get(0)?,
                    title: row.get(1)?,
                    magnet: row.get(2)?,
                    content: None,
                    description: None,
                })
            },
        )?;
        Ok(item)
    }
    #[allow(dead_code)]
    pub fn update_status(&self, host: &str, key: &str, value: bool) -> Result<usize> {
        let value: u8 = value.into();
        let value = value.to_string();
        let stmt = format!("UPDATE sites_status SET {key} = {value} WHERE name = \"{host}\"");
        let num = self.conn.execute(&stmt, [])?;
        Ok(num)
    }
    #[allow(dead_code)]
    pub fn reset_status(&self, name: &str) -> Result<usize> {
        let stmt =
            format!("UPDATE sites_status SET abnormalOp = 0,needLogin = 0 WHERE name = \"{name}\"");
        let num = self.conn.execute(&stmt, [])?;
        Ok(num)
    }
    #[allow(dead_code)]
    pub fn is_ready(&self, name: &str) -> bool {
        let r = self.conn.query_row(
            "SELECT needLogin,abnormalOp FROM sites_status WHERE name = ?1",
            [name],
            |row| <(u8, u8)>::try_from(row),
        );
        match r {
            Ok((0, 0)) => true,
            Ok(_) => false,
            Err(Error::QueryReturnedNoRows) => {
                let now: DateTime<Utc> = Utc::now();
                self.conn
                    .execute(
                        "INSERT INTO sites_status (name,`createdAt`,`updatedAt`,`needLogin`, `abnormalOp`) VALUES (?,?,?,0,0)",
                        [name, &now.to_string(), &now.to_string()],
                    )
                    .unwrap();
                true
            }
            Err(err) => panic!("{:?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_items_test() {
        let value = false;
        let value: u8 = value.into();
        let value = value.to_string();
        println!("{}", value);
    }
    #[test]
    fn get_item_test() {
        let service = RssService::new();
        let r = service.get_item_by_magnet("magnet");
        assert!(r.is_err());
    }
    #[test]
    fn update_status_test() {
        let host = "115.com";
        let key = "abnormalOp";
        let value = false;
        let service = RssService::new();
        let r = service.update_status(host, key, value);
        println!("{:?}", r);
    }
    #[test]
    fn is_ready_test() {
        let host = "114.com";
        let service = RssService::new();
        let r = service.is_ready(host);
        assert!(r);
    }
}
