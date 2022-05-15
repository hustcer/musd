// Note: this requires the `derive` feature

use clap::Parser;

/// A CLI App to search and download musics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path you want to save your music to
    #[clap(short, long, default_value = ".")]
    output: String,

    music: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("Start download the music and output to `{}`!", args.output);
    // 'music' values: Some(["sloppy", "slop", "slop"])
    println!("'music' values: {:?}", args.music);
}
