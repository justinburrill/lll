use crate::format;
use crate::pathj::file::*;
use crate::pathj::path::*;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Directory {
    pub location: PathBuf,
    pub subdirs: Vec<Directory>,
    pub subfiles: Vec<File>,
}

impl ToString for Directory {
    fn to_string(&self) -> String {
        format!(
            "Directory {} with {} subdirs and {} subfiles",
            self.location.display(),
            self.subdirs.len(),
            self.subfiles.len()
        )
    }
}

impl HasPath for Directory {
    fn get_path(&self) -> PathBuf {
        self.location.clone()
    }
    fn get_item_name(&self) -> String {
        let str = self
            .location
            .to_str()
            .expect("Error converting a PathBuf to a str");
        let start_index: usize = *format::find_indices_of_char(str, '/').last().unwrap_or(&0);
        str[start_index..].to_owned()
    }
}

impl Directory {
    pub fn empty() -> Directory {
        Directory {
            location: PathBuf::new(),
            subdirs: vec![],
            subfiles: vec![],
        }
    }
    pub fn from_pathbuf(p: &PathBuf) -> Directory {
        let path = p.clone();
        Directory {
            location: path,
            subdirs: Directory::assemble_child_dirs(&p),
            subfiles: Directory::assemble_child_files(&p),
        }
    }
    pub fn print(d: Directory) {
        for subdir in d.subdirs {
            println!("subdir {:?}", subdir);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.subdirs.len() + self.subfiles.len() == 0
    }

    pub fn build_descendant_tree(&mut self) {
        self.subfiles = vec![File::new()];
    }

    pub fn get_descendant_count(&self) -> usize {
        let mut total_count: usize = 0;
        for subdir in &self.subdirs {
            total_count += 1;
            total_count += subdir.get_descendant_count();
        }
        total_count + self.subfiles.len()
    }

    /// Returns a [`usize`] representing the number of child files, NOT child directories
    pub fn get_child_file_count(&self) -> usize {
        self.subfiles.len()
    }

    fn assemble_child_files(p: &PathBuf) -> Vec<File> {
        let x = fs::read_dir(p).unwrap();
        todo!()
    }

    fn assemble_child_dirs(p: &PathBuf) -> Vec<Directory> {
        todo!()
    }
}
