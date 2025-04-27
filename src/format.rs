use colored::ColoredString;
use colored::Colorize;

pub fn format_info(s: String) -> ColoredString {
    let grey = 100;
    s.custom_color(colored::CustomColor {
        r: (grey),
        g: (grey),
        b: (grey),
    })
    .italic()
}

pub fn format_other_dir(s: String) -> ColoredString {
    (s + "/").yellow().italic()
}

pub fn format_error(s: String) -> ColoredString {
    s.red()
}

pub fn format_title(s: String) -> ColoredString {
    s.bold()
}

pub fn format_dir(s: String) -> ColoredString {
    (s + "/").bold().blue()
}

/// Adds a number of spaces to the start of a [`str`]
pub fn format_spacing_str(str: &str, depth: usize, tab_size: usize) -> String {
    generate_spacing('|', '-', tab_size, depth) + str
}

pub fn format_spacing_cstr(str: ColoredString, depth: usize, tab_size: usize) -> String {
    let first_char = '|';
    let fill_char = '-';
    format!(
        "{}{}",
        ColoredString::from(generate_spacing(first_char, fill_char, tab_size, depth)),
        str,
    )
}

fn generate_spacing(first_char: char, fill_char: char, tab_size: usize, depth: usize) -> String {
    String::from(first_char.to_string() + &fill_char.to_string().repeat(tab_size - 1)).repeat(depth)
}

// /// Adds spaces to a vector of [`String`] objects.
// fn buffer_spaces_vec(strings: Vec<&str>, depth: usize, tab_size: usize) -> Vec<String> {
//     let mut strings2: Vec<String> = Vec::new();
//     for str in strings {
//         strings2.push(buffer_spaces_str(str, depth, tab_size));
//         // println!("{} spaces added to {}", level * space_count, str.clone());
//     }
//     strings2
// }

pub fn find_indices_of_char(str: &str, ch: char) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    for (i, c) in str.chars().enumerate() {
        if c == ch {
            indices.push(i);
        }
    }
    indices
}
