# Rss2pan

Work in process

将 Rss 订阅离线下载到网盘。目前支持 115

## 关于

之前使用 Nodejs 写的工具[blog/rss2pan.ts](https://github.com/22earth/blog/blob/master/demos/test-node/src/bin/rss2pan.ts)

正在尝试用 Rust 实现一遍。

TODO

- [x] 115 离线功能
- [ ] 初始化 sqlite 数据
- [x] 实现 cli
- [x] proxy 配置
  - ~~目前写死在 build_proxy_client 里面~~
  - 读取 ALL_PROXY 或者 HTTPS_PROXY 环境变量

## 配置文件 rss.json

```json
{
  "mikanani.me": [
    {
      "name": "test",
      "filter": "简体内嵌",
      "url": "https://mikanani.me/RSS/Bangumi?bangumiId=2739&subgroupid=12"
    }
  ]
}
```

## node-site-config.json

配置示例

```json
{
  "115.com": {},
  "share.dmhy.org": {
    "httpsAgent": "httpsAgent"
  },
  "nyaa.si": {
    "httpsAgent": "httpsAgent"
  },
  "sukebei.nyaa.si": {
    "httpsAgent": "httpsAgent"
  },
  "mikanani.me": {
    "httpsAgent": "httpsAgent",
    "headers": {
      "Referer": "https://mikanani.me/"
    }
  }
}

```

如果没有在 headers 设置 "cookie": "xxx"。会自动读取 Chrome 的 cookie。

设置【httsAgent】会默认使用代理。默认使用的地址 `http://127.0.0.1:10809`。

需要自定义代理时，在命令行设置  set ALL_PROXY=http://youraddr:port
