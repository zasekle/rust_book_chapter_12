use std::fs;
use std::error::Error;

//`()` is the Unit type in Rust. The `dyn` keyword is short of dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let word_index = contents.find(config.query.as_str());

    match word_index {
        None => println!("Word not found!"),
        Some(_) => println!("Word has been found in file!"),
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
