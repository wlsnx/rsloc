use rsloc::print_count;
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name
    let path = args.next().unwrap_or(".".to_string());
    print_count(path);
}
