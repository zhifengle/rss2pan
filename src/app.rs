use std::path::PathBuf;

use clap::{arg, crate_version, ArgAction, Command};

pub fn build_app() -> Command {
    let app = Command::new("rss2pan")
        .version(crate_version!())
        .about("rss to pan")
        .arg(arg!(-r --rss [rss] "rss.json path").value_parser(clap::value_parser!(PathBuf)))
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
        .conflicts_with("chrome_path"))
        .subcommand(
            Command::new("magnet")
                .about("magnet to pan")
                .arg(arg!(-l --link [link] "magnet link").conflicts_with("txt"))
                .arg(arg!(--txt [txt] "magnet txt file")
                    .value_parser(clap::value_parser!(PathBuf))
                    .conflicts_with("link"))
                .arg(arg!(--cid [cid] "folder id in wangpan"))
                .arg(arg!(-c --chrome [chrome] "Chrome's name. Chrome, Chromium, Chrome Beta or Edge is OK.")
                    .default_value("Chrome"))
        );

    app
}

#[test]
fn t_cmd() {
    let cmd = build_app();
    let matches = cmd.clone().try_get_matches_from(["rss2pan", "-m"]).unwrap();
    assert_eq!(matches.get_one::<bool>("concurrent").copied(), Some(true));
    let matches = cmd.clone().try_get_matches_from(["rss2pan"]).unwrap();
    assert_eq!(matches.get_one::<bool>("concurrent").copied(), Some(false));
}

#[test]
fn t_subcomd() {
    let cmd = build_app();
    let matches = cmd
        .clone()
        .try_get_matches_from([
            "rss2pan",
            "magnet",
            "--cid",
            "21345",
            "--link",
            "magnet:?xt=urn:btih:12345",
        ])
        .unwrap();
    match matches.subcommand() {
        Some(("magnet", matches)) => {
            assert_eq!(
                matches.get_one::<String>("cid").cloned(),
                Some("21345".to_string())
            );
            assert_eq!(
                matches.get_one::<String>("link").cloned(),
                Some("magnet:?xt=urn:btih:12345".to_string())
            );
        }
        _ => panic!("subcommand not found"),
    }
    let matches = cmd
        .clone()
        .try_get_matches_from(["rss2pan", "magnet", "--txt", "magnet.txt"])
        .unwrap();
    match matches.subcommand() {
        Some(("magnet", matches)) => {
            assert_eq!(
                matches.get_one::<PathBuf>("txt").cloned(),
                Some(PathBuf::from("magnet.txt"))
            );
        }
        _ => panic!("subcommand not found"),
    }
}
