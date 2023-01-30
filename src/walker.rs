use crate::counter::Counter;
use ignore::Walk;
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use syn::visit::Visit;

#[derive(Debug, Default)]
pub struct Walker {
    pub lines: usize,
    pub doc_lines: usize,
    pub test_lines: usize,
    pub files: HashMap<PathBuf, Counter>,
    pub dirs: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Walker {
    fn merge(&mut self, path: PathBuf, counter: Counter) {
        self.lines += counter.lines.len();
        self.doc_lines += counter.doc_lines;
        let parent = path.parent().unwrap().to_path_buf();
        self.files.insert(path.clone(), counter);
        let dir = self.dirs.entry(parent).or_default();
        dir.push(path);
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
                    self.merge(entry.path().to_path_buf(), counter);
                }
            }
        }
    }
}
