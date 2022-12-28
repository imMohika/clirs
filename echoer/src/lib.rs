use clap::{App, Arg};
use std::error::Error;

type CLIResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: Vec<String>,
    omit_newline: bool,
}

pub fn run(config: Config) -> CLIResult<()> {
    let Config { text, omit_newline } = config;

    print!("{}", text.join(" "));

    if !omit_newline {
        print!("\n");
    }
    Ok(())
}

pub fn get_args() -> CLIResult<Config> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Text to echo")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Don't print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    Ok(Config { text, omit_newline })
}
