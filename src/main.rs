use std::env;

use unsafe_finder::prelude::traverse_dir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <crate dir name>");
        return;
    }

    traverse_dir(args[1].to_string());
}
