use crate::counter::Counter;
use anyhow::{bail, Result};
use ignore::{DirEntry, Walk};
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
    files: HashMap<PathBuf, usize>,
}

fn parse(entry: &DirEntry) -> Result<syn::File> {
    let path = entry.path();
    let extension = path.extension();
    if entry.metadata()?.is_file() && extension.is_some_and(|e| e == "rs") {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        return Ok(syn::parse_file(&buf)?);
    }
    bail!("{} is not a rust file.", path.to_string_lossy());
}

impl Walker {
    fn merge(&mut self, path: PathBuf, counter: Counter) {
        let lines = counter.lines();
        self.lines += lines;
        self.files.insert(path.clone(), lines);
    }

    pub fn print(&self) {
        let len = self.lines.to_string().len();
        for (path, lines) in self.files.iter().sorted_by_key(|key| (key.1, key.0)) {
            println!("{1:<0$} {2}", len, lines, path.to_str().unwrap());
        }
        println!("{1:<0$} {2}", len, self.lines, "Total")
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) {
        let walker = Walk::new(path);
        for result in walker {
            if let Ok(entry) = result {
                if let Ok(ast) = parse(&entry) {
                    let mut counter = Counter::default();
                    counter.visit_file(&ast);
                    counter.remove_doc();
                    self.merge(entry.path().to_path_buf(), counter);
                }
            }
        }
    }
}
