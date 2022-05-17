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

    let music = sq_format.get(0).unwrap();

    // println!("{:#?}", &music);
    let target = if music.android_url.is_empty() {
        &music.url
    } else {
        &music.android_url
    };
    let mut download_url = Url::parse(&target)?;
    let result = download_url.set_scheme("https");
    assert!(result.is_ok());
    let result = download_url.set_host(Some("freetyst.nf.migu.cn"));
    assert!(result.is_ok());
    // println!("{}", &download_url);

    let response = reqwest::get(download_url.as_str()).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("music-tmp.flac");

        let extension = Path::new(fname).extension().unwrap().to_str().unwrap();
        let dest = format!("{}-{}.{extension}", song.name, song.singers[0].name);
        println!("The music to download: '{:?}'", dest);
        let dest = path.join(dest);
        println!("Will be located under: '{:?}'", dest);
        File::create(dest)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
