use std::env;
use std::io::Result;

use clap::{App, Arg};

mod data;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> Result<()> {
    let matches = App::new(NAME)
        .version(VERSION)
        .about("DIY Git")
        .subcommand(App::new("init").about("Initialize ugit repository"))
        .subcommand(
            App::new("hash-object")
                .about("Save a file in ugit object database")
                .arg(Arg::new("pathspec").takes_value(true).multiple(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            init()?;
        }
        Some(("hash-object", hash_object_matches)) => {
            let pathspecs = hash_object_matches.values_of("pathspec").unwrap().collect::<Vec<_>>();
            hash_object(pathspecs)?;
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    Ok(())
}

fn init() -> Result<()> {
    let retval = data::init();
    println!(
        "Initialized empty ugit repository in {}/{}",
        env::current_dir()?.display(),
        data::GIT_DIR
    );
    retval
}

fn hash_object(pathspecs: Vec<&str>) -> Result<()> {
    data::hash_object(pathspecs)
}
