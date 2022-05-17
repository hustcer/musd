/**
 * Author: hustcer
 * Created: 2022/05/17 13:52:00
 * Description: Music download helper
 */
use crate::def::Song;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use thiserror::Error;
use url::Url;
use yansi::Paint;

type MusdResult<T> = Result<T, MusdError>;

#[derive(Error, Debug)]
pub enum MusdError {
    #[error("download url parsing error")]
    UrlErr(#[from] url::ParseError),
    #[error("download target copy failed")]
    CopyFailed(#[from] std::io::Error),
    #[error("download by reqwest failed")]
    ReqwestErr(#[from] reqwest::Error),
}

#[tokio::main]
pub async fn download(song: &Song) -> MusdResult<()> {
    let path = std::env::current_dir()?;
    let sq_format = song
        .new_rate_formats
        .iter()
        .filter(|f| f.format_type == "SQ")
        .collect::<Vec<_>>();

    let music = sq_format
        .get(0)
        .expect("No super quality format available!");

    // println!("{:#?}", &music);
    let target = if music.android_url.is_empty() {
        &music.url
    } else {
        &music.android_url
    };
    // Use `https` download instead of `ftp`
    let mut download_url = Url::parse(target)?;
    let result = download_url.set_scheme("https");
    assert!(result.is_ok());
    let result = download_url.set_host(Some("freetyst.nf.migu.cn"));
    assert!(result.is_ok());
    // println!("{}", &download_url);
    // Get music file name from download URI
    let fname = download_url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("music-tmp.flac");
    let extension = Path::new(fname)
        .extension()
        .expect("Get music file extension error!")
        .to_str()
        .expect("Convert music file extension error!");

    let dest_file = format!("{}-{}.{extension}", song.name, song.singers[0].name);
    let dest_path = path.join(&dest_file);
    // Check file existence, stop downloading if already exists.
    if Path::new(&dest_path).exists() {
        println!(
            "File {} already exists, stop downloading, bye...",
            Paint::green(&dest_path.to_string_lossy())
        );
        std::process::exit(0);
    }

    let response = reqwest::get(download_url.as_str()).await?;

    let mut dest = {
        println!("The music to download: '{:?}'", Paint::green(&dest_file));
        println!("Will be located under: '{:?}'", Paint::green(&dest_path));
        File::create(dest_path)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
