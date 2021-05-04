#![allow(dead_code)]
#![allow(unused_imports)]
mod utils;
use utils::*;
use clap::{App, Arg, SubCommand};
use colored::*;
use std::env;

fn main() {
    let arg_matches = App::new("pecho")
        .version("0.0.1")
        .author("Yuto Nishida")
        .about("Painted echo: Echo with easy colors.")
        .arg(
            Arg::with_name("input")
                .help("The string to print")
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("noNewline")
                .short("n")
                .help("No newline at the end"),
        )
        .arg(
            Arg::with_name("noEscapes")
                .short("E")
                .help("Treat backslashes literally"),
        )
        .arg(Arg::with_name("black").short("k").long("black"))
        .arg(Arg::with_name("red").short("r").long("red"))
        .arg(Arg::with_name("green").short("g").long("green"))
        .arg(Arg::with_name("yellow").short("y").long("yellow"))
        .arg(Arg::with_name("blue").short("b").long("blue"))
        .arg(Arg::with_name("purple").short("p").long("purple"))
        .arg(Arg::with_name("cyan").short("q").long("cyan"))
        .arg(Arg::with_name("white").short("w").long("white"))
        .arg(Arg::with_name("blackBg").short("K").long("black-bg"))
        .arg(Arg::with_name("redBg").short("R").long("red-bg"))
        .arg(Arg::with_name("greenBg").short("G").long("green-bg"))
        .arg(Arg::with_name("yellowBg").short("Y").long("yellow-bg"))
        .arg(Arg::with_name("blueBg").short("B").long("blue-bg"))
        .arg(Arg::with_name("purpleBg").short("P").long("purple-bg"))
        .arg(Arg::with_name("cyanBg").short("Q").long("cyan-bg"))
        .arg(Arg::with_name("whiteBg").short("W").long("white-bg"))
        .arg(
            Arg::with_name("bright")
                .help("Use the bright variant")
                .short("l")
                .long("bright"),
        )
        .arg(
            Arg::with_name("brightBg")
                .help("Use the bright background variant")
                .short("L")
                .long("bright-bg"),
        )
        .arg(
                Arg::with_name("color")
                    .help("Specify color using an argument. Overrides single color options")
                    .short("c")
                    .long("color")
                    .takes_value(true)
            )
        .arg(
                Arg::with_name("colorBg")
                    .help("Specify background color using an argument. Overrides single color options")
                    .short("C")
                    .long("color-bg")
                    .takes_value(true)
            )
        .arg(
            Arg::with_name("style")
                .help("Styling")
                .short("s")
                .long("style")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("truecolor")
                .help("Hex color in xxxxxx format. Overrides other color options")
                .short("t")
                .long("truecolor")
                .takes_value(true)
                .value_name("hex"),
        )
        .arg(
            Arg::with_name("truecolorBg")
                .help("Background in hex in xxxxxx format. Overrides other color options")
                .short("T")
                .long("truecolor-bg")
                .takes_value(true)
                .value_name("hex"),
        )
        .get_matches();

    // Concatenate input into space-separated words
    let input = args_to_input(arg_matches.values_of("input"));

    // Replace escaped special characters and add trailing newline if necessary
    let std_print_string = special_chars_and_newlines(input, arg_matches.is_present("noEscapes"), arg_matches.is_present("noNewline"));

    let std_print_string = add_color(std_print_string, &arg_matches, false);

    let std_print_string = add_color(std_print_string.to_string(), &arg_matches, true);

    let std_print_string = add_style(std_print_string, &arg_matches);

    print!("{}", std_print_string);
}
