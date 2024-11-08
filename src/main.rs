mod config;
mod find;
mod format;
mod input;
use crate::config::*;
mod pathj;
use crate::pathj::path::*;
use format::*;
use std::env;
use std::ffi::OsString;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::Instant;

// Turn this into a wrapper function for a find_children or something? ?? i'm not happy with the way i handle the errors here
fn print_children(dir: &mut Path, depth: usize, config: &Config) -> io::Result<()> {
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
        let x = dir.get_direct_child_file_count();
        println!("children of {}: {}", dir.to_str(), x);
        x
    };

    let files = dir.clone_child_files();
    let folders = dir.clone_child_dirs();
    // needs to handle 3 cases:
    // - is a dir with children
    // - is a dir without children
    // - is a file

    // print subfolders first
    for mut subfolder in folders {
        // Print name of the folder
        println!(
            "{}",
            format_spacing_cstr(
                format_dir(subfolder.get_item_name().to_owned()),
                depth,
                tab_size
            )
        );
        if subfolder.is_empty_dir() {
            println!(
                "{}",
                format_spacing_cstr(format_info("<Empty dir>".to_owned()), depth + 1, tab_size)
            )
        }
        // Followed by it's children
        else {
            let s: Option<&str> = match print_children(&mut subfolder, depth + 1, config) {
                Ok(()) => None,
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => {
                        println!("permission denied");
                        Some("<Permission Error>")
                    }
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
            format_spacing_str(subfile.get_item_name(), depth, tab_size)
        );
    }
    // if we skipped some, then say so here
    if max_subfiles_to_print < dir.get_direct_child_file_count() {
        let unprinted_file_count = dir.get_direct_child_file_count() - max_subfiles_to_print;
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

fn handle_args(args: Vec<String>) -> Vec<Path> {
    let current_working_directory: PathBuf = pathj::utils::get_cwd_pathbuf();

    // vector to hold the paths to be searched
    let mut paths_to_search: Vec<Path> = Vec::new();

    // if there are args given by the user,
    if args.len() > 0 {
        for arg in args {
            // copy path from the cmd line arguments
            let path_ext = PathBuf::from(arg);
            // replace back slashes from user inputwith forward slashes
            //path_ext = path_ext.replace("\\", "/");
            // push the modified path ending to the cwd
            let mut pb = current_working_directory.clone();
            // path.push("/");
            pb.push(path_ext);
            if pb.exists() {
                if pb.is_file() {
                    print!(
                        "Can't search {:?} because it is a file, not a directory",
                        pb.to_str().unwrap()
                    );
                } else {
                    paths_to_search.push(Path::from_pathbuf(&pb));
                }
            } else {
                print!("No such path: {:?}", pb.to_str().unwrap());
            }
        }
    } else {
        paths_to_search.push(Path::from_pathbuf(&current_working_directory));
    }

    paths_to_search
}

fn check_found_file_count(path: &mut Path, cfg: &Config) -> bool {
    let continue_by_default = cfg.continue_on_file_warning_default;
    let now = Instant::now();
    let descendant_count = (*path).get_child_file_count_recursive();
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

// fn assemble_dir(path: PathBuf, depth: usize, max_depth: usize) -> io::Result<Directory> {
//     let mut subdirs: Vec<Directory> = Vec::new();
//     let folders: Vec<Directory> = path.subdirs;
//     for folder in folders {
//         subdirs.push(assemble_dir(&folder, depth + 1, max_depth)?);
//     }
//     let files: Vec<File> = path.subfiles;
//     Ok(Directory {
//         path: path.clone(),
//         subdirs,
//         subfiles: files,
//     })
// }

fn print_dir(dir: &Path, depth: usize, config: Config) {}

fn main() {
    // collect cmd line args
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let paths_to_search = handle_args(args);
    let config = Config {
        show_hidden_files: false,
        continue_on_file_warning_default: true,
        file_count_warning_cutoff: 100,
        tab_size: 4,
        max_depth: 5,
        max_subfiles: 5,
    };

    // let config = Config::parse();

    // search each path
    for mut path in paths_to_search {
        if check_found_file_count(&mut path, &config) {
            println!();
            continue;
        }

        let message: String = format!("Searching {}", &path.to_str());
        println!("{}", format_title(message));
        let start = Instant::now();
        let _ = print_children(&mut path, 0, &config);
        let duration = start.elapsed();
        println!(
            "{}",
            format_info(format!("(completed in {:?}s)", duration.as_secs_f32()))
        );
    }
}

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use crate::pathj::directory;

//     #[test]
//     fn count_descendants() {
//         let path = PathBuf::from(r"C:\src\lll\test1");
//         let directory = directory::Directory::from_pathbuf(&path);
//         println!("{:?}", path);
//         assert_eq!(directory.get_descendant_count(), 11);
//     }
// }
