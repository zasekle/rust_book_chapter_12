use std::env;
use std::process;

use rust_book_chapter_11::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg! is something for `quick and dirty debugging`. I should probably use this more.
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) =  rust_book_chapter_11::run(config) {
        println!("Application: error: {e}");
        process::exit(1);
    }
}

