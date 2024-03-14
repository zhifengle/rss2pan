use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn get_magnet_list_by_txt(txt: &PathBuf) -> anyhow::Result<Vec<String>> {
    let mut magnet_list = Vec::new();
    let file = File::open(txt)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("magnet:")
            || line.starts_with("https://")
            || line.starts_with("http://")
            || line.starts_with("ftp://")
            || line.starts_with("ftps://")
        {
            magnet_list.push(line.trim_end().to_string());
        }
    }
    Ok(magnet_list)
}
