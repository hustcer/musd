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
    #[error("download url parsing error")]
    UrlErr(#[from] url::ParseError),
    #[error("download target copy failed")]
    CopyFailed(#[from] std::io::Error),
    #[error("download by reqwest failed")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("response parsing failed")]
    JsonParseErr(#[from] serde_json::Error),
}

/// A CLI App to search and download musics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path you want to save the downloaded music to
    #[clap(short, long, default_value = ".")]
    pub output: String,

    /// The music or singer name to search for further downloading
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
