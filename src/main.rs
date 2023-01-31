use rsloc::walker::Walker;
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name
    let path = args.next().unwrap_or(".".to_string());
    let mut walker = Walker::default();
    walker.walk(path);
    walker.print();
}
