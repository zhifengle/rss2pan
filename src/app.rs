use clap::{arg, Command};

pub fn build_app() -> Command<'static> {
    let app = Command::new("rss2pan")
        .version("0.0.1")
        .about("rss to pan")
        .arg(arg!(-r --rss <rss> "rss json").default_value("rss.json"));
    app
}