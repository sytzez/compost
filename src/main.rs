use std::env;
use compost::run::run_file;

mod class;
mod definition;
mod expression;
mod instance;
mod lett;
mod module;
mod parser;
mod raw_value;
mod scope;
mod strukt;
mod token;
mod tokenizer;
mod trayt;
mod typ;
mod run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let result = run_file(file_path);

        println!("{}", result);
    } else {
        println!("Specify a source file to run")
    }
}
