<div align="center">

# Musd

🎵 A Rust CLI App to download super high quality musics🎵

</div>

## Installation

You need to have `cargo` been installed, here is a guide for you: https://www.rust-lang.org/tools/install

```bash
# Install from crates.io
cargo install musd
# Or install from local source
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

1. Use `--output` or `-o` to specify the directory that downloaded music will be saved;
2. Set `MUSD_OUTPUT` env to the path where you want to have your musics been saved;

## Resource

- All the musics will be downloaded from [MIGU](https://music.migu.cn/)

## Thanks

This app was totally inspired by [musicn](https://github.com/zonemeen/musicn), special thanks to them.
`musicn` was written in Node.js and I rewrite it by rust just for practice. And this is my first Rust App.
Hope you love it.

