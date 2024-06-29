mod tree_element;
mod cli_args;


use tree_element::TreeElement;
use std::time::Instant;
use std::env;
use cli_args::CliArgs;
use clap::Parser;


fn main() {
    let args = CliArgs::parse();
    let mut dir = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_e) => {
            panic!("Error getting current directory! Consider passing it explicitly.")
        }

    };
    let start_point = Instant::now();

    if let Some(path) = args.get_path() {
        dir = path;
    }

    let storage = TreeElement::new(String::from(dir));
    storage.print_sorted_tree();

    if args.get_time() {
        let elapsed = start_point.elapsed();
        println!("Time elapsed: {:.2?}", elapsed);
    }
}
