use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.0.0")
        .author("imMohika")
        .about("Rust version of echo")
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

    print!("{}", text.join(" "));

    if !omit_newline {
        print!("\n");
    }
}
