use std::{fs, io};

pub fn run(config: Config) -> io::Result<()> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With test:\n{contents}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}
