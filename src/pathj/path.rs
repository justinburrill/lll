use std::collections::HashMap;

use std::{fs, path::PathBuf};

use lazy_static::lazy_static;
extern crate lazy_static;

/// What to do upon finding any directory that we may want to handle differently,
/// e.g. not print recursively because there's too many subfiles.
pub enum SpecialDirAction {
    GiveChildCount,
    IgnoreEntirely,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PathType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Path {
    location: PathBuf,
    path_type: PathType,
    // children: Option<Vec<Path>>,
    pub child_files: Option<Vec<Path>>,
    pub child_dirs: Option<Vec<Path>>,
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

    // pub fn is_file(&self) -> bool {
    //     self.path_type == PathType::File
    // }

    pub fn is_dir(&self) -> bool {
        self.path_type == PathType::Directory
    }

    pub fn to_str(&self) -> &str {
        self.location.to_str().unwrap()
    }

    pub fn get_item_name(&self) -> &str {
        self.location.file_name().unwrap().to_str().unwrap()
    }

    pub fn is_loaded(&self) -> bool {
        self.child_dirs.is_some() && self.child_files.is_some()
    }

    pub fn get_direct_child_dir_count(&mut self) -> usize {
        if !self.is_dir() {
            panic!("called get_child_dir_count on a file not a dir!")
        }
        if !self.is_loaded() {
            // load children into the vector if we haven't yet
            self.read_children();
        }
        self.child_dirs.as_ref().unwrap().len()
    }

    pub fn get_direct_child_file_count(&mut self) -> usize {
        if !self.is_dir() {
            panic!("called get_child_file_count on a file not a dir!")
        }
        if !self.is_loaded() {
            // load children into the vector if we haven't yet
            self.read_children();
        }
        self.child_files.as_ref().unwrap().len()
    }

    pub fn get_descendant_counts(&mut self) -> (usize, usize) {
        if self.descendant_count_dirs.is_some() && self.descendant_count_files.is_some() {
            return (
                self.descendant_count_dirs.unwrap(),
                self.descendant_count_files.unwrap(),
            );
        } else {
            let mut total_dirs: usize = self.get_direct_child_dir_count();
            let mut total_files: usize = self.get_direct_child_file_count();
            for dir in self.child_dirs.as_mut().unwrap() {
                let totals = dir.get_descendant_counts();
                total_dirs += totals.0;
                total_files += totals.1;
            }
            return (total_dirs, total_files);
        }
    }

    pub fn get_descendant_count(&mut self) -> usize {
        let (file_count, dir_count) = self.get_descendant_counts();
        file_count + dir_count
    }

    pub fn is_empty_dir(&mut self) -> bool {
        self.is_dir() && self.get_descendant_count() == 0
    }

    pub fn get_children(&mut self) -> (&mut Vec<Path>, &mut Vec<Path>) {
        if !self.is_loaded() {
            self.read_children();
        }
        return (
            self.child_dirs.as_mut().unwrap(),
            self.child_files.as_mut().unwrap(),
        );
    }

    fn read_children(&mut self) {
        let children = match fs::read_dir(&self.location) {
            Ok(x) => x,
            Err(e) => panic!(
                "Can't read a file in {:?} because of {:?}",
                self.location, e
            ),
        };
        let (dirs, files): (Vec<_>, Vec<_>) = children
            .into_iter()
            .map(|f| f.unwrap())
            .partition(|f| f.file_type().unwrap().is_dir());
        self.child_dirs = Some(dirs.iter().map(|d| Path::from_pathbuf(&d.path())).collect());
        self.child_files = Some(
            files
                .iter()
                .map(|f| Path::from_pathbuf(&f.path()))
                .collect(),
        );
    }
}
