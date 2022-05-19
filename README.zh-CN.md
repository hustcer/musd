<div align="center">

# Musd

![ci](https://img.shields.io/github/workflow/status/hustcer/musd/ci)

🎵 一个可以下载高品质音乐的 Rust CLI 工具

![](https://img.alicdn.com/imgextra/i2/O1CN01xGrmUJ1ncQeYjvDcj_!!6000000005110-1-tps-1964-878.gif)

</div>

`musd` 是一个音乐下载器，也是 MUSic Downloader 的简称。

## 安装

如果您尚未安装 `cargo` 请根据这个[指南](https://www.rust-lang.org/tools/install) 安装 rust 相关工具链。

```bash
# 从 crates.io 安装
cargo install musd
# 或者，你也可以将代码克隆到本地然后从本地安装，此方式可以安装尚未发布的最新版本
git clone git@github.com:hustcer/musd.git
cd musd && cargo install --path .
```

## 使用

```bash
# 通过音乐名称搜索
musd 传奇
# 通过歌手名称搜索
musd 单依纯
```

### 将音乐保存到指定路径

1. 通过 `--output` 或者 `-o` 参数可以指定音乐下载后的存储路径，该参数具有最高优先级；
2. 也可以通过 `MUSD_OUTPUT` 环境变量指定默认的音乐存储路径;

## 音乐来源

- 目前所有音乐都来自于 [咪咕](https://music.migu.cn/)

## 其他说明

本工具默认会下载最高品质的无损音乐，通常为 `flac` 格式；

本命令行工具主要受 [musicn](https://github.com/zonemeen/musicn) 的启发，在此表示感谢，`musicn` 是通过 Node.js 写的，本人用 Rust 重写了一遍。作为学习 Rust 后的第一个练手应用，希望你能喜欢。

