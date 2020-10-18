use clap::{App, Arg};
use std::{env, io};

mod data;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> io::Result<()> {
    let matches = App::new(NAME)
        .version(VERSION)
        .about("DIY Git")
        .arg(
            Arg::new("init")
                .about("Initialize ugit repository")
                .index(1),
        )
        .get_matches();

    if let Some(_) = matches.value_of("init") {
        init()?;
        println!(
            "Initialized empty ugit repository in {}/{}",
            env::current_dir()?.display(),
            data::GIT_DIR
        );
    }

    Ok(())
}

fn init() -> io::Result<()> {
    data::init()
}
