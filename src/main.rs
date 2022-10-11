mod app;
mod db;
mod request;
mod rss_config;
mod rss_site;
mod yiyiwu;

use app::build_app;
use db::RssService;
use once_cell::sync::OnceCell;
use request::Ajax;
use reqwest::Method;
use rss::Channel;
use rss_config::{get_rss_dict, RssConfig};
use rss_site::{MagnetItem, get_site};
use yiyiwu::Yiyiwu;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    AJAX_INSTANCE.get_or_init(|| Ajax::new());
    let app = build_app();
    let matches = app.get_matches();
    let yiyiwu = Yiyiwu::from_matches(&matches);
    let service = RssService::new();

    let url = matches.get_one::<String>("url");
    if url.is_some() {
        if let Err(err) = execute_url_task(&service, &yiyiwu, &url.unwrap()).await {
            println!("{}", err);
        }
        return Ok(())
    }

    if let Err(err) = execute_all_rss_task(&service, &yiyiwu).await {
        println!("{}", err);
    }

    Ok(())
}

fn get_rss_config_by_url(url: &str) -> anyhow::Result<RssConfig> {
    let rss_dict = get_rss_dict(None)?;
    let url_obj = url::Url::parse(url)?;
    let site = url_obj.host_str().unwrap().to_string();
    let config = rss_dict
        .get(&site)
        .unwrap()
        .iter()
        .find(|config| config.url == url);
    Ok(match config {
        Some(c) => c.clone(),
        None => {
            let mut config = RssConfig::default();
            config.url = url.to_string();
            config
        }
    })
}

async fn get_feed(url: &str) -> anyhow::Result<Channel> {
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

#[allow(dead_code)]
fn get_feed_by_file(path: PathBuf) -> anyhow::Result<Channel> {
    let file = File::open(path).expect("no such file");
    let buf_reader = BufReader::new(file);
    let channel = Channel::read_from(buf_reader)?;
    Ok(channel)
}

async fn execute_rss_task(
    service: &RssService,
    yiyiwu: &Yiyiwu,
    config: &RssConfig,
) -> anyhow::Result<()> {
    let site = get_site(&config.url);
    if let Ok(channel) = get_feed(&config.url).await {
        let mut item_list: Vec<MagnetItem> = Vec::with_capacity(channel.items().len());
        for item in channel.items() {
            let m = site.get_magnet_item(item);
            let mut flag = true;
            if let Some(pat) = &config.filter {
                flag = m.title.contains(pat);
            }
            if flag && !service.has_item(&m.magnet) {
                item_list.push(m)
            }
        }
        if item_list.len() == 0 {
            return Ok(());
        }
        log::info!("[{}] add {} task", config.url, item_list.len());
        let tasks: Vec<&str> = item_list.iter().map(|item| &*item.magnet).collect();
        let res = yiyiwu.add_batch_task(&tasks, config.cid.clone()).await?;
        match res.errcode {
            0 => {
                service.save_items(&item_list, true)?;
            }
            911 => {
                log::error!("[115] response {:?}", res);
                return Err(anyhow::format_err!("115 abnoraml operation"));
            }
            10004 => {
                log::warn!("[115] wrong links");
            }
            10008 => {
                log::warn!("[115] task exist");
                service.save_items(&item_list, true)?;
            }
            _ => {
                log::error!("[115] response {:?}", res);
            }
        };
    }

    Ok(())
}

async fn execute_all_rss_task(service: &RssService, yiyiwu: &Yiyiwu) -> anyhow::Result<()> {
    if !yiyiwu.is_logged().await {
        return Err(anyhow::format_err!("115 need login"));
    }
    let rss_dict = get_rss_dict(None)?;
    for (_, v) in rss_dict.iter() {
        for config in v.iter() {
            execute_rss_task(service, yiyiwu, config).await?;
        }
    }
    Ok(())
}
async fn execute_url_task(service: &RssService, yiyiwu: &Yiyiwu, url: &str) -> anyhow::Result<()> {
    if !yiyiwu.is_logged().await {
        return Err(anyhow::format_err!("115 need login"));
    }
    let config = get_rss_config_by_url(url)?;

    execute_rss_task(service, yiyiwu, &config).await?;
    Ok(())
}

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
