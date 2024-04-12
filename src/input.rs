// use std::io::{self, BufRead, Error};

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

// fn input(prompt: String) -> Result<String, Error> {
//     print!("{}", prompt);
//     // let stdin = io::stdin();
//     // let line1 = stdin.lock().lines().next().unwrap().unwrap();
//     let mut user_in = String::new();
//     let stdin = io::stdin();
//     stdin
//         .lock()
//         .read_line(&mut user_in)
//         .expect("Could not read line");
//     // let user_in = match user_in {
//     //     Ok(str) => str,
//     //     Err(error) => panic!("Problem reading input 😭😭: {:?}", error),
//     // };
//     Ok(user_in)
// }
