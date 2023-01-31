use crate::counter::Counter;
use ignore::Walk;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use syn::visit::Visit;

#[derive(Debug, Default)]
pub struct Walker {
    lines: usize,
    files: HashMap<PathBuf, Counter>,
}

impl Walker {
    fn merge(&mut self, path: PathBuf, counter: Counter) {
        self.lines += counter.lines();
        self.files.insert(path.clone(), counter);
    }

    pub fn print(&self) {
        for (path, counter) in self.files.iter().sorted_by_key(|key| key.0) {
            println!("{:<7} {}", counter.lines(), path.to_str().unwrap());
        }
        println!("{:<7} {}", self.lines, "Total")
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) {
        let walker = Walk::new(path);
        for result in walker {
            if let Ok(entry) = result {
                let path = entry.path().to_str().unwrap();
                if entry.metadata().unwrap().is_file() && path.ends_with(".rs") {
                    let mut counter = Counter::default();
                    let mut file = File::open(path).unwrap();
                    let mut buf = String::new();
                    file.read_to_string(&mut buf).unwrap();
                    let ast = syn::parse_file(&buf).unwrap();
                    counter.visit_file(&ast);
                    counter.remove_doc();
                    self.merge(entry.path().to_path_buf(), counter);
                }
            }
        }
    }
}
