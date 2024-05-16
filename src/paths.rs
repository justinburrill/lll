use dirs_next;
use std::fs::DirEntry;
use std::fs::{self, ReadDir};
use std::io;
use std::path::Path;
use std::{env, path::PathBuf};

// Auto-implement clone for type FilePath
#[derive(Debug, Clone)]
pub struct FilePath {
    location: PathBuf,
}

pub struct Directory {
    pub path: FilePath,
    pub subdirs: Vec<Directory>,
    pub children: Vec<FilePath>,
}

impl FilePath {
    /// Create new [`FilePath`] with default [`PathBuf`] location
    pub fn new() -> FilePath {
        let location = PathBuf::new();
        FilePath { location }
    }

    /// Create new [`FilePath`] from provided [`PathBuf`] location
    pub fn from(location: PathBuf) -> FilePath {
        FilePath { location }
    }

    /// Convert a [`String`] to a [`FilePath`] object.
    pub fn from_string(s: String) -> FilePath {
        let mut parts: Vec<&str> = s.split("/").collect();
        let first_part = &enforce_leading_slash(parts[0].to_owned());
        parts[0] = first_part;

        let mut path = PathBuf::new();
        for part in parts {
            path.push(part);
        }
        FilePath::from(path)
    }

    /// Convert a [`str`] to a [`FilePath`] object.
    pub fn from_str(s: &str) -> FilePath {
        FilePath::from_string(s.to_owned())
    }

    /// Returns a [`PathBuf`] to the user's home directory
    pub fn get_home_path() -> FilePath {
        let home_path = dirs_next::home_dir();
        match home_path {
            Some(p) => FilePath::from(p),
            None => panic!(
            "Problem getting user's home directory. Try running without the use of '~' in the path"
        ),
        }
    }

    pub fn get_cwd_path() -> FilePath {
        match env::current_dir() {
            Ok(p) => FilePath::from(p),
            Err(e) => panic!("Error finding the current working path: {:?}", e),
        }
    }

    pub fn append(&self, other: FilePath) -> FilePath {
        // appends p2 to p1
        FilePath::from(self.location.join(other.location))
    }

    pub fn get_item_name(&self) -> String {
        let self_string = self.to_string();
        let x: Vec<&str> = self_string.split(get_slash_type(&self_string)).collect();
        let s = x.get(x.len() - 1).unwrap().to_string();
        s
    }

    pub fn is_file(&self) -> bool {
        self.location.is_file()
    }

    pub fn is_directory(&self) -> bool {
        self.location.is_dir()
    }

    pub fn is_empty_dir(&self) -> io::Result<bool> {
        Ok(self.is_directory() && self.get_child_count()? == 0)
    }

    pub fn is_absolute(&self) -> bool {
        self.location.is_absolute()
    }

    fn dir_entries_to_fp(iter: impl Iterator<Item = io::Result<DirEntry>>) -> Vec<FilePath> {
        let mut out: Vec<FilePath> = Vec::new();
        for item in iter {
            out.push(FilePath::from(
                item.expect("Error getting DirEntry from iter").path(),
            ));
        }
        out
    }

    pub fn get_children_iter(&self) -> io::Result<ReadDir> {
        match fs::read_dir(&self.location) {
            Ok(dir_iter) => Ok(dir_iter),
            Err(e) => Err(e),
        }
    }

    /// Returns the direct children of a specified [`FilePath`]
    pub fn get_children(&self) -> io::Result<Vec<FilePath>> {
        // Init empty Vec to hold children
        let mut children: Vec<FilePath> = Vec::new();
        for result in self.get_children_iter()? {
            let p = result.expect("Error getting PathBuf from DirEntry").path();
            children.push(FilePath::from(p));
        }
        Ok(children)
    }

    pub fn get_child_folders(&self) -> io::Result<Vec<FilePath>> {
        let iter = self.get_children_iter()?;
        let is_folder =
            |x: &io::Result<DirEntry>| x.as_ref().unwrap().file_type().unwrap().is_dir();
        Ok(FilePath::dir_entries_to_fp(iter.filter(is_folder)))
    }

    pub fn get_child_files(&self) -> io::Result<Vec<FilePath>> {
        Ok(FilePath::dir_entries_to_fp(
            self.get_children_iter()?
                .filter(|x: &io::Result<DirEntry>| {
                    x.as_ref().unwrap().file_type().unwrap().is_file()
                }),
        ))
    }

    pub fn get_child_count(&self) -> io::Result<usize> {
        // Ok(self.get_children()?.len())
        Ok(self.get_children_iter()?.count())
    }

    pub fn get_child_file_count(&self) -> usize {
        let x = self.get_child_files().unwrap().len();
        // println!("{} has {} children", self.to_string(), x);
        x
    }

    pub fn get_descendant_count(&self) -> usize {
        let mut total_desc_count: usize = 0;
        let child_count: usize = self.get_child_count().unwrap_or(0);
        total_desc_count += child_count;
        // default to 0 kids if there was an error getting the subfolders
        for subfolder in self.get_child_folders().unwrap_or(Vec::new()) {
            total_desc_count += subfolder.get_descendant_count();
        }
        return total_desc_count;
    }
}

impl ToString for FilePath {
    fn to_string(&self) -> String {
        self.location
            .to_str()
            .expect("Error converting FilePath to String")
            .to_owned()
    }
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            path: FilePath::new(),
            subdirs: vec![],
            children: vec![],
        }
    }
    pub fn from_fp(fp: FilePath) -> Directory {
        Directory {
            path: fp,
            subdirs: vec![],
            children: vec![],
        }
    }
}

pub fn handle_path(path_str: String) -> FilePath {
    let is_absolute = path_str.starts_with("/"); // || path_str.starts_with();
    if is_absolute {
        return FilePath::from_string(path_str);
    }

    // start with the current directory
    let mut fp = FilePath::get_cwd_path().location;
    let parts: Vec<&str> = path_str.split("/").collect();
    for part in parts {
        // println!("{}", part);

        match part {
            "~" => {
                fp = FilePath::get_home_path().location;
            }
            ".." => {
                fp.pop();
            }
            "." => {}
            _x => fp.push(_x), // wildcard
        }
    }
    FilePath::from(fp)
}

fn enforce_leading_slash(mut path_str: String) -> String {
    if path_str.len() == 0 {
        return "/".to_owned();
    }
    if path_str.as_bytes()[0] != b'/' {
        path_str.insert(0, get_slash_type(&path_str));
    }
    path_str
}

fn get_slash_type(s: &str) -> char {
    if s.contains('\\') {
        '\\'
    } else {
        '/'
    }
}
