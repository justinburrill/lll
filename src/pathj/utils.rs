use std::{env, path::PathBuf};

/// Returns a [`PathBuf`] to the user's home directory
pub fn get_home_path() -> PathBuf {
    let home_path = dirs_next::home_dir();
    match home_path {
        Some(p) => p,
        None => panic!(
            "Problem getting user's home directory. Try running without the use of '~' in the path"
        ),
    }
}

pub fn get_cwd_path() -> PathBuf {
    match env::current_dir() {
        Ok(p) => p,
        Err(e) => panic!("Error finding the current working path: {:?}", e),
    }
}
