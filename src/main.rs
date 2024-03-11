/**
 * Author: hustcer
 * Created: 2022/05/15 16:52:00
 * Description: A CLI App to search and download musics
 */
use clap::Parser;
use yansi::Paint;

fn main() {
    // Parsing user input args
    // 'music' values: Some(["someone", "like", "you"])
    // println!("'music' values: {:?}", args.music);
    let args = musd::Args::parse();
    // Disable coloring by `CLICOLOR` env variable
    if let Ok(true) = std::env::var("CLICOLOR").map(|v| v == "0") {
        yansi::disable();
    }

    if args.build_info {
        musd::build_info::show_info();
        std::process::exit(0);
    }

    if let Err(e) =
        musd::search(&args.music.join(" ")).and_then(|songs| musd::download_selected(songs, &args))
    {
        eprintln!("[ERROR]: {}", Paint::red(&e));
        std::process::exit(1);
    }
}
