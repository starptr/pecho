#[allow(dead_code)]
#[allow(unused_imports)]
use std::env;
use colored::*;
use clap::{Arg, App, SubCommand};

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg_matches = App::new("pecho")
        .version("0.0.1")
        .author("Yuto Nishida")
        .about("Painted echo, or echo with easy colors.")
        .arg(Arg::with_name("input")
            .help("The string to print")
            .index(1))
        .arg(Arg::with_name("newline")
            .short("n")
            .help("No newline at the end."))
        .arg(Arg::with_name("truecolor")
            .help("Hex color in xxxxxx format.")
            .short("t")
            .long("truecolor")
            .takes_value(true)
            .value_name("hex"))
        .get_matches();
}
