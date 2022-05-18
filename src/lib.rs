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
 *  7. https://docs.rs/url/latest/url/index.html
 *  8. https://github.com/mihaigalos/tutorials/tree/master/rust/download_with_progressbar
 *
 * TODO:
 *  [x] Search music;
 *  [x] List music searching results, include name, singer and size, etc.;
 *  [x] Select the music(s) to be downloaded;
 *  [x] Notify user if there is no matching music；
 *  [x] Colored output to user terminal；
 *  [x] Stop downloading if music file already exists；
 *  [x] Add just dev tasks: fmt, clippy, run, release;
 *  [x] Add CHANGELOG.md;
 *  [x] Download the selected music;
 *  [x] Show progress bar while downloading;
 *  [ ] Customizable output path by using ENV var;
 *  [ ] Customizable output path by using `--output(-o)` option;
 *  [ ] Add README.md;
 *  [ ] Download multiple files at one time;
 *  [ ] Extract configs;
 *  [ ] Modular refactoring;
 *  [ ] CI tests;
 *  [ ] I18n output?;
 *  [ ] Make sure that it works on Mac & Windows;
 *  [ ] Handling `Ctrl + c` for Windows
 *  [ ] Add github release workflow to release binary for mac/win/linux
 */
mod def;
mod download;

pub use def::*;

use crate::def::{MusdResult, Song};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use serde_json::Value;
use url::Url;
use yansi::Paint;

/**
 * Search music by music or singer name
 */
#[tokio::main]
pub async fn search(search: &str) -> MusdResult<Vec<Song>> {
    let mut url = Url::parse("https://pd.musicapp.migu.cn/MIGUM3.0/v1.0/content/search_all.do")?;
    url.query_pairs_mut()
        .append_pair("text", search)
        .append_pair("pageSize", "1")
        .append_pair("searchSwitch", "{song:1}");
    // println!("Current search url {}", url.as_str());
    let resp = reqwest::get(url.as_str()).await?.text().await?;
    let val: Value = serde_json::from_str(&resp)?;
    let result = &val["songResultData"]["result"];

    // Handling no searching result cases
    #[allow(clippy::cmp_owned)]
    if result.to_string() == "null" {
        println!(
            "Can not find {} related musics! Bye ...",
            Paint::green(search).bold()
        );
        std::process::exit(0);
    }
    // let songs = val["songResultData"]["result"].as_array().unwrap();
    let songs = Vec::<Song>::deserialize(result)?;
    // println!("{:#?}", songs);
    Ok(songs)
}

/**
 * Show music selector and press `enter` or `space` to download it
 */
pub fn download_selected(songs: Vec<Song>) -> MusdResult<()> {
    // Filter songs that have super quality
    let sq_songs = songs
        .iter()
        .filter(|s| s.new_rate_formats.iter().any(|f| f.format_type == "SQ"))
        .collect::<Vec<_>>();

    // Select one music a time, we can use multiple select later
    let selection = select_music(sq_songs.clone());

    // println!("You select {:#?}", sq_songs[selection]);
    // println!("Start to download {}!", Paint::green(&selections[selection]));

    if let Err(e) = download::download_music(sq_songs[selection]) {
        eprintln!("[ERROR]: {}", Paint::red(e));
        std::process::exit(2);
    }
    Ok(())
}

/**
 * Show music selector with music name, singer and size included
 */
fn select_music(sq_songs: Vec<&Song>) -> usize {
    // Prepare music list for user selecting
    let selections = sq_songs
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let sq_format = s
                .new_rate_formats
                .iter()
                .filter(|f| f.format_type == "SQ")
                .collect::<Vec<_>>();

            // println!("{:#?}", sq_format);
            let idx = i + 1;
            let size = sq_format[0]
                .size
                .parse::<u32>()
                .expect("Music file size parsing error!");
            let size_mb: f32 = (size as f32) / 1024.0 / 1024.0;

            format!("{idx:2}. {} -{} -- {:.2} MB", s.name, s.singers[0], size_mb)
        })
        .collect::<Vec<_>>();

    let prompt = format!(
        "Find {} results with super high quality, Press `enter` or `space` key to download...",
        selections.len()
    );

    // Select one music a time, we can use multiple select later
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("Music selection failed!")
}
