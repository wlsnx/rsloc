#![feature(is_some_and)]

use std::path::Path;
mod counter;
mod walker;
use walker::Walker;

pub fn print_count<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut walker = Walker::default();
    walker.walk(path);
    walker.print();
}
