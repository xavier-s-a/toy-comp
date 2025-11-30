use super::token::Token;
use crate::lexer::Lexer;

pub fn lex_number(lex: &mut Lexer, first: char) -> Token {
    let mut s = first.to_string();

    while let Some(c) = lex.peek() {
        if c.is_ascii_digit() || c == '.' {
            s.push(lex.next_char().unwrap());
        } else {
            break;
        }
    }

    Token::Number(s)
}

pub fn quantum_reduce(buf: &mut Vec<Token>) {
    loop {
        let mut changed = false;
        let mut i = 0;

        while i + 1 < buf.len() {
            let cancel = match (&buf[i], &buf[i + 1]) {
                (
                    Token::QOp { gate: g1, target: q1 },
                    Token::QOp { gate: g2, target: q2 },
                ) if q1 == q2 && g1 == g2 => {
                    g1 == "H" || g1 == "X" || g1 == "Y" || g1 == "Z"
                }
                _ => false,
            };

            if cancel {
                buf.remove(i + 1);
                buf.remove(i);
                changed = true;
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }

        if !changed {
            break;
        }
    }
}
