pub mod token;
pub mod classical;
pub mod quantum;
pub mod annotate;

pub use token::Token;
pub use annotate::Annotation;
pub use quantum::quantum_reduce;
pub use classical::lex_identifier;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    pub pe_enabled: bool,
    gate_buffer: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            pe_enabled: true,
            gate_buffer: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn next_raw_token(&mut self) -> Token {
        self.skip_ws();

        let c = match self.next_char() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match c {

            '#' => {
                if self.peek() == Some('[') {
                    self.next_char();
                    let mut name = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == ']' {
                            self.next_char();
                            break;
                        }
                        name.push(self.next_char().unwrap());
                    }
                    return Token::Attr(name.trim().to_string());
                }
                Token::Unknown("#".into())
            }

            ch if ch.is_ascii_digit() => quantum::lex_number(self, ch),

            ch if ch.is_alphanumeric() => classical::lex_identifier(self, ch),

            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semicolon,
            '+' => Token::Plus,
            '*' => Token::Star,
            '/' => Token::Slash,
            ',' => Token::Comma,
            '-' => {
                if self.peek() == Some('>') {
                    self.next_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '=' => Token::Assign,

            other => Token::Unknown(other.to_string()),
        }
    }

    // ---------------- OPTIMIZED TOKENIZATION ----------------
    pub fn next_token(&mut self) -> Token {
        if !self.gate_buffer.is_empty() {
            return self.gate_buffer.remove(0);
        }

        let t = self.next_raw_token();

        if let Token::Attr(name) = &t {
            match name.as_str() {
                "pe" | "static" => self.pe_enabled = true,
                "nope" | "dynamic" => self.pe_enabled = false,
                _ => {}
            }
            return self.next_token(); 
        }

        // Quantum gate + PE
        if let Token::Gate(_) = &t {
            if self.pe_enabled {
                self.gate_buffer.push(t);
                quantum::quantum_reduce(&mut self.gate_buffer);

                if self.gate_buffer.is_empty() {
                    return self.next_token();
                }
                return self.gate_buffer.remove(0);
            }
        }

        t
    }
}
