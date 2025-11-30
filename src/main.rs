mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let code = r#"
        qbit q;

        #[pe]
        H(q);
        H(q);

        #[nope]
        X(q);
        X(q);

        #[pe]
        CNOT(a,b);
        CNOT(a,b);

        measure q -> r;
    "#;

    let mut lex = Lexer::new(code);

    loop {
        let tok = lex.next_token();
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
