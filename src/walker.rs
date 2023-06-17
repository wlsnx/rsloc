use crate::counter::Counter;
use anyhow::Result;
use ignore::WalkBuilder;
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

fn parse(path: &Path) -> Result<syn::File> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(syn::parse_file(&buf)?)
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
        let walker = WalkBuilder::new(path)
            .filter_entry(|entry| {
                entry.path().extension().is_some_and(|ext| ext == "rs")
                    || (entry.file_type().is_some_and(|ty| ty.is_dir())
                        && !entry.path().to_string_lossy().ends_with("tests"))
            })
            .build();
        for result in walker {
            if let Ok(entry) = result {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(ast) = parse(&path) {
                        let mut counter = Counter::default();
                        counter.visit_file(&ast);
                        counter.remove_doc();
                        self.merge(entry.path().to_path_buf(), counter);
                    }
                }
            }
        }
    }
}
