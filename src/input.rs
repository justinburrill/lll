use std::io::{self, BufRead, Error};

// pub fn bool_input(prompt: String, default: bool) -> bool {
//     let yes_statements: Vec<&str> = vec!["y", "yes", "yeah", "yup"];
//     let no_statements: Vec<&str> = vec!["n", "yes", "nah", "nope"];
//     println!("{} {}", prompt, if default { "(Y/n)" } else { "(N/y)" });
//     loop {
//         let user_input: String = input(String::from(">> ")).unwrap();
//         if user_input == String::from("\n") {
//             return default;
//         } else if yes_statements.contains(&user_input.to_lowercase().as_str()) {
//             return true;
//         } else if no_statements.contains(&user_input.to_lowercase().as_str()) {
//             return false;
//         } else {
//             println!("Unrecognized input");
//             continue;
//         }
//     }
// }

fn input(prompt: String) -> String {
    print!("{}", prompt);
    let mut input_string = String::new();

    match io::stdin().read_line(&mut input_string) {
        Ok(_) => input_string,
        Err(error) => panic!("Problem reading input 😭😭: {:?}", error),
    }
}
