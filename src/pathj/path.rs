use std::collections::HashMap;
use std::iter::Chain;
use std::{ffi::OsStr, fs, path::PathBuf};

use lazy_static::lazy_static;
/// What to do upon finding any directory that we may want to handle differently,
/// e.g. not print recursively because there's too many subfiles.
pub enum SpecialDirAction {
    GiveChildCount,
    IgnoreEntirely,
}
extern crate lazy_static;

#[derive(PartialEq, Clone, Copy)]
pub enum PathType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct Path {
    location: PathBuf,
    path_type: PathType,
    // children: Option<Vec<Path>>,
    child_files: Option<Vec<Path>>,
    child_dirs: Option<Vec<Path>>,
    descendant_count_files: Option<usize>,
    descendant_count_dirs: Option<usize>,
}

lazy_static! {
    static ref SpecialDirs: HashMap<String, SpecialDirAction> = {
        let mut special_names: HashMap<String, SpecialDirAction> = HashMap::new();
        // special_names.insert(".git".to_string(), SpecialDirAction::GiveChildCount); // not needed because everything with a . is given this by default
        special_names.insert("node_modules".to_string(), SpecialDirAction::GiveChildCount);
        special_names.insert("incremental".to_string(), SpecialDirAction::GiveChildCount); // rust builds
        special_names
    };
}

impl Path {
    pub fn is_special_dir(&self) -> Option<&SpecialDirAction> {
        if self.path_type != PathType::Directory {
            return None;
        }

        let name = self.get_item_name();
        let mut result = SpecialDirs.get(name);
        result = match result {
            None if name.starts_with('.') => Some(&SpecialDirAction::GiveChildCount),
            _ => result,
        };
        return result;
    }

    pub fn from_osstr(p: &OsStr) -> Path {
        Path::from_pathbuf(&PathBuf::from(p))
    }

    pub fn from_str(p: &str) -> Path {
        Path::from_pathbuf(&PathBuf::from(p))
    }

    pub fn from_pathbuf(p: &PathBuf) -> Path {
        let path_type = if p.is_dir() {
            PathType::Directory
        } else {
            PathType::File
        };
        return Path {
            location: (*p).clone(),
            path_type,
            child_dirs: Option::None,
            child_files: Option::None,
            descendant_count_files: Option::None,
            descendant_count_dirs: Option::None,
        };
    }
    pub fn is_file(&self) -> bool {
        self.path_type == PathType::File
    }
    pub fn is_dir(&self) -> bool {
        self.path_type == PathType::Directory
    }
    pub fn to_str(&self) -> &str {
        self.location.to_str().unwrap()
    }
    pub fn get_item_name(&self) -> &str {
        self.location.file_name().unwrap().to_str().unwrap()
    }
    pub fn get_children_if_loaded(
        &self,
    ) -> Option<std::iter::Chain<std::slice::Iter<'_, Path>, std::vec::IntoIter<Path>>> {
        if self.child_dirs.is_none() || self.child_files.is_none() {
            None
        } else {
            let files = self.child_files.unwrap();
            Some(self.child_dirs.unwrap().iter().chain(files))
        }
    }

    pub fn get_direct_child_file_count(&mut self) -> usize {
        if !self.is_dir() {
            panic!("called get_child_file_count on a file not a dir!")
        }
        if self.get_children_if_loaded().is_none() {
            // load children into the vector if we haven't yet
            self.read_children();
        }
        self.children.as_ref().unwrap().len()
    }
    pub fn get_descendant_count(&mut self) -> (usize, usize) {
        if self.descendant_count_dirs.is_some() && self.descendant_count_files.is_some() {
            return (
                self.descendant_count_dirs.unwrap(),
                self.descendant_count_files.unwrap(),
            );
        } else {
            let mut total: usize = self.get_direct_child_file_count();
            for mut dir in self.clone_child_dirs() {
                total += dir.get_descendant_count();
            }
            return total;
        }
    }
    pub fn is_empty_dir(&mut self) -> bool {
        self.is_dir() && self.get_direct_child_file_count() == 0
    }

    // fn get_path_type(&mut self) -> PathType {
    //     let p = &self.location;
    //     if p.is_dir() {
    //         return Option::Some(PathType::Directory);
    //     } else if p.is_file() {
    //         return Option::Some(PathType::File);
    //     } else {
    //         panic!("path {} is not a file or a directory", p.to_string_lossy())
    //     }
    // }

    // pub fn clone_path_type(&mut self) -> PathType {
    //     match &self.path_type {
    //         Some(x) => x.clone(),
    //         None => {
    //             self.get_path_type();
    //             self.clone_path_type()
    //         }
    //     }
    // }

    pub fn clone_children(&mut self) -> Vec<Path> {
        match &self.children {
            Some(x) => x.to_vec(),
            None => {
                self.read_children();
                self.clone_children()
            }
        }
    }

    pub fn clone_child_files(&mut self) -> Vec<Path> {
        self.clone_children()
            .into_iter()
            .filter(|x| x.is_file())
            .collect()
    }

    pub fn clone_child_dirs(&mut self) -> Vec<Path> {
        self.clone_children()
            .into_iter()
            .filter(|x| x.is_dir())
            .collect()
    }

    fn read_children(&mut self) {
        self.children = Some(vec![]);
        // set self.children to a list of paths
        let files = match fs::read_dir(&self.location) {
            Ok(x) => x,
            Err(e) => panic!(
                "Can't read a file in {:?} because of {:?}",
                self.location, e
            ),
        };
        for dir_entry in files {
            self.children
                .as_mut()
                .unwrap()
                .push(Path::from_pathbuf(&dir_entry.unwrap().path()));
        }
    }
}
