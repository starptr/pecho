#![allow(dead_code)]
#![allow(unused_imports)]
use clap::{App, Arg, SubCommand};
use colored::*;
use lazy_static::*;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let arg_matches = App::new("pecho")
        .version("0.0.1")
        .author("Yuto Nishida")
        .about("Painted echo, or echo with easy colors.")
        .arg(
            Arg::with_name("input")
                .help("The string to print")
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("newline")
                .short("n")
                .help("No newline at the end"),
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
                .help("Hex color in xxxxxx format")
                .short("t")
                .long("truecolor")
                .takes_value(true)
                .value_name("hex"),
        )
        .arg(
            Arg::with_name("truecolorBg")
                .help("Background in hex in xxxxxx format")
                .short("T")
                .long("truecolor-bg")
                .takes_value(true)
                .value_name("hex"),
        )
        .get_matches();

    // Concatenate input into space-separated words
    let input = match arg_matches.values_of("input") {
        None => String::from(""),
        Some(mut values) => {
            let mut input = String::from(values.next().unwrap());
            for value in values {
                input.push(' ');
                input.push_str(value);
            }
            input
        }
    };

    println!("({})", input);

    // Replace escaped special characters and add trailing newline if necessary
    let std_print_string = if true {
        replace_std_escapes(input, arg_matches.is_present("newline"))
    } else {
        input
    };

    if let Some(hex) = arg_matches.value_of("truecolor") {
        println!("{}", is_valid_hex(&hex));
    }
}

// Replace escaped characters and conditionally add trailing newline
fn replace_std_escapes(input: String, mut no_trailing_newline: bool) -> String {
    let mut formatted: Vec<char> = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(pt) = it.next() {
        if pt == '\\' && it.peek() != None {
            let pt2 = it.next().unwrap();
            if pt2 == '0' || pt2 == 'x' || pt2 == 'u' || pt2 == 'U' {
                let radix = if pt2 == '0' { 8 } else { 16 };
                let numerals = String::new();
            } else {
                formatted.push(match pt2 {
                    'a' => '\x07',
                    'b' => '\x08',
                    'c' => {
                        no_trailing_newline = true;
                        break;
                    }
                    'e' => '\x1b',
                    'f' => '\x0c',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'v' => '\x0b',
                    '\\' => '\\',
                    other => other,
                });
            }
        } else {
            formatted.push(pt);
        }
    }
    if !no_trailing_newline {
        formatted.push('\n');
    }
    formatted.into_iter().collect()
}

fn is_valid_hex(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\da-f]{6}$").unwrap();
    }
    RE.is_match(s)
}
