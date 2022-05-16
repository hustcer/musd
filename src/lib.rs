/**
 * Author: hustcer
 * Created: 2022/05/15 16:52:00
 * Description: A CLI App to search and download musics
 * REFS:
 *  1. https://serde.rs/
 *  2. https://rust-lang-nursery.github.io/rust-cookbook/
 *  3. The map trait on serde_json::Value: https://github.com/serde-rs/json/issues/484
 *  4. Iterating through an serde_json::Value: https://github.com/serde-rs/json/issues/417
 *  5. https://pd.musicapp.migu.cn/MIGUM3.0/v1.0/content/search_all.do?text=传奇&pageSize=1&searchSwitch={song:1}
 *  6. https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
 *
 * TODO:
 *  [x] 搜索歌曲;
 *  [x] 以表格形式显示歌曲搜索列表含名称、歌手、大小、格式等信息;
 *  [x] 选择歌曲;
 *  [ ] 下载歌曲并显示下载进度;
 *  [ ] 配置分离;
 *  [ ] 代码模块化重构;
 *  [ ] 集成测试;
 *  [ ] 允许通过环境变量和 `--output(-o)` 配置歌曲下载路径;
 *  [ ] 国际化文案显示;
 *  [ ] 确保 Mac & Windows 下均可使用;
 */
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use url::Url;

type MusdResult<T> = Result<T, Box<dyn Error>>;

/// A CLI App to search and download musics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path you want to save the downloaded music to
    #[clap(short, long, default_value = ".")]
    pub output: String,

    /// The music or singer name to search for future downloading
    pub music: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    name: String,
    singers: Vec<Singer>,
    new_rate_formats: Vec<MusicFormat>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Singer {
    id: String,
    name: String,
}

impl std::fmt::Display for Singer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.name)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MusicFormat {
    format_type: String,

    #[serde(default = "default_string")]
    url: String,

    #[serde(default = "default_string")]
    ios_url: String,

    #[serde(default = "default_string")]
    android_url: String,

    #[serde(default = "default_size")]
    size: String,

    #[serde(default = "default_string")]
    file_type: String,
}

fn default_string() -> String {
    "".to_string()
}

fn default_size() -> String {
    "0".to_string()
}

#[tokio::main]
pub async fn search(search: &str) -> MusdResult<Vec<Song>> {
    let mut url = Url::parse("https://pd.musicapp.migu.cn/MIGUM3.0/v1.0/content/search_all.do")?;
    let mut query = format!("text={0}&pageSize=1", search);
    query.push_str("&searchSwitch={song:1}");
    url.set_query(Some(&query));
    // println!("Current search url {}", url.as_str());
    let resp = reqwest::get(url.as_str()).await?.text().await?;
    let val: Value = serde_json::from_str(&resp)?;
    // let songs = val["songResultData"]["result"].as_array().unwrap();
    let songs = Vec::<Song>::deserialize(&val["songResultData"]["result"])?;
    // println!("{:#?}", songs);
    Ok(songs)
}

// Show music selector and press `enter` to download it
pub fn choose_music(songs: Vec<Song>) -> MusdResult<()> {
    // Filter songs that have super quality
    let sq_songs = songs
        .iter()
        .filter(|s| s.new_rate_formats.iter().any(|f| f.format_type == "SQ"))
        .collect::<Vec<_>>();
    let selections = sq_songs
        .iter()
        .map(|s| {
            let sq_format = s
                .new_rate_formats
                .iter()
                .filter(|f| f.format_type == "SQ")
                .collect::<Vec<_>>();

            println!("{:#?}", sq_format);
            let size = sq_format[0].size.parse::<u32>().unwrap();
            let size_mb: f32 = (size as f32) / 1024.0 / 1024.0;
            format!("{} -{} -- {:.2} MB", s.name, s.singers[0], size_mb)
        })
        .collect::<Vec<_>>();

    let prompt = format!(
        "Find {} results with high quality, Press `enter` or `space` key to download...",
        selections.len()
    );

    // Select one music a time, we can use multiple select later
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("You select {:#?}", sq_songs[selection]);
    println!("Start to download {}!", selections[selection]);
    Ok(())
}
