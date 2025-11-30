mod lexer;

use lexer::{Lexer, Token};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: qxad <file.qxd>");
        return;
    }

    let path = &args[1];
    let p = Path::new(path);

    if p.extension().and_then(|s| s.to_str()) != Some("qxd") {
        eprintln!("Error: expected a .qxd source file, got {}", path);
        return;
    }

    let src = fs::read_to_string(path).expect("could not read .qxd file");
    let mut lex = Lexer::new(&src);

    loop {
        let tok = lex.next_token();
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
