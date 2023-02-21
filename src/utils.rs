use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};

pub fn get_magnet_list_by_txt(txt: &PathBuf) -> anyhow::Result<Vec<String>> {
    let mut magnet_list = Vec::new();
    let file = File::open(txt)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("magnet:?xt=urn:btih:") {
            magnet_list.push(line.trim_end().to_string());
        }
    }
    Ok(magnet_list)
}
