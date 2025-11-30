#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Attr(String),
    Let,
    Fn,
    Return,
    Qbit,
    Measure,
    Gate(String),
    Ident(String),
    Int(i64),
    Plus, Minus, Star, Slash,
    Assign, Arrow,
    LParen, RParen,
    LBrace, RBrace,
    Semicolon, Comma,
    EOF,
    Unknown(String),
}
