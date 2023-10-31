use std::collections::HashMap;

mod crypto;

pub fn gen_offline_params(
    uid: String,
    app_ver: String,
    urls: Vec<String>,
    cid: Option<String>,
) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("uid".to_string(), uid);
    params.insert("app_ver".to_string(), app_ver);
    params.insert("ac".to_string(), "add_task_urls".to_string());
    for (i, task) in urls.iter().enumerate() {
        params.insert(format!("url[{}]", i), task.to_string());
    }

    if let Some(cid) = cid {
        params.insert("wp_path_id".to_string(), cid);
    }

    params
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_encode() {
    }
}
