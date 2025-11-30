use super::token::Token;

pub fn lex_identifier(lex: &mut crate::lexer::Lexer, first: char) -> Token {
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

        // quantum keywords
        "qbit" => Token::Qbit,
        "measure" => Token::Measure,

        // quantum gates
        "H" | "X" | "Y" | "Z" | "CX" | "CNOT" | "CCX" =>
            Token::Gate(s.to_string()),

        _ => Token::Ident(s),
    }
}
