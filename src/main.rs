/**
 * Author: hustcer
 * Created: 2022/05/15 16:52:00
 * Description: A CLI App to search and download musics
 * REFS:
 *  1. https://serde.rs/
 *  2. The map trait on serde_json::Value: https://github.com/serde-rs/json/issues/484
 *  3. Iterating through an serde_json::Value: https://github.com/serde-rs/json/issues/417
 *  4. https://pd.musicapp.migu.cn/MIGUM3.0/v1.0/content/search_all.do?text=传奇&pageSize=1&searchSwitch={song:1}
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

/// A CLI App to search and download musics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path you want to save the downloaded music to
    #[clap(short, long, default_value = ".")]
    output: String,

    /// The music or singer name to search for future downloading
    music: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Song {
    name: String,
    singers: Vec<Singer>,
    new_rate_formats: Vec<MusicFormat>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Singer {
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

fn main() {
    let args = Args::parse();

    println!("Start download the music and output to `{}`!", args.output);
    // 'music' values: Some(["someone", "like", "you"])
    println!("'music' values: {:?}", args.music);
    show_select(&args.music.join(" "));
}

#[tokio::main]
async fn search(search: &str) -> Result<Vec<Song>, Box<dyn Error>> {
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

    // for item in songs.iter() {
    //     let sq_format = item.new_rate_formats.iter().filter(|f| f.format_type == "SQ").collect::<Vec<_>>();
    //     println!("{} -{}, {:#?}", item.name, item.singers[0], sq_format);
    // }
    Ok(songs)
}

// Show music selector and press `enter` to download it
fn show_select(query: &str) {
    let songs = search(query).unwrap();
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

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("You select {:#?}", sq_songs[selection]);
    println!("Start to download {}!", selections[selection]);
}
