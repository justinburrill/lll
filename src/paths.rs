use dirs_next;
use std::{env, path::PathBuf};

use crate::get_children;

pub struct FilePath {
    location: PathBuf,
}

impl FilePath {
    pub fn new(location: PathBuf) -> FilePath {
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
        FilePath::new(path)
    }

    /// Returns a [`PathBuf`] to the user's home directory
    pub fn get_home_path() -> FilePath {
        let home_path = dirs_next::home_dir();
        match home_path {
            Some(p) => FilePath::new(p),
            None => panic!(
            "Problem getting user's home directory. Try running without the use of '~' in the path"
        ),
        }
    }

    pub fn get_cwd_path() -> FilePath {
        match env::current_dir() {
            Ok(p) => FilePath::new(p),
            Err(e) => panic!("Error finding the current working path: {:?}", e),
        }
    }

    pub fn append(&self, other: FilePath) -> FilePath {
        // appends p2 to p1
        self.location.join(other.location);
        *self.to_owned()
    }

    pub fn get_item_name(&self) -> String {
        let self_string = self.to_string();
        let x: Vec<&str> = self_string.split("/").collect();
        x.get(x.len()).unwrap().to_string()
    }

    pub fn is_file(&self) -> bool {
        self.location.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.location.is_dir()
    }

    pub fn is_empty(&self) -> bool {
        self.is_dir() && get_children(*self).len().eq(&0)
    }
}

impl ToString for FilePath {
    fn to_string(&self) -> String {
        self.location.as_mut_os_string().into_string().unwrap()
    }
}

pub fn handle_path(path_str: String) -> FilePath {
    // TODO: fix support usage of . and ~
    // using https://lib.rs/install/dirs-next

    let chars = path_str.chars();
    // if the first char is a '~',
    if chars.clone().nth(0).unwrap() == '~' {
        // println!("replacing ~ with home path");
        // then replace '~' with home dir
        let path_ext_str = &path_str[1..];
        let path: FilePath = FilePath::from_string(path_ext_str.to_owned());
        return FilePath::get_home_path().append(path);
    }

    return FilePath::from_string(path_str);
}

pub fn enforce_leading_slash(mut path_str: String) -> String {
    if path_str.len() == 0 {
        return "/".to_owned();
    }
    if path_str.as_bytes()[0] != b'/' {
        path_str.insert(0, find_slash_type(&path_str));
    }
    path_str
}

fn find_slash_type(s: &str) -> char {
    if s.contains('\\') {
        '\\'
    } else {
        '/'
    }
}

pub fn pathbuf_to_string(p: PathBuf) -> String {
    p.into_os_string().into_string().unwrap()
}

pub fn pathbuf_ref_to_string(p: &PathBuf) -> String {
    p.clone().into_os_string().into_string().unwrap()
}
