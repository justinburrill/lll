mod format;
mod input;
use format::*;
use std::env;
use std::io;
use std::io::ErrorKind;
use std::time::Instant;
mod paths;
use crate::paths::*;
mod config;
use crate::config::*;

// Turn this into a wrapper function for a find_children or something? ?? i'm not happy with the way i handle the errors here
fn print_children(path: &FilePath, depth: usize, config: &Config) -> io::Result<()> {
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
        return Ok(());
    }
    let max_subfiles_to_print: usize = if depth != 0 {
        config.max_subfiles
    } else {
        path.get_child_file_count()
    };

    let files = path.get_child_files()?;
    let folders = path.get_child_folders()?;
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
        if subfolder.is_empty_dir()? {
            println!(
                "{}",
                format_spacing_cstr(format_info("<Empty dir>".to_owned()), depth + 1, tab_size)
            )
        }
        // Followed by it's children
        else {
            let s: Option<&str> = match print_children(&subfolder, depth + 1, config) {
                Ok(()) => None,
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => Some("<Permission Error>"),
                    _ => Some("<Unknown Error>"),
                },
            };
            if s.is_some() {
                println!(
                    "{}",
                    format_spacing_cstr(format_error(s.unwrap().to_owned()), depth + 1, tab_size)
                )
            }
        }
    }

    // followed by subfiles
    for x in 0..max_subfiles_to_print {
        if x == files.len() {
            break;
        }
        let subfile = &files[x];
        println!(
            "{}",
            format_spacing_str(subfile.get_item_name().as_str(), depth, tab_size)
        );
    }
    // if we skipped some, then say so here
    if max_subfiles_to_print < path.get_child_file_count() {
        let unprinted_file_count = path.get_child_file_count() - max_subfiles_to_print;
        println!(
            "{}",
            format_spacing_cstr(
                format_info(format!("<{} more files>", unprinted_file_count)),
                depth,
                tab_size
            )
        );
    }
    Ok(())
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

fn check_found_file_count(path: &FilePath, cfg: &Config) -> bool {
    let continue_by_default = cfg.continue_on_file_warning_default;
    let now = Instant::now();
    let descendant_count = path.get_descendant_count();
    let max_count = cfg.file_count_warning_cutoff;

    if descendant_count > max_count {
        let time = now.elapsed().as_secs_f32();
        let prompt = format!("Warning: {} items - continue?", descendant_count);
        let time_info = format_info(format!("(counted in {}s)", time.to_string()));
        if input::bool_input(&format!("{} {}", prompt, time_info), continue_by_default) {
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
        continue_on_file_warning_default: true,
        file_count_warning_cutoff: 50,
        tab_size: 4,
        max_depth: 5,
        max_subfiles: 10,
    };

    // search each path
    for path in paths_to_search {
        if check_found_file_count(&path, &config) {
            println!("bye 🖐");
            continue;
        }
        let message: String = format!("Searching {}", &path.to_string());
        println!("{}", format_title(message));
        let start = Instant::now();
        let _ = print_children(&path, 0, &config);
        let duration = start.elapsed();
        println!(
            "{}",
            format_info(format!("(completed in {:?}s)", duration.as_secs_f32()))
        );
    }
}
