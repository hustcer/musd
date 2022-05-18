/**
 * Author: hustcer
 * Created: 2022/05/17 13:52:00
 * Description: Commonly used structs
 */
use clap::Parser;
use serde::Deserialize;
use thiserror::Error;

pub type MusdResult<T> = Result<T, MusdError>;

#[derive(Error, Debug)]
pub enum MusdError {
    #[error("Download url parsing error")]
    UrlErr(#[from] url::ParseError),
    #[error("Dest file creation failed")]
    CreationErr(#[from] std::io::Error),
    #[error("Download by reqwest failed")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("Searching response parsing failed")]
    JsonParseErr(#[from] serde_json::Error),
    #[error("Failed to get content length from `{0}`")]
    GetLengthFailed(String),
}

/**
 * REF: https://github.com/clap-rs/clap/blob/v3.1.18/examples/derive_ref/README.md
 */
/// A CLI App to search and download musics
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, arg_required_else_help(true))]
pub struct Args {
    /// The path to save the downloaded music, current directory by default, and you can use `MUSD_OUTPUT` env to set the default output path too
    #[clap(short, long, parse(from_os_str), value_name = "PATH")]
    pub output: Option<std::path::PathBuf>,

    /// The music or singer name to search for further downloading
    #[clap(value_name = "MUSIC_OR_SINGER")]
    pub music: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub name: String,
    pub singers: Vec<Singer>,
    pub new_rate_formats: Vec<MusicFormat>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Singer {
    pub id: String,
    pub name: String,
}

impl std::fmt::Display for Singer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.name)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFormat {
    pub format_type: String,

    #[serde(default = "default_string")]
    pub url: String,

    #[serde(default = "default_string")]
    pub ios_url: String,

    #[serde(default = "default_string")]
    pub android_url: String,

    #[serde(default = "default_size")]
    pub size: String,

    #[serde(default = "default_string")]
    pub file_type: String,
}

fn default_string() -> String {
    "".to_string()
}

fn default_size() -> String {
    "0".to_string()
}
