use rsloc::Walker;
use std::env;

fn main() {
    let mut args = env::args();
    let mut walker = Walker::default();
    if args.len() == 1 {
        walker.walk(".");
    } else {
        let _ = args.next(); // executable name
        while let Some(path) = args.next() {
            walker.walk(path);
        }
    }
    walker.print();
}
