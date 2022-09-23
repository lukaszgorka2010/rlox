use std::{env, process};
use rlox::util::*;
use rand::Rng;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("{:#?}", args);

    let lox = Lox {err: false};

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args.pop().unwrap());
    } else {
        run_prompt();
    }
}
