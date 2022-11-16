use compost::run::run_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let result = run_file(file_path);

        println!("{}", result);
    } else {
        println!("Specify a source file to run")
    }
}
