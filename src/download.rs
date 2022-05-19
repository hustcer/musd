/**
 * Author: hustcer
 * Created: 2022/05/17 13:52:00
 * Description: Music download helper
 */
use crate::def::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use url::Url;
use yansi::Paint;

/**
 * Download music file and save to local disk
 */
#[tokio::main]
pub async fn download_music(song: &Song, args: &Args) -> MusdResult<()> {
    let dest_dir = get_dest_directory(args)?;
    let download_url = get_download_url(song)?;
    let download_src = download_url.to_string();
    // println!("{}", &download_url);
    // Get music file extension from download URI
    let extension = get_file_extension(&download_url);

    let dest_file = format!("{}-{}.{extension}", song.name, song.singers[0].name);
    let dest_path = dest_dir.join(&dest_file);
    // Check file existence, stop downloading if already exists.
    if Path::new(&dest_path).exists() {
        println!(
            "File {} already exists, stop downloading, bye...",
            Paint::green(&dest_path.to_string_lossy())
        );
        std::process::exit(0);
    }

    // Reqwest setup
    let res = reqwest::get(download_url).await?;
    let total_size = res
        .content_length()
        .ok_or_else(|| MusdError::GetLengthFailed(String::from(&download_src)))
        .expect("Get content length failed!");

    println!(
        "Start downloading the music and will be saved to `{}`!",
        Paint::green(&dest_dir.display()).bold()
    );

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .expect("Progress bar rendering failed!")
        .progress_chars("# >-")); // "█ >-" OR "#>-"

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let mut dest = {
        println!("The music to download: {:?}", Paint::green(&dest_file));
        println!("Will be located under: {:?}", Paint::green(&dest_path));
        File::create(dest_path)?
    };
    while let Some(item) = stream.next().await {
        let chunk = item?;
        let _ = dest
            .write_all(&chunk)
            .map_err(|_| "Error while writing to file");
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded to {}", dest_file));
    Ok(())
}

/**
 * Get music download url
 */
fn get_download_url(song: &Song) -> MusdResult<Url> {
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
    let result = download_url.set_scheme(MIGU_DOWNLOAD_SCHEME);
    assert!(result.is_ok());
    let result = download_url.set_host(Some(MIGU_DOWNLOAD_HOST));
    assert!(result.is_ok());
    Ok(download_url)
}

/**
 * Get music file extension
 */
fn get_file_extension(download_url: &Url) -> &str {
    let fname = download_url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("music-tmp.flac");

    Path::new(fname)
        .extension()
        .expect("Get music file extension error!")
        .to_str()
        .expect("Convert music file extension error!")
}

/**
 * Get dest directory to save the downloaded music files
 * 1. Use current directory by default;
 * 2. If `MUSD_OUTPUT` was set and the value as a directory exists, then use it;
 * 3. Finally, `--output(-o)` option will has the hightest priority if it exists;
 */
fn get_dest_directory(args: &Args) -> MusdResult<PathBuf> {
    // println!("{:?}", args);
    let output_env_key = MUSD_OUTPUT_ENV_KEY;
    if let Some(path) = &args.output {
        return Ok(path.to_path_buf());
    }
    // If no `--output(-o)` option specified use current directory by default
    let path = std::env::current_dir()?;
    match std::env::var(output_env_key) {
        Ok(val) if Path::new(&val).exists() => Ok(Path::new(&val).to_path_buf()),
        Err(_) => Ok(path),
        _ => Ok(path),
    }
}
