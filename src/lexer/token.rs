#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(String),

    Let,
    Fn,
    Return,
    Qbit,
    Measure,

    Gate(String),
    QOp { gate: String, target: String },

    Attr(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Semicolon,
    Arrow,
    Assign,

    EOF,
    Unknown(String),
}
