/**
 * Author: hustcer
 * Created: 2022/05/15 16:52:00
 * Description: A CLI App to search and download musics
 */
use clap::Parser;
use yansi::Paint;

fn main() {
    // Parsing user input args
    let args = musd::Args::parse();
    // Disable coloring by `CLICOLOR` env variable
    if let Ok(true) = std::env::var("CLICOLOR").map(|v| v == "0") {
        Paint::disable();
    }
    println!(
        "Start downloading the music and will save to `{}`!",
        Paint::green(args.output).bold()
    );
    // 'music' values: Some(["someone", "like", "you"])
    // println!("'music' values: {:?}", args.music);

    if let Err(e) = musd::search(&args.music.join(" ")).and_then(musd::choose_music) {
        eprintln!("[ERROR]: {}", Paint::red(e));
        std::process::exit(1);
    }
}
