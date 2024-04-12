use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;
mod input;
mod paths;
use crate::paths::*;

fn print_vec(v: Vec<String>) {
    for x in v {
        println!("{}", x)
    }
}

// fn pathbuf_vec_to_string_vec(paths: Vec<PathBuf>) -> Vec<String> {
//     paths.iter().map(pathbuf_ref_to_str).collect()
// }

fn get_string_path_tail(p: String) -> String {
    // println!("{}", p);
    let parts: Vec<&str> = p.split("\\").collect();
    parts[parts.len() - 1].to_string()
}

fn get_path_tail_string(p: &PathBuf) -> String {
    get_string_path_tail(pathbuf_ref_to_string(p))
}

fn get_children(path: PathBuf) -> (Vec<PathBuf>, Vec<String>) {
    get_children_str(pathbuf_to_string(path))
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
            subfiles.push(get_path_tail_string(&p));
        }
    }
    (subdirs, subfiles)
}

fn buffer_spaces_vec(strings: Vec<String>, level: usize, space_count: usize) -> Vec<String> {
    let mut strings2: Vec<String> = Vec::new();
    for str in strings {
        strings2.push(buffer_spaces_string(str, level, space_count));
        // println!("{} spaces added to {}", level * space_count, str.clone());
    }
    strings2
}

// sexy recursion TODO: unused
// fn get_descendant_count(path: PathBuf, file_count_warning_cutoff: usize) -> usize {
//     let mut total_child_count: usize = 0;
//     let (subdirs, subfiles) = get_children(path);
//     total_child_count += subdirs.len() + subfiles.len();
//     for subdir in subdirs {
//         total_child_count += get_descendant_count(subdir, file_count_warning_cutoff);
//         if total_child_count > file_count_warning_cutoff {
//             return total_child_count;
//         }
//     }
//     total_child_count
// }

fn buffer_spaces_string(str: String, level: usize, space_count: usize) -> String {
    " ".repeat(level * space_count) + &str
}

fn is_empty_dir(path: &PathBuf) -> bool {
    let (subdirs, subfiles) = get_children_str(pathbuf_ref_to_string(path));
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
            buffer_spaces_string(get_path_tail_string(&subdir), level, space_count)
                .bold()
                .blue()
        );
        if is_empty_dir(&subdir) {
            println!(
                "{}",
                buffer_spaces_string(String::from("<Empty dir>"), level + 1, space_count).italic()
            );
        } else {
            if level < max_depth {
                print_dir(subdir, level + 1);
            } else {
                println!(
                    "{}",
                    buffer_spaces_string(
                        String::from("<Max depth reached>"),
                        level + 1,
                        space_count
                    )
                    .italic()
                );
            }
        }
    }
    // println!("{} subfiles:", pathbuf_ref_to_string(&path));
    print_vec(buffer_spaces_vec(subfiles, level, space_count));
}

fn main() {
    // collect cmd line args
    let args: Vec<String> = env::args().collect();
    // ensure that the cwd has the slash at the end
    let current_working_directory: String =
        enforce_trailing_slash(pathbuf_to_string(get_cwd_path()));
    // vector to hold the paths to be searched
    let mut paths_to_search: Vec<String> = Vec::new();
    // if there are args given by the user,
    if args.len() > 1 {
        for arg in args {
            // copy path from the cmd line arguments
            let path_ext = &arg;

            // replace back slashes from user inputwith forward slashes
            //path_ext = path_ext.replace("\\", "/");
            // push the modified path ending to the cwd
            paths_to_search.push(current_working_directory.clone() + path_ext);
        }
    } else {
        // if there is no path given in the cmd arguments,
        // then add the cwd to the paths to be scanned
    }

    // unused file count warning
    // TODO: fix this
    // println!("working directory: {}", current_working_directory);
    // let file_count_warning_cutoff: usize = 10;
    // if get_descendant_count(get_cwd(), file_count_warning_cutoff) > file_count_warning_cutoff {
    //     let prompt = String::from("Warning: greater than ")
    //         + file_count_warning_cutoff.to_string().as_str()
    //         + " files - continue?";
    //     let default = false;
    //     if input::bool_input(prompt, default) {
    //         // keep going :)
    //     } else {
    //         return;
    //     }
    // }

    // search each path
    for path in paths_to_search {
        println!("{}", path);
        print_dir(string_to_pathbuf(path), 0);
    }
}
