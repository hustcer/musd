/**
 * Author: hustcer
 * Created: 2022/05/15 16:52:00
 * Description: A CLI App to search and download musics
 */
use clap::Parser;

fn main() {
    let args = musd::Args::parse();
    println!("Start download the music and output to `{}`!", args.output);
    // 'music' values: Some(["someone", "like", "you"])
    println!("'music' values: {:?}", args.music);

    if let Err(e) = musd::search(&args.music.join(" ")).and_then(musd::choose_music) {
        eprintln!("[ERROR]: {}", e);
        std::process::exit(1);
    }
}
