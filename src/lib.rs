#[macro_use] extern crate shrinkwraprs;

pub mod util {
    use std::{fs, io::Error, process};
    use text_io::read;
    pub fn run_file(path: String) {
        if let Ok(file) = fs::read_to_string(path) {
            let _ = match run(file) {
                Err(_) => process::exit(65),
                Ok(_) => (),
            };
        }
    }

    pub fn run_prompt(){
        loop {
            println!("> ");
            let line: String = read!("{}\n");
            match run(line) {
                Ok(_) => (),
                Err(_) => continue,
            };
        }
    }

    pub fn run(source: String) -> Result<(), Box<Error>>{
        let tokens = Tokens::from_source(source);

        for token in tokens.0 {
            println!("{:?}", token);
        }

        Ok(())
    }

    fn error(line: usize, message: String) {
        report(line, "", message);
    }

    fn report(line: usize, location: &str, message: String) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
    }

    #[derive(Debug)]
    enum Token {
        Token(String),
    }

    #[derive(Shrinkwrap)]
    struct Tokens(Vec<Token>);

    impl Tokens {
        fn from_source(source: String) -> Tokens {
            Tokens(vec![Token::Token(source)])
        }
    }

    pub struct Lox {
        pub err: bool,
    }
}