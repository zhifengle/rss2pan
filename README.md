# Rss2pan

将 Rss 订阅离线下载到 115 网盘。

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
- [x] 正则过滤 filter
- [ ] Windows 定时任务
  - ~~懒得写了，我是手动配置的~~
- [x] 不同网站的并发任务

## 用法

在同一目录下面，配置好 `rss.json` 和 `node-site-config.json`

在命令行运行 `rss2pan`

```bash
# 查看帮助
rss2pan -h
# 直接运行。读取 rss.json，依次添加离线任务
rss2pan
# 并发请求 rss 网站。然后再添加 115 离线任务
rss2pan -m
# 读取和使用 Edge 的 cookie
rss2pan -c Edge

# 指定 rss URL 离线下载
# 如果 rss.json 存在这条url 的配置，会读取配置。没有配置，默认离线到 115 的默认目录
rss2pan -u "https://mikanani.me/RSS/Bangumi?bangumiId=2739&subgroupid=12"
```

### 注意

日志报 `115 abnoraml operation` 时，说明账号触发了异常验证，需要在浏览器端手动离线，输入验证码后解除。

暂时没有想到好的方法处理。现在想到的方式打印一个 URL，打开这个 URL，配合油猴脚本触发验证码。

## 配置

### 配置文件 rss.json

```json
{
  "mikanani.me": [
    {
      "name": "test",
      "filter": "/简体|1080p/",
      "url": "https://mikanani.me/RSS/Bangumi?bangumiId=2739&subgroupid=12"
    }
  ],
  "nyaa.si": [
    {
      "name": "VCB-Studio",
      "cid: "2479224057885794455",
      "url": "https://nyaa.si/?page=rss&u=VCB-Studio"
    }
  ],
  "sukebei.nyaa.si": [
    {
      "name": "hikiko123",
      "cid: "2479224057885794455",
      "url": "https://sukebei.nyaa.si/?page=rss&u=hikiko123"
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

配置了 `filter` 后，标题包含该文字的会被离线。不设置 `filter` 默认离线全部

`/简体|\\d{3-4}[pP]/` 使用斜线包裹的正则规则。注意转义规则

cid 是离线到指定的文件夹的 id 。
获取方法: 浏览器打开 115 的文件，地址栏像 `https://115.com/?cid=2479224057885794455&offset=0&tab=&mode=wangpan`

> 其中 2479224057885794455 就是 cid

### node-site-config.json 配置

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

#### cookie 配置

Windows 下 如果设置了 headers, 但是没在 headers 里面设置 "cookie": "xxx"。会自动读取命令行指定浏览器的 cookie。默认使用 Chrome

Linux 下使用，必须配置 115 的 cookie。或者指定 Firefox 目录读取 cookie(这项功能我没测试)

```json
{
  "115.com": {
    "headers": {
      "cookie": "yourcookie"
    }
  }
}
```

### proxy 配置

设置【httsAgent】会使用代理。默认使用的地址 `http://127.0.0.1:10809`。

> 【httsAgent】沿用的 node 版的配置。

需要自定义代理时，在命令行设置 Windows: set ALL_PROXY=http://youraddr:port

> Linux: export ALL_PROXY=http://youraddr:port

```batch
@ECHO off
SETLOCAL
CALL :find_dp0
REM set ALL_PROXY=http://youraddr:port
rss2pan.exe  %*
ENDLOCAL
EXIT /b %errorlevel%
:find_dp0
SET dp0=%~dp0
EXIT /b
```

把上面的 batch 例子改成自己的代理地址。另存为 rss2pan.cmd 和 rss2pan.exe 放在一个目录下面。

在命令行运行 rss2pan.cmd 就能够使用自己的代理的了。

### 日志的环境变量

不想看日志时，Windows: set RUST_LOG=error
