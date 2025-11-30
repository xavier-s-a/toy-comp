use super::token::Token;

pub fn lex_number(lex: &mut crate::lexer::Lexer, first: char) -> Token {
    let mut s = first.to_string();

    while let Some(c) = lex.peek() {
        if c.is_ascii_digit() {
            s.push(lex.next_char().unwrap());
        } else {
            break;
        }
    }
    Token::Int(s.parse().unwrap())
}

// Quantum Peephole Optimization
pub fn quantum_reduce(buf: &mut Vec<Token>) {
    loop {
        let changed = if buf.len() >= 2 {
            match (&buf[buf.len() - 2], &buf[buf.len() - 1]) {
                (Token::Gate(a), Token::Gate(b)) if a == b => {
                    buf.pop(); buf.pop();
                    true
                }
                _ => false
            }
        } else {
            false
        };

        if !changed { break; }
    }
}
