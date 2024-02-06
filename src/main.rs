use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;
mod utils;

fn get_cwd() -> PathBuf {
    env::current_dir().unwrap()
}

fn print_vec(v: Vec<String>) {
    for x in v {
        println!("{}", x)
    }
}

fn get_cwd_str() -> String {
    // let path = env::current_dir()
    //     .unwrap()
    //     .into_os_string()
    //     .into_string()
    //     .unwrap();
    // path
    pathbuf_to_str(env::current_dir().unwrap())
}

fn enforce_trailing_slash(mut s: String) -> String {
    if s.as_bytes()[s.len() - 1] != b'/' {
        if s.contains('\\') {
            s.push('\\')
        } else {
            s.push('/')
        }
    }
    s
}

// fn pathbuf_vec_to_string_vec(paths: Vec<PathBuf>) -> Vec<String> {
//     paths.iter().map(pathbuf_ref_to_str).collect()
// }

fn pathbuf_to_str(p: PathBuf) -> String {
    p.into_os_string().into_string().unwrap()
}

fn pathbuf_ref_to_str(p: &PathBuf) -> String {
    p.clone().into_os_string().into_string().unwrap()
}

fn get_path_str_tail(p: String) -> String {
    // println!("{}", p);
    let parts: Vec<&str> = p.split("\\").collect();
    parts[parts.len() - 1].to_string()
}

fn get_path_tail(p: &PathBuf) -> String {
    get_path_str_tail(pathbuf_ref_to_str(p))
}

fn get_children(path: PathBuf) -> (Vec<PathBuf>, Vec<String>) {
    get_children_str(pathbuf_to_str(path))
}

fn get_children_str(path: String) -> (Vec<PathBuf>, Vec<String>) {
    let dir_iterator = fs::read_dir(path.clone());
    let dir_iterator = match dir_iterator {
        Ok(itr) => itr,
        Err(err) => panic!("could not read path {:?} - {:?}", &path, err),
    };
    let mut subdirs = Vec::new();
    let mut subfiles = Vec::new();
    for result in dir_iterator {
        let p = result.unwrap().path();
        if p.is_dir() {
            // println!("subdir {} found", pathbuf_ref_to_string(&p));
            subdirs.push(p)
        } else {
            subfiles.push(get_path_tail(&p));
        }
    }
    (subdirs, subfiles)
}

fn buffer_spaces_vec(strings: Vec<String>, level: usize, space_count: usize) -> Vec<String> {
    let mut strings2: Vec<String> = Vec::new();
    for str in strings {
        strings2.push(buffer_spaces(str, level, space_count));
        // println!("{} spaces added to {}", level * space_count, str.clone());
    }
    strings2
}

// sexy recursion
fn get_descendant_count(path: PathBuf, file_count_warning_cutoff: usize) -> usize {
    let mut total_child_count: usize = 0;
    let (subdirs, subfiles) = get_children(path);
    total_child_count += subdirs.len() + subfiles.len();
    for subdir in subdirs {
        total_child_count += get_descendant_count(subdir, file_count_warning_cutoff);
        if total_child_count > file_count_warning_cutoff {
            return total_child_count;
        }
    }
    total_child_count
}

fn buffer_spaces(str: String, level: usize, space_count: usize) -> String {
    " ".repeat(level * space_count) + &str
}

fn is_empty_dir(path: &PathBuf) -> bool {
    let (subdirs, subfiles) = get_children_str(pathbuf_ref_to_str(path));
    subdirs.len() + subfiles.len() == 0
}

fn print_dir(path: PathBuf, level: usize) {
    let space_count: usize = 4;
    let max_depth: usize = 5;

    let (subdirs, subfiles) = get_children(path.clone());
    for subdir in subdirs {
        // print name of dir in bold
        println!(
            "{}",
            buffer_spaces(get_path_tail(&subdir), level, space_count)
                .bold()
                .blue()
        );
        if is_empty_dir(&subdir) {
            println!(
                "{}",
                buffer_spaces(String::from("<Empty dir>"), level + 1, space_count).italic()
            );
        } else {
            if level < max_depth {
                print_dir(subdir, level + 1);
            } else {
                println!(
                    "{}",
                    buffer_spaces(String::from("<Max depth reached>"), level + 1, space_count)
                        .italic()
                );
            }
        }
    }
    // println!("{} subfiles:", pathbuf_ref_to_string(&path));
    print_vec(buffer_spaces_vec(subfiles, level, space_count));
}

fn str_to_pathbuf(s: String) -> PathBuf {
    let parts = s.split("/");
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path
}

fn main() {
    // let p = PathBuf::from(r"C:\src\cli_tools\super_search\one\one.md");
    // println!("{}", p.is_dir());

    let args: Vec<String> = env::args().collect();
    let mut current_working_directory: String = get_cwd_str();
    if args.len() > 1 {
        let mut path_ext = args[1].clone();
        path_ext = path_ext.replace("\\", "/");
        current_working_directory = enforce_trailing_slash(current_working_directory);
        current_working_directory.push_str(&path_ext);
    }
    // println!("working directory: {}", current_working_directory);
    // let file_count_warning_cutoff: usize = 10;
    // if get_descendant_count(get_cwd(), file_count_warning_cutoff) > file_count_warning_cutoff {
    //     let prompt = String::from("Warning: greater than ")
    //         + file_count_warning_cutoff.to_string().as_str()
    //         + " files - continue?";

    //     let default = false;
    //     if utils::bool_input(prompt, default) {
    //         // keep going :)
    //     } else {
    //         return;
    //     }
    // }
    print_dir(str_to_pathbuf(current_working_directory), 0);
}
