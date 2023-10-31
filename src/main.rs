mod app;
mod db;
mod downloader;
mod request;
mod rss_config;
mod rss_site;
mod utils;
mod yiyiwu;
mod m115;

use std::path::PathBuf;

use app::build_app;
use db::RssService;
use once_cell::sync::OnceCell;
use request::Ajax;
use utils::get_magnet_list_by_txt;
use yiyiwu::{execute_all_rss_task, execute_magnets_task, execute_tasks, execute_url_task};

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();
static RSS_JSON: OnceCell<Option<PathBuf>> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let app = build_app();
    let matches = app.get_matches();
    AJAX_INSTANCE.get_or_init(|| Ajax::from_matches(&matches));
    RSS_JSON.get_or_init(|| matches.get_one::<PathBuf>("rss").map(|p| p.clone()));

    if let Some(("magnet", matches)) = matches.subcommand() {
        let link = matches.get_one::<String>("link").cloned();
        let txt = matches.get_one::<PathBuf>("txt").cloned();
        let cid = matches.get_one::<String>("cid").cloned();
        let mut magnets: Vec<String> = Vec::new();
        if txt.is_some() {
            magnets = get_magnet_list_by_txt(&txt.unwrap())?;
        } else if link.is_some() {
            magnets.push(link.unwrap());
        } else {
            eprintln!("magnet link or txt file is required");
            std::process::exit(1);
        }
        if let Err(err) = execute_magnets_task(&magnets, cid).await {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        return Ok(());
    }

    let service = RssService::new();

    let url = matches.get_one::<String>("url");
    if url.is_some() {
        if let Err(err) = execute_url_task(&service, &url.unwrap()).await {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        return Ok(());
    }
    if matches.get_one::<bool>("concurrent").copied() == Some(true) {
        if let Err(err) = execute_tasks(&service).await {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    } else {
        if let Err(err) = execute_all_rss_task(&service).await {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
