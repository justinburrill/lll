use dirs_next;
use std::{env, path::PathBuf};

pub fn string_to_pathbuf(s: String) -> PathBuf {
    let mut parts: Vec<&str> = s.split("/").collect();
    let first_part = &enforce_leading_slash(parts[0].to_owned());
    parts[0] = first_part;

    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path
}

/// Returns a [`PathBuf`] to the user's home directory
fn home_path() -> PathBuf {
    let home_path = dirs_next::home_dir();
    // let home_path: PathBuf = match home_path {
    //     Some(p) => p,
    //     None => panic!(
    //         "Problem getting user's home directory. Try running without the use of '~' in the path"
    //     ),
    // };
    match home_path {
        Some(p) => p,
        None => panic!(
            "Problem getting user's home directory. Try running without the use of '~' in the path"
        ),
    }
}

pub fn add_paths(p1: PathBuf, p2: PathBuf) -> PathBuf {
    // appends p2 to p1
    p1.join(p2)
}

pub fn get_cwd_path() -> PathBuf {
    match env::current_dir() {
        Ok(p) => p,
        Err(e) => panic!("Error finding the current working path: {:?}", e),
    }
}

pub fn handle_path(path_str: String) -> PathBuf {
    // TODO: fix support usage of . and ~
    // using https://lib.rs/install/dirs-next

    // This should never happen
    if path_str.len() == 0 {
        return PathBuf::new();
    }

    // if the first char is a '~',
    if path_str.chars().nth(0).unwrap() == '~' {
        // println!("replacing ~ with home path");
        // then replace '~' with home dir
        let path_ext_str = &path_str[1..];
        let path: PathBuf = string_to_pathbuf(path_ext_str.to_owned());
        return add_paths(home_path(), path);
    }

    return PathBuf::new();
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

pub fn enforce_trailing_slash(mut path_str: String) -> String {
    if path_str.len() == 0 {
        return "/".to_owned();
    }
    // if the last char is a slash,
    if path_str.as_bytes()[path_str.len() - 1] != b'/' {
        // add slash character according to what was inputted
        path_str.push(find_slash_type(&path_str));
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
