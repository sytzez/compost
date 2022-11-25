use compost::run::run_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        println!("{}", run_file(file_path))
    } else {
        println!("Specify a source file to run")
    }
}
