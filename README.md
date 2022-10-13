# Rss2pan

将 Rss 订阅离线下载到网盘。目前支持 115

## 关于

之前使用 Nodejs 写的工具[blog/rss2pan.ts](https://github.com/22earth/blog/blob/master/demos/test-node/src/bin/rss2pan.ts)

正在尝试用 Rust 实现一遍。

支持 Rss 源: nyaa, dmhy, mikanni

- [x] 115 离线功能
- [x] sqlite 存储数据
- [x] 实现 cli
- [x] proxy 配置
  - ~~目前写死在 build_proxy_client 里面~~
  - 读取 ALL_PROXY 或者 HTTPS_PROXY 环境变量
- [ ] 正则过滤 filter

## 用法

在同一目录下面，配置好 `rss.json` 和 `node-site-config.json`

在命令行运行 `rss2pan`

```bash
# 查看帮助
rss2pan -h
# 读取和使用 Edge 的 cookie
rss2pan -c Edge

# 指定 rss URL 离线下载
# 如果 rss.json 存在这条url 的配置，会读取配置。没有配置，默认离线到 115 的默认目录
rss2pan -u "https://mikanani.me/RSS/Bangumi?bangumiId=2739&subgroupid=12"
```

## Examples

## 配置文件 rss.json

```json
{
  "mikanani.me": [
    {
      "name": "test",
      "filter": "简体内嵌",
      "url": "https://mikanani.me/RSS/Bangumi?bangumiId=2739&subgroupid=12"
    }
  ],
  "share.dmhy.org": [
    {
      "name": "水星的魔女",
      "filter": "简日双语",
      "cid: "2479224057885794455",
      "url": "https://share.dmhy.org/topics/rss/rss.xml?keyword=%E6%B0%B4%E6%98%9F%E7%9A%84%E9%AD%94%E5%A5%B3&sort_id=2&team_id=0&order=date-desc"
    }
  ]
}
```

配置了 `filter` 后，标题包含该文字的会被离线
cid 是离线到指定的文件夹的id 。
获取方法: 浏览器打开115的文件，地址栏像 `https://115.com/?cid=2479224057885794455&offset=0&tab=&mode=wangpan`

> 其中 2479224057885794455 就是 cid

## node-site-config.json

配置示例

```json
{
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

Windows 下 如果设置了 headers, 但是没在headers里面设置 "cookie": "xxx"。会自动读取命令行指定浏览器的 cookie。默认使用 Chrome

设置【httsAgent】会使用代理。默认使用的地址 `http://127.0.0.1:10809`。

需要自定义代理时，在命令行设置 Windows: set ALL_PROXY=http://youraddr:port

> Linux: export ALL_PROXY=http://youraddr:port
