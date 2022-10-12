use anyhow::Result;
use reqwest::{header::HeaderMap, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{request::Ajax, AJAX_INSTANCE};

type FormData = HashMap<String, String>;

pub struct Yiyiwu;

#[derive(Serialize, Deserialize)]
struct Sign {
    sign: String,
    time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub errcode: u32,
    pub errno: Option<u32>,
    // pub errtype: String,
    pub state: Option<bool>,
    pub error_msg: Option<String>,
}

impl Yiyiwu {
    fn gen_req(&self, method: Method, url: &str) -> RequestBuilder {
        let ajax = AJAX_INSTANCE.get_or_init(|| Ajax::new());
        ajax.gen_req_host(method, url, "115.com").header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.61 Safari/537.36 115Browser/23.9.3.6"
        )
    }

    async fn get_sign(&self) -> Result<Sign> {
        let res: Sign = self
            .gen_req(Method::GET, "https://115.com/?ct=offline&ac=space")
            .header("Accept", "application/json, text/javascript, */*; q=0.01")
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
    async fn post_data(&self, url: &str, data: FormData) -> Result<Response> {
        let data: FormData = {
            let mut data: FormData = data;
            let sign = self.get_sign().await?;
            data.insert("sign".to_string(), sign.sign);
            data.insert("time".to_string(), sign.time.to_string());

            data
        };
        let mut headers = HeaderMap::new();
        headers.append(
            "Accept",
            "application/json, text/javascript, */*; q=0.01"
                .parse()
                .unwrap(),
        );
        headers.append("X-Requested-With", "XMLHttpRequest".parse()?);
        Ok(self
            .gen_req(Method::POST, url)
            .headers(headers)
            .form(&data)
            .send()
            .await?
            .json()
            .await?)
    }
    // 任务已存在  errcode 0; result 里面有存在的任务信息
    pub async fn add_batch_task(&self, tasks: &[&str], cid: Option<String>) -> Result<Response> {
        if tasks.len() == 1 {
            return self.add_task_url(tasks[0], cid).await;
        }
        let mut data: FormData = HashMap::new();
        if let Some(cid) = cid {
            data.insert("wp_path_id".to_string(), cid);
        }
        for (i, task) in tasks.iter().enumerate() {
            data.insert(format!("url[{}]", i), task.to_string());
        }

        let res = self
            .post_data(
                "https://115.com/web/lixian/?ct=lixian&ac=add_task_urls",
                data,
            )
            .await?;

        Ok(res)
    }
    pub async fn add_task_url(&self, task: &str, cid: Option<String>) -> Result<Response> {
        let mut data: FormData = HashMap::new();
        data.insert("url".to_string(), task.to_string());
        if let Some(cid) = cid {
            data.insert("wp_path_id".to_string(), cid);
        }

        let url = "https://115.com/web/lixian/?ct=lixian&ac=add_task_url";
        let res = self.post_data(url, data).await?;

        Ok(res)
    }
    pub async fn get_upload_info(&self) -> Result<Value> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append("host", "115.com".parse().unwrap());
        Ok(self
            .gen_req(Method::GET, "https://proapi.115.com/app/uploadinfo")
            .send()
            .await?
            .json()
            .await?)
    }
    pub async fn is_logged(&self) -> bool {
        let s = self.get_upload_info().await.unwrap();
        let n = s["errno"].as_u64().unwrap();
        n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn t_get_sign() {
        let yiyiwu = Yiyiwu;
        let s = yiyiwu.get_sign().await;
        assert!(s.is_ok());
    }
    #[tokio::test]
    async fn t_get_upload_info() {
        let yiyiwu = Yiyiwu;
        let s = yiyiwu.get_upload_info().await;
        assert!(s.is_ok());
        let s = s.unwrap();
        let _n = s["errno"].as_u64().unwrap();
    }
    #[tokio::test]
    async fn t_add_url() {
        let yiyiwu = Yiyiwu;
        let s = yiyiwu
            .add_task_url(
                "magnet:?xt=urn:btih:e6bd034f77af87ccfe062acbf481d34afe089133",
                None,
            )
            .await;
        assert!(s.is_ok());
        let s = s.unwrap();
        println!("{:?}", s);
    }
}
