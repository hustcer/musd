<div align="center">

# Musd

[中文说明](README.zh-CN.md)

![build](https://img.shields.io/github/workflow/status/hustcer/musd/ci)

🎵 A Rust CLI App to download super high quality musics🎵

</div>

`musd` is a MUSic Downloader, and that's why it was named as `musd`.

## Installation

You need to have `cargo` been installed, if you haven't installed it yet here is a [guide](https://www.rust-lang.org/tools/install) for you.

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

