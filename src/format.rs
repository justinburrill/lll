use colored::ColoredString;
use colored::Colorize;

pub fn format_notify(s: String) -> ColoredString {
    s.italic().bold().dimmed()
}

pub fn format_title(s: String) -> ColoredString {
    s.bold()
}

pub fn format_dir(s: String) -> ColoredString {
    s.bold().blue()
}

/// Adds a number of spaces to the start of a [`String`]
pub fn buffer_spaces_str(str: &str, level: usize, space_count: usize) -> String {
    " ".repeat(level * space_count) + str
}

/// Adds spaces to a vector of [`String`] objects.
fn buffer_spaces_vec(strings: Vec<&str>, level: usize, space_count: usize) -> Vec<String> {
    let mut strings2: Vec<String> = Vec::new();
    for str in strings {
        strings2.push(buffer_spaces_str(str, level, space_count));
        // println!("{} spaces added to {}", level * space_count, str.clone());
    }
    strings2
}
