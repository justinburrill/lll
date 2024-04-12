use colored::ColoredString;
use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;
mod input;
mod paths;
use crate::paths::*;

fn get_string_path_tail(p: String) -> String {
    // println!("{}", p);
    let parts: Vec<&str> = p.split("\\").collect();
    parts[parts.len() - 1].to_string()
}

fn get_path_tail_string(p: &PathBuf) -> String {
    get_string_path_tail(pathbuf_ref_to_string(p))
}

// fn get_children(path: PathBuf) -> (Vec<PathBuf>, Vec<String>) {
//     let dir_iterator = fs::read_dir(path.clone());
//     let dir_iterator = match dir_iterator {
//         Ok(itr) => itr,
//         Err(err) => panic!("could not read path {:?} - {:?}", &path, err),
//     };
//     let mut subdirs = Vec::new();
//     let mut subfiles = Vec::new();
//     for result in dir_iterator {
//         let p = result.unwrap().path();
//         if p.is_dir() {
//             // println!("subdir {} found", pathbuf_ref_to_string(&p));
//             subdirs.push(p)
//         } else {
//             subfiles.push(get_path_tail_string(&p));
//         }
//     }
//     (subdirs, subfiles)
// }

fn get_children(path: FilePath) -> Vec<FilePath> {
    let dir_iterator = fs::read_dir(path);
    let dir_iterator = match dir_iterator {
        Ok(itr) => itr,
        Err(err) => panic!("could not read path {:?} - {:?}", &path.to_string(), err),
    };
    // Init two empty Vecs to hold children
    let mut children: Vec<FilePath> = Vec::new();

    for result in dir_iterator {
        let p = result.unwrap().path();
        children.push(FilePath::new(p));
    }
    children
}

/// Adds spaces to a vector of [`String`] objects.
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

/// Adds a number of spaces to the start of a [`String`]
fn buffer_spaces_string(str: String, level: usize, space_count: usize) -> String {
    " ".repeat(level * space_count) + &str
}

fn is_empty_dir(path: FilePath) -> bool {
    let (subdirs, subfiles) = get_children(path);
    subdirs.len() + subfiles.len() == 0
}

fn print_dir(path: FilePath, level: usize) {
    let space_count: usize = 4;
    let max_depth: usize = 5;
    let max_subfiles_to_print: usize = 3;

    // let (subdirs, subfiles) = get_children(path);
    let children = get_children(path);
    for child in children {
        // print name of dir in bold
        println!(
            "{}",
            buffer_spaces_string(child.get_item_name(), level, space_count)
                .bold()
                .blue()
        );
        if is_empty_dir(&subdir) {
            println!(
                "{}",
                format_notify_string(buffer_spaces_string(
                    String::from("<Empty dir>"),
                    level + 1,
                    space_count
                ))
            );
        } else {
            if level < max_depth {
                print_dir(subdir, level + 1);
            } else {
                println!(
                    "{}",
                    format_notify_string(buffer_spaces_string(
                        String::from("<Max depth reached>"),
                        level + 1,
                        space_count
                    ))
                );
            }
        }
    }
    // println!("{} subfiles:", pathbuf_ref_to_string(&path));
    // print_vec(
    // buffer_spaces_vec(subfiles, level, space_count),
    // max_subfiles_to_print,
    // );

    let mut printed_file_count: usize = 0;
    let strings_to_be_printed = buffer_spaces_vec(subfiles, level, space_count);
    let string_count = strings_to_be_printed.len();
    for line in strings_to_be_printed {
        println!("{}", line);
        printed_file_count += 1;
        if printed_file_count > max_subfiles_to_print {
            let remaining_files = string_count - printed_file_count;
            let s: String = buffer_spaces_string(
                format!("<{} more files>", remaining_files),
                level,
                space_count,
            );
            println!("{}", format_notify_string(s));
            break;
        }
    }
}

fn format_notify_string(s: String) -> ColoredString {
    s.italic().bold().dimmed()
}

fn handle_args(args: Vec<String>) -> Vec<FilePath> {
    let current_working_directory: FilePath = get_cwd_path();

    // vector to hold the paths to be searched
    let mut paths_to_search: Vec<FilePath> = Vec::new();

    // if there are args given by the user,
    if args.len() > 1 {
        for arg in args {
            // copy path from the cmd line arguments
            let path_ext = handle_path(arg);
            // replace back slashes from user inputwith forward slashes
            //path_ext = path_ext.replace("\\", "/");
            // push the modified path ending to the cwd
            paths_to_search.push(current_working_directory.append(path_ext))
        }
    } else {
        // TODO:
        // if there is no path given in the cmd arguments,
        // then add the cwd to the paths to be scanned
        paths_to_search.push(current_working_directory);
    }

    paths_to_search
}

fn main() {
    // println!("{}", home_dir().unwrap().to_str().unwrap());

    // collect cmd line args
    let args: Vec<String> = env::args().collect();

    let paths_to_search = handle_args(args);

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
        let message: ColoredString = format!("Searching 👉👉 {}", path.to_string()).bold();
        println!("{}", message);
        // print_dir(path, 0);
    }
}
