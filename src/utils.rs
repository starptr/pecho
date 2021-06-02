use clap::{ArgMatches, Values, App, Arg};
use colored::*;
use lazy_static::*;
use regex::Regex;
use std::char;

#[derive(Debug, PartialEq)]
enum PechoErr {
//    NoMatch,
    InvalidHex,
}

pub const COLORS: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "purple", "cyan", "white",
];

pub fn parse(arg_matches: ArgMatches) -> ColoredString {
    // Concatenate input into space-separated words
    let input = args_to_input(arg_matches.values_of("input"));

    // Replace escaped special characters and add trailing newline if necessary
    let std_print_string = special_chars_and_newlines(input, arg_matches.is_present("noEscapes"), arg_matches.is_present("noNewline"));

    let std_print_string = add_color_fg(std_print_string.normal(), &arg_matches);

    let std_print_string = add_color_bg(std_print_string, &arg_matches);

    add_style(std_print_string, &arg_matches)
}

// convert a color string to a Color enum
//fn parse_color(s: &str) -> Result<Color, PechoErr>{
//    match s {
//        "black" => Ok(Color::Black),
//        "red" => Ok(Color::Red),
//        "green" => Ok(Color::Green),
//        "yellow" => Ok(Color::Yellow),
//        "blue" => Ok(Color::Blue),
//        "purple" => Ok(Color::Magenta),
//        "cyan" => Ok(Color::Cyan),
//        "white" => Ok(Color::White),
//        _ => Err(PechoErr::NoMatch),
//    }
//}

// Get the cli App parser object
pub fn get_app() -> App<'static, 'static> {
    App::new("pecho")
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
                    .help("Specify color using an argument")
                    .short("c")
                    .long("color")
                    .takes_value(true)
                    .possible_values(&COLORS)
            )
        .arg(
                Arg::with_name("colorBg")
                    .help("Specify background color using an argument")
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
        //.group(ArgGroup::with_name("specific")
        //    .args(&["black", "red", "green", "yellow", "blue", "purple", "cyan", "white",
        //        "blackBg", "redBg", "greenBg", "yellowBg", "blueBg", "purpleBg", "cyanBg", "whiteBg",]))
        //.group(ArgGroup::with_name("generic")
        //    .args(&["color", "colorBg"]))
        //    //.conflicts_with("specific"))
        //.group(ArgGroup::with_name("trueGeneric")
        //    .args(&["truecolor", "truecolorBg"]))
        //    //.conflicts_with_all(&["specific", "generic"]))
}

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

fn hex_to_dectuple(hex: &str) -> Result<(u8, u8, u8), PechoErr> {
    if is_valid_hex(&hex) {
        let first = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let second = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let third = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Ok((first, second, third))
    } else {
        Err(PechoErr::InvalidHex)
    }
}

pub fn add_color_fg(input: ColoredString, arg_matches: &ArgMatches) -> ColoredString {
    add_color(input, &arg_matches, false)
}

pub fn add_color_bg(input: ColoredString, arg_matches: &ArgMatches) -> ColoredString {
    add_color(input, &arg_matches, true)
}

fn add_color(input: ColoredString, arg_matches: &ArgMatches, is_bg: bool) -> ColoredString {
    let suffix = if is_bg { "Bg" } else { "" };
    if let Some(hex) = arg_matches.value_of("truecolor".to_owned() + suffix) {
        if let Ok(truecolor_args) = hex_to_dectuple(&hex) {
            if is_bg {
                input.on_truecolor(truecolor_args.0, truecolor_args.1, truecolor_args.2)
            } else {
                input.truecolor(truecolor_args.0, truecolor_args.1, truecolor_args.2)
            }
        } else {
            input
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
            input
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
            input
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
            input
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let app = get_app();
        let out = parse(app.clone().get_matches_from(vec!["pecho", "echo"]));
        assert_eq!("echo\n", out.to_string());

        let out = parse(app.clone().get_matches_from(vec!["pecho", "yeah", "another"]));
        assert_eq!("yeah another\n", out.to_string());
    }

    #[test]
    fn test_replace_std_escapes() {
        let input = "\\\\abc\\077colon\\x3a\\x3Ajpo\\u304a\\u304A4byte\\U0001f602misc\\a\\b\\e\\f\\n\\r\\t\\v";
        let expected = "\\abc?colon::jpoおお4byte😂misc\x07\x08\x1b\x0c\n\r\t\x0b";
        assert_eq!(replace_std_escapes(input.to_string(), true), expected);
    }

    #[test]
    fn test_replace_std_escapes_no_newline_escape() {
        let input = "yeah yeah \\c no trailling newline nor text after";
        let expected = "yeah yeah ";
        assert_eq!(replace_std_escapes(input.to_string(), true), expected);
    }

    #[test]
    fn test_replace_std_escapes_newline_param() {
        let input = "please add newline";
        let expected = "please add newline\n";
        assert_eq!(replace_std_escapes(input.to_string(), false), expected);
    }

    #[test]
    fn test_special_chars_and_newlines() {
        let input = "basic \\n string";
        assert_eq!("basic \\n string\n", special_chars_and_newlines(input.to_string(), false, true));
        assert_eq!("basic \n string", special_chars_and_newlines(input.to_string(), true, true));
        assert_eq!("basic \\n string\n", special_chars_and_newlines(input.to_string(), false, false));
        assert_eq!("basic \n string\n", special_chars_and_newlines(input.to_string(), true, false))
    }

    #[test]
    fn test_hex_to_dectuple() {
        assert_eq!((0u8, 0u8, 0u8), hex_to_dectuple("000000").unwrap());
        assert_eq!((10u8, 10u8, 10u8), hex_to_dectuple("0a0a0a").unwrap());
        assert_eq!((255u8, 255u8, 255u8), hex_to_dectuple("ffffff").unwrap());
        assert_eq!(PechoErr::InvalidHex, hex_to_dectuple("00000g").unwrap_err());
    }

    #[test]
    fn test_add_color() {
        let app = get_app();
        assert_eq!("foo".red(), add_color("foo".normal(), &app.clone().get_matches_from(vec!["pecho", "-r"]), false));
        assert_eq!("foo".on_red(), add_color("foo".normal(), &app.clone().get_matches_from(vec!["pecho", "-R"]), true));
    }

    #[test]
    fn integration() {
        let app = get_app();
        assert_eq!("foo\n".red(), parse(app.clone().get_matches_from(vec!["pecho", "-r", "foo"])));
    }
}
