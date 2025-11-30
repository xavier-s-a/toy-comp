use super::token::Token;
use crate::lexer::Lexer;

pub fn lex_identifier(lex: &mut Lexer, first: char) -> Token {
    let mut s = first.to_string();

    while let Some(c) = lex.peek() {
        if c.is_alphanumeric() || c == '_' {
            s.push(lex.next_char().unwrap());
        } else {
            break;
        }
    }

    match s.as_str() {
        "let" => Token::Let,
        "fn" => Token::Fn,
        "return" => Token::Return,
        "qbit" => Token::Qbit,
        "measure" => Token::Measure,

        "H" | "X" | "Y" | "Z" | "CX" | "CNOT" | "CCX" => Token::Gate(s),

        _ => Token::Ident(s),
    }
}
