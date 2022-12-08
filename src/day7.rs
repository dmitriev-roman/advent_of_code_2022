use std::{fs::File, io::{BufReader, BufRead}, rc::Rc, collections::HashMap, vec};

#[derive(Debug)]
struct Directory {
    children: HashMap<String, Self>,
    files: Vec<(String, usize)>
}

impl Directory {
    fn new() -> Self {
        Self { children: HashMap::new(), files: vec![] }
    }

    fn mkdir(&mut self, name: impl Into<String>) {
        &self.children.insert(name.into(), Self::new());
    }

    fn add_file(&mut self, name: impl Into<String>, size: usize) {
        self.files.push((name.into(), size));
    }
}


fn main() {
    let data = BufReader::new(File::open("task_data/7.txt").unwrap())
        .lines()
        .flatten()
        .collect::<Vec<_>>();
    let mut root_dir = Directory::new();
    root_dir.mkdir("sub_dir_1");
    root_dir.add_file("some_file", 1024);
    println!("{root_dir:#?}");
}