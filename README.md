<div align="center">

# Musd

[中文说明](README.zh-CN.md)

[![Release](https://github.com/hustcer/musd/actions/workflows/release.yaml/badge.svg)](https://github.com/hustcer/musd/actions/workflows/release.yaml)

🎵 A Rust CLI App to download super high quality musics🎵

![](https://img.alicdn.com/imgextra/i1/O1CN01oWvzdy1xIPUyZEyAK_!!6000000006420-1-tps-1964-878.gif)

</div>

`musd` is a MUSic Downloader, and that's why it was named as `musd`.

## Installation

You need to have `cargo` been installed, if you haven't installed it yet here is a [guide](https://www.rust-lang.org/tools/install) for you. (Rust v1.60.0 or above required)

```bash
# Install from crates.io
cargo install musd
# OR clone the source code and install it from local disk
git clone git@github.com:hustcer/musd.git
cd musd && cargo install --path .
```

## Usage

```bash
# Search music by name
musd someone like you
# Or search music by singer name
musd Celine Dion
```

### Save to a specified folder

1. Use `--output` or `-o` to specify the directory that downloaded musics will be saved;
2. Set `MUSD_OUTPUT` env variable to the path where you want to have your musics been saved;

## Resource

- All the musics will be downloaded from [MIGU](https://music.migu.cn/)

## PS

By default, the highest quality lossless music will be downloaded, usually in `flac` format.

This app was heavily inspired by [musicn](https://github.com/zonemeen/musicn), special thanks to them.
`musicn` was written in Node.js and I rewrote it by rust just for practice. And this is my first Rust App.
Hope you love it.

