use colored::ColoredString;
mod input;
use input::*;
mod format;
use format::*;
use std::env;
use std::fs;
use std::path::PathBuf;
mod paths;
use crate::paths::*;

/// Returns the direct children of a specified [`FilePath`]
fn get_children(path: &FilePath) -> Vec<FilePath> {
    let dir_iterator = match fs::read_dir(path.to_string()) {
        Ok(itr) => itr,
        Err(e) => panic!(
            "Could not read path: {:?} due to error: {:?}",
            &path.to_string(),
            e
        ),
    };
    // Init empty Vec to hold children
    let mut children: Vec<FilePath> = Vec::new();

    for result in dir_iterator {
        let p = result.expect("DirEntry error").path();
        children.push(FilePath::new(p));
    }
    children
}

// sexy recursion TODO: unused
fn get_descendant_count(path: FilePath, file_count_warning_cutoff: usize) -> usize {
    let mut total_child_count: usize = 0;
    let children = get_children(&path);
    total_child_count += children.len();
    for child in children {
        total_child_count += get_descendant_count(child, file_count_warning_cutoff);
        if total_child_count > file_count_warning_cutoff {
            return total_child_count;
        }
    }
    total_child_count
}

fn print_children(path: FilePath, depth: usize) {
    let space_count: usize = 4;
    let max_depth: usize = 5;
    let max_subfiles_to_print: usize = 3;
    let children_itr = get_children(&path).into_iter();
    let files: Vec<FilePath> = children_itr.clone().filter(|x| x.is_file()).collect();
    let folders: Vec<FilePath> = children_itr.filter(|x| x.is_directory()).collect();
    // needs to handle 3 cases:
    // - is a dir with children
    // - is a dir without children
    // - is a file

    // print subfolders first
    for subfolder in folders {
        // Print name of the folder
        println!(
            "{:?}",
            format_dir(buffer_spaces_str(
                subfolder.get_item_name().as_str(),
                depth,
                space_count
            ))
        );
        // Followed by it's children
        print_children(subfolder, depth + 1);
    }
    // followed by subfiles
    let mut printed_file_count: usize = 0;
    for subfile in files {
        println!("{:?}", subfile.get_item_name());
        printed_file_count += 1;
        if printed_file_count > max_subfiles_to_print {
            println!(
                "{}",
                format_notify(buffer_spaces_str(
                    "<Max depth reached>",
                    depth + 1,
                    space_count
                ))
            );
            break;
        }
    }

    // for child in children_itr.clone() {
    // println!(
    //     "{}",
    //     buffer_spaces_string(child.get_item_name(), level, space_count)
    //         .bold()
    //         .blue()
    // );
    // if child.is_empty_dir() {
    //     println!(
    //         "{}",
    //         format_notify(buffer_spaces_str("<Empty dir>", level + 1, space_count))
    //     );
    // } else {
    //     if level < max_depth {
    //         print_children(child, level + 1);
    //     } else {
    //         println!(
    //             "{}",
    //             format_notify(buffer_spaces_str(
    //                 "<Max depth reached>",
    //                 level + 1,
    //                 space_count
    //             ))
    //         );
    //     }
    // }
    // }

    // let mut printed_file_count: usize = 0;
    // let strings = children_itr.map(|fp| fp.to_string().as_str()).collect();
    // let strings_to_be_printed: Vec<String> = buffer_spaces_vec(strings, level, space_count);
    // let string_count = strings_to_be_printed.len();
    // for line in strings_to_be_printed {
    //     println!("{}", line);
    //     printed_file_count += 1;
    //     if printed_file_count > max_subfiles_to_print {
    //         let remaining_files_count: usize = string_count - printed_file_count;
    //         let s: String = buffer_spaces_str(
    //             format!("<{} more files>", remaining_files_count).as_str(),
    //             level,
    //             space_count,
    //         );
    //         println!("{}", format_notify(s));
    //         break;
    //     }
    // }
}

fn handle_args(args: Vec<String>) -> Vec<FilePath> {
    let current_working_directory: FilePath = FilePath::get_cwd_path();

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
        paths_to_search.push(current_working_directory);
    }

    paths_to_search
}

fn check_found_file_count(max_count: usize) {}

fn main() {
    // collect cmd line args
    let args: Vec<String> = env::args().collect();

    let paths_to_search = handle_args(args);

    // unused file count warning
    // TODO: fix this
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
        let message: String = format!("Searching 👉👉 {}", path.to_string());
        println!("{}", format_title(message));
        println!("Is absolute: {}", path.is_absolute());
        // print_children(path, 0);
    }
}
