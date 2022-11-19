use compost::run::run_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let result = run_file(file_path);

        match result {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{}", error.message),
        }
    } else {
        println!("Specify a source file to run")
    }
}
