pub mod token;
pub mod classical;
pub mod quantum;
pub mod annotate;

pub use token::Token;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    pub pe_enabled: bool,
    gate_buffer: Vec<Token>,
    unread: Option<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            pe_enabled: true,
            gate_buffer: Vec::new(),
            unread: None,
        }
    }

   
    pub fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    pub fn next_char(&mut self) -> Option<char> {
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

    fn unread_token(&mut self, t: Token) {
        self.unread = Some(t);
    }

   
    fn next_raw_token(&mut self) -> Token {
        if let Some(t) = self.unread.take() {
            return t;
        }

        self.skip_ws();

        let c = match self.next_char() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match c {
            // Attribute: #[pe], #[nope], #[static], #[dynamic]
            '#' => {
                if self.peek() == Some('[') {
                    self.next_char(); // skip '['
                    let mut name = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == ']' {
                            self.next_char(); // skip ']'
                            break;
                        }
                        name.push(self.next_char().unwrap());
                    }
                    Token::Attr(name.trim().to_string())
                } else {
                    Token::Unknown("#".into())
                }
            }

            ch if ch.is_ascii_digit() => quantum::lex_number(self, ch),

            ch if ch.is_alphanumeric() || ch == '_' => classical::lex_identifier(self, ch),

            
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '*' => Token::Star,
            '/' => Token::Slash,
            ',' => Token::Comma,
            ';' => Token::Semicolon,

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

    fn read_gate_call_or_gate(&mut self, gname: String) -> Token {
        let t1 = self.next_raw_token();
        let t2 = self.next_raw_token();
        let t3 = self.next_raw_token();
        let t4 = self.next_raw_token();

        if let (
            Token::LParen,
            Token::Ident(qname),
            Token::RParen,
            Token::Semicolon,
        ) = (&t1, &t2, &t3, &t4)
        {
            Token::QOp {
                gate: gname,
                target: qname.clone(),
            }
        } else {
            self.unread_token(t4);
            self.unread_token(t3);
            self.unread_token(t2);
            self.unread_token(t1);
            Token::Gate(gname)
        }
    }

    fn next_raw_or_qop(&mut self) -> Token {
        let t = self.next_raw_token();
        if let Token::Gate(gname) = t {
            self.read_gate_call_or_gate(gname)
        } else {
            t
        }
    }

  
    pub fn next_token(&mut self) -> Token {
        if !self.gate_buffer.is_empty() {
            return self.gate_buffer.remove(0);
        }

        loop {
            let t0 = self.next_raw_token();

            match t0 {
                Token::Attr(name) => {
                    match name.as_str() {
                        "pe" | "static" => self.pe_enabled = true,
                        "nope" | "dynamic" => self.pe_enabled = false,
                        _ => {}
                    }
                    continue;
                }

                Token::Gate(gname) => {
                    let first = self.read_gate_call_or_gate(gname);

                    if let Token::QOp { .. } = first {
                        if self.pe_enabled {
                            self.gate_buffer.push(first);
                            quantum::quantum_reduce(&mut self.gate_buffer);

                            loop {
                                let t_next = self.next_raw_or_qop();

                                match t_next {
                                    Token::QOp { .. } if self.pe_enabled => {
                                        self.gate_buffer.push(t_next);
                                        quantum::quantum_reduce(&mut self.gate_buffer);
                                    }

                                    Token::Attr(name) => {
                                        match name.as_str() {
                                            "pe" | "static" => self.pe_enabled = true,
                                            "nope" | "dynamic" => self.pe_enabled = false,
                                            _ => {}
                                        }
                                        break;
                                    }

                                    other => {
                                        if other != Token::EOF {
                                            self.unread_token(other);
                                        }
                                        break;
                                    }
                                }
                            }

                          
                            if !self.gate_buffer.is_empty() {
                                return self.gate_buffer.remove(0);
                            } else {
                                // All gates cancelled out.
                                continue;
                            }
                        } else {
                            // PE disabled: emit QOp directly, no cancellation.
                            return first;
                        }
                    } else {
                        return first;
                    }
                }

               
                other => return other,
            }
        }
    }
}
