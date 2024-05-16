use clap;
#[derive(clap::Parser, Debug)]
#[clap(author = "Justin Burrill", about = "about text")]
pub struct Config {
    pub show_hidden_files: bool,
    pub continue_on_file_warning_default: bool,
    pub file_count_warning_cutoff: usize,
    pub tab_size: usize,
    pub max_depth: usize,
    pub max_subfiles: usize,
}
