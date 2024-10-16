use std::path::PathBuf;

pub struct File;
pub struct Directory;

pub struct Path<T> {
    location: PathBuf,
    path_type: std::marker::PhantomData<T>,
}

impl<T> Path<T> {
    // fn new<R>() -> Path<R> {
    //     Path {
    //         location: PathBuf::new(),
    //         path_type: R,
    //     }
    // }
    fn is_file(&self) -> bool {
        self.path_type == File
    }
    fn is_dir(&self) -> bool {
        self.path_type == Directory
    }
}

impl Path<File> {}

impl Path<Directory> {}
