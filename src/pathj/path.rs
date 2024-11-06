use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

#[derive(PartialEq)]
pub enum PathType {
    File,
    Directory,
}

pub struct Path {
    location: PathBuf,
    path_type: Option<PathType>,
}

impl Path {
    pub fn from_str(p: &str) -> Path {
        Path {
            location: PathBuf::from(p),
            path_type: Option::None,
        }
    }
    pub fn from_pathbuf(p: &PathBuf) -> Result<Path, std::io::Error> {
        let mut path_t = Option::None;
        if p.is_dir() {
            path_t = Option::Some(PathType::Directory);
        } else if p.is_file() {
            path_t = Option::Some(PathType::File);
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                format!("path {} is not a file or a directory", p.to_string_lossy()),
            ));
        }
        Ok(Path {
            location: *p,
            path_type: path_t,
        })
    }
    pub fn is_file(&self) -> bool {
        self.path_type == Option::Some(PathType::File)
    }
    pub fn is_dir(&self) -> bool {
        self.path_type == Option::Some(PathType::Directory)
    }
    pub fn to_str(&self) -> &str {
        self.location.to_str().unwrap()
    }
}
