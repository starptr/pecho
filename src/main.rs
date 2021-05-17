mod utils;
use utils::*;

fn main() {
    print!("{}", parse(get_app().get_matches()));
}
