use clap::{ArgMatches, Values};
use colored::*;
use lazy_static::*;
use regex::Regex;
use std::char;

const COLORS: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "purple", "cyan", "white",
];

const STYLES: [&str; 8] = [
    "bold",
    "dimmed",
    "italic",
    "underline",
    "blink",
    "reversed",
    "hidden",
    "strikethrough",
];

// turn iterable of input into a string of space-separated words
pub fn args_to_input(values: Option<Values>) -> String {
    match values {
        None => String::from(""),
        Some(mut values) => {
            let mut input = String::from(values.next().unwrap());
            for value in values {
                input.push(' ');
                input.push_str(value);
            }
            input
        }
    }
}

pub fn special_chars_and_newlines(input: String, special_chars: bool, no_newline: bool) -> String {
    if special_chars {
        replace_std_escapes(input, no_newline)
    } else {
        let mut input = input;
        input.push('\n');
        input
    }
}

// Replace escaped characters and conditionally add trailing newline
fn replace_std_escapes(input: String, mut no_trailing_newline: bool) -> String {
    let mut formatted: Vec<char> = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(pt) = it.next() {
        // Handle escaped chars
        if pt == '\\' && it.peek() != None {
            let pt2 = it.next().unwrap();
            if pt2 == '0' || pt2 == 'x' || pt2 == 'u' || pt2 == 'U' {
                // Handle character code formats
                let radix = if pt2 == '0' { 8 } else { 16 };
                let (mini, maxi) = match pt2 {
                    '0' => (2, 3),
                    'x' => (1, 3),
                    'u' => (4, 4),
                    'U' => (8, 8),
                    _ => panic!(),
                };
                // Buffer in the next <=maxi digits
                let mut numerals: Vec<char> = Vec::new();
                while numerals.len() < maxi {
                    let c = it.peek();
                    if c == None {
                        // There is no next char
                        break;
                    } else {
                        let c = c.unwrap();
                        // Check c is a valid digit
                        let d = c.to_digit(radix);
                        if d == None {
                            // c isn't a valid digit
                            break;
                        } else {
                            numerals.push(*c);
                            it.next();
                        }
                    }
                }
                // Append character to formatted if the length is correct. Otherwise, append
                // numerals directly
                if numerals.len() >= mini {
                    let numerals: String = numerals.into_iter().collect();
                    let code_point = u32::from_str_radix(&numerals, radix).unwrap();
                    formatted.push(char::from_u32(code_point).unwrap());
                } else {
                    formatted.append(&mut numerals);
                }
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
        static ref RE: Regex = Regex::new(r"^[\da-fA-F]{6}$").unwrap();
    }
    RE.is_match(s)
}

pub fn hex_to_dectuple(hex: &str) -> Result<(u8, u8, u8), &str> {
    if is_valid_hex(&hex) {
        let first = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let second = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let third = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Ok((first, second, third))
    } else {
        Err("Not a valid hex")
    }
}

pub fn add_color(input: String, arg_matches: &ArgMatches, is_bg: bool) -> ColoredString {
    let suffix = if is_bg { "Bg" } else { "" };
    if let Some(hex) = arg_matches.value_of("truecolor".to_owned() + suffix) {
        if let Ok(truecolor_args) = hex_to_dectuple(&hex) {
            if is_bg {
                input.on_truecolor(truecolor_args.0, truecolor_args.1, truecolor_args.2)
            } else {
                input.truecolor(truecolor_args.0, truecolor_args.1, truecolor_args.2)
            }
        } else {
            input.normal()
        }
    } else {
        let prefix = if arg_matches.is_present("bright".to_owned() + suffix) {
            "bright "
        } else {
            ""
        };
        if let Some(color) = arg_matches.value_of("color".to_owned() + suffix) {
            if is_bg {
                input.on_color(prefix.to_owned() + color)
            } else {
                input.color(prefix.to_owned() + color)
            }
        } else {
            for color in &COLORS {
                if arg_matches.is_present(color.to_owned().to_owned() + suffix) {
                    if is_bg {
                        return input.on_color(prefix.to_owned() + color);
                    } else {
                        return input.color(prefix.to_owned() + color);
                    }
                }
            }
            input.normal()
        }
    }
}

pub fn add_style(input: ColoredString, arg_matches: &ArgMatches) -> ColoredString {
    if let Some(style) = arg_matches.value_of("style") {
        if style == "bold" {
            input.bold()
        } else if style == "dimmed" {
            input.dimmed()
        } else if style == "italic" {
            input.italic()
        } else if style == "underline" {
            input.underline()
        } else if style == "blink" {
            input.blink()
        } else if style == "reversed" {
            input.reversed()
        } else if style == "hidden" {
            input.hidden()
        } else if style == "strikethrough" {
            input.strikethrough()
        } else {
            input.normal()
        }
    } else {
        if arg_matches.is_present("bold") {
            input.bold()
        } else if arg_matches.is_present("dimmed") {
            input.dimmed()
        } else if arg_matches.is_present("italic") {
            input.italic()
        } else if arg_matches.is_present("underline") {
            input.underline()
        } else if arg_matches.is_present("blink") {
            input.blink()
        } else if arg_matches.is_present("reversed") {
            input.reversed()
        } else if arg_matches.is_present("hidden") {
            input.hidden()
        } else if arg_matches.is_present("strikethrough") {
            input.strikethrough()
        } else {
            input.normal()
        }
    }
}
