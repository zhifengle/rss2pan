use std::path::PathBuf;

use clap::{arg, crate_version, ArgAction, Command};

pub fn build_app() -> Command<'static> {
    let app = Command::new("rss2pan")
        .version(crate_version!())
        .about("rss to pan")
        // .arg(arg!(-r --rss [rss] "rss json").default_value("rss.json"))
        .arg(arg!(-u --url [url] "rss url"))
        .arg(arg!(-m --concurrent "concurrent request").action(ArgAction::SetTrue))
        .arg(arg!(-c --chrome [chrome] "Chrome's name. Chrome, Chromium, Chrome Beta or Edge is OK.")
        .default_value("Chrome"))
        .arg(
            arg!(chrome_path: -p --"chrome-path" [chrome_path] "the user data path of Chrome")
            .value_parser(clap::value_parser!(PathBuf))
            .conflicts_with("firefox")
        )
        .arg(
            arg!(firefox: -f --firefox [firefox] "path of firefox profile")
                .value_parser(clap::value_parser!(PathBuf)
        )
        .conflicts_with("chrome_path"));

    app
}

#[test]
fn t_cmd() {
    let cmd = build_app();
    let matches = cmd.clone().try_get_matches_from(["rss2pan", "-m"]).unwrap();
    assert!(matches.contains_id("concurrent"));
    let matches = cmd.clone().try_get_matches_from(["rss2pan"]).unwrap();
    assert_eq!(matches.get_one::<bool>("concurrent").copied(), Some(false));
}
