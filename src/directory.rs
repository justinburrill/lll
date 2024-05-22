use crate::filepath::*;
use crate::pathj::*;
use std::{env, path::PathBuf};



#[derive(Debug)]
pub struct Directory {
    pub path: FilePath,
    pub subdirs: Vec<Directory>,
    pub subfiles: Vec<FilePath>,
}

impl ToString for Directory {
    fn to_string(&self) -> String {
        format!("Directory {} with {} subdirs and {} subfiles", self.path.to_string(), self.subdirs.len(), self.subfiles.len())
    }
}

impl Pathj for Directory {
    fn get_path(&self) -> FilePath {
        self.path
    }
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            path: FilePath::new(),
            subdirs: vec![],
            subfiles: vec![],
        }
    }
    pub fn from_fp(fp: FilePath) -> Directory {
        
        Directory {
            path: fp,
            subdirs: vec![],
            subfiles: vec![],
        }
    }
    pub fn print(d: Directory) {
        for subdir in d.subdirs {
            println!("subdir {:?}", subdir);
        }
    }
}
