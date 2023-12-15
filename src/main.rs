use std::env;

fn get_cwd() -> String {
    let path = env::current_dir();
    path.unwrap().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path: String = get_cwd();
    if args.len() != 0 {
        path.push();
    }
}
