use std::path::PathBuf;

pub fn str_to_pathbuf(s: String) -> PathBuf {
    let mut parts:Vec<&str> = s.split("/").collect();
    let first_part = &enforce_leading_slash(parts[0].to_owned());
    parts[0] = first_part;

    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path
}

pub fn handle_path(s: String) {
    // TODO: fix support usage of . and ~
    // using https://lib.rs/install/dirs-next

}

pub fn enforce_leading_slash(mut s: String) -> String {
    if s.len() == 0 {
        return "/".to_owned();
    }
    if s.as_bytes()[0] != b'/' {
        s.insert(0, find_slash_type(&s));
    }
    s
}


pub fn enforce_trailing_slash(mut s: String) -> String {
    if s.len() == 0 {
        return "/".to_owned();
    }
    // if the last char is a slash,
    if s.as_bytes()[s.len() - 1] != b'/' {
        // add slash character according to what was inputted
        s.push(find_slash_type(&s));
    }
    s
}

fn find_slash_type(s: &str) -> char {
    if s.contains('\\') {
        '\\'
    }
    else {
        '/'
    }
}

pub fn pathbuf_to_str(p: PathBuf) -> String {
    p.into_os_string().into_string().unwrap()
}

pub fn pathbuf_ref_to_str(p: &PathBuf) -> String {
    p.clone().into_os_string().into_string().unwrap()
}
