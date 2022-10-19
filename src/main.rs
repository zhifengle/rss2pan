mod app;
mod db;
mod downloader;
mod request;
mod rss_config;
mod rss_site;
mod yiyiwu;

use app::build_app;
use db::RssService;
use once_cell::sync::OnceCell;
use request::Ajax;
use yiyiwu::{execute_tasks, execute_url_task, execute_all_rss_task};

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let app = build_app();
    let matches = app.get_matches();
    AJAX_INSTANCE.get_or_init(|| Ajax::from_matches(&matches));

    let service = RssService::new();

    let url = matches.get_one::<String>("url");
    if url.is_some() {
        if let Err(err) = execute_url_task(&service, &url.unwrap()).await {
            println!("{}", err);
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
