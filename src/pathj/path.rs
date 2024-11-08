use std::{
    ffi::OsStr,
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

#[derive(PartialEq, Clone, Copy)]
pub enum PathType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct Path {
    location: PathBuf,
    path_type: PathType,
    children: Option<Vec<Path>>,
}

impl Path {
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
            path_type: path_type,
            children: Option::None,
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
    pub fn get_direct_child_file_count(&mut self) -> usize {
        if !self.is_dir() {
            panic!("called get_child_file_count on a file not a dir!")
        }
        if self.children.is_none() {
            // load children into the vector if we haven't yet
            self.read_children();
        }
        self.children.as_ref().unwrap().len()
    }
    pub fn get_child_file_count_recursive(&mut self) -> usize {
        let mut total: usize = self.get_direct_child_file_count();
        for mut dir in self.clone_child_dirs() {
            total += dir.get_child_file_count_recursive();
        }
        return total;
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
