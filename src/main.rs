use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
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
        init();
    }
}

fn init() {
    println!("Init");
}
