mod format;
mod input;
use format::*;
use std::env;
mod paths;
use crate::paths::*;
mod config;
use crate::config::*;

fn print_children(path: &FilePath, depth: usize, config: &Config) {
    let tab_size: usize = config.tab_size;
    let max_depth: usize = config.max_depth;
    if depth > max_depth {
        println!(
            "{}",
            format_spacing_cstr(
                format_info("<Max depth reached>".to_string()),
                depth,
                tab_size
            )
        );
        return;
    }
    let max_subfiles_to_print: usize = config.max_subfiles;
    let children_itr = path.get_children().into_iter();
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
            "{}",
            format_spacing_cstr(format_dir(subfolder.get_item_name()), depth, tab_size)
        );
        // Followed by it's children
        print_children(&subfolder, depth + 1, config);
    }

    // followed by subfiles
    for x in 0..max_subfiles_to_print {
        if x == files.len() {
            break;
        }
        let subfile = &files[x];
        println!(
            "{}",
            format_spaces_str(subfile.get_item_name().as_str(), depth, tab_size)
        );
    }
    // if we skipped some, then say so here
    if max_subfiles_to_print < path.get_immediate_child_file_count() {
        let unprinted_file_count = path.get_immediate_child_file_count() - max_subfiles_to_print;
        println!(
            "{}",
            format_spacing_cstr(
                format_info(format!("<{} more files>", unprinted_file_count)),
                depth + 1,
                tab_size
            )
        );
    }
}

fn handle_args(args: Vec<String>) -> Vec<FilePath> {
    let current_working_directory: FilePath = FilePath::get_cwd_path();

    // vector to hold the paths to be searched
    let mut paths_to_search: Vec<FilePath> = Vec::new();

    // if there are args given by the user,
    if args.len() > 0 {
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

fn check_found_file_count(path: &FilePath, max_count: usize) -> bool {
    let continue_by_default = true;
    let descendant_count = path.get_descendant_count();

    if descendant_count > max_count {
        let prompt = format!("Warning: {} items - continue?", descendant_count);
        if input::bool_input(&prompt, continue_by_default) {
            // keep going :)
            return false;
        } else {
            // dont keep going :(
            return true;
        }
    }
    return false;
}

fn main() {
    // collect cmd line args
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    let paths_to_search = handle_args(args);
    let config = Config {
        show_hidden_files: false,
        file_count_warning_cutoff: 50,
        tab_size: 4,
        max_depth: 6,
        max_subfiles: 10,
    };

    // search each path
    for path in paths_to_search {
        if check_found_file_count(&path, config.file_count_warning_cutoff) {
            println!("bye 🖐");
            continue;
        }
        let message: String = format!("Searching 👉 {}", &path.to_string());
        println!("{}", format_title(message));
        print_children(&path, 0, &config);
    }
}
