use std::mem;

use anyhow::{bail, Result};

use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let first = lexer.next_token();
        Self {
            lexer,
            lookahead: first,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut functions = Vec::new();

        while !matches!(self.lookahead, Token::EOF) {
            functions.push(self.parse_function()?);
        }

        Ok(Program { functions })
    }

    fn bump(&mut self) -> Token {
        mem::replace(&mut self.lookahead, self.lexer.next_token())
    }

    fn expect_token(&mut self, expected: &Token) -> Result<Token> {
        let tok = self.bump();
        if &tok == expected {
            Ok(tok)
        } else {
            bail!("expected {:?}, found {:?}", expected, tok);
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        let tok = self.bump();
        if let Token::Ident(name) = tok {
            Ok(name)
        } else {
            bail!("expected identifier, found {:?}", tok);
        }
    }

    fn expect_number(&mut self) -> Result<String> {
        let tok = self.bump();
        if let Token::Number(s) = tok {
            Ok(s)
        } else {
            bail!("expected number literal, found {:?}", tok);
        }
    }

    fn current(&self) -> &Token {
        &self.lookahead
    }

    fn parse_function(&mut self) -> Result<Function> {
        match self.bump() {
            Token::Fn => {}
            other => bail!("expected 'fn', found {:?}", other),
        }

        let name = self.expect_ident()?;
        self.expect_token(&Token::LParen)?;

        let mut params = Vec::new();
        if !matches!(self.current(), Token::RParen) {
            loop {
                let p = self.expect_ident()?;
                params.push(p);

                if matches!(self.current(), Token::Comma) {
                    self.bump();
                    continue;
                } else {
                    break;
                }
            }
        }

        self.expect_token(&Token::RParen)?;

        let pe_enabled = self.lexer.pe_enabled;
        let body = self.parse_block()?;

        Ok(Function {
            name,
            params,
            body,
            pe_enabled,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        self.expect_token(&Token::LBrace)?;
        let mut stmts = Vec::new();

        while !matches!(self.current(), Token::RBrace | Token::EOF) {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        self.expect_token(&Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.current() {
            Token::Let => self.parse_let_stmt(),
            Token::Qbit => self.parse_qbit_decl(),
            Token::Return => self.parse_return_stmt(),
            Token::Measure => self.parse_measure_stmt(),
            Token::QOp { .. } => self.parse_qop_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        self.bump();
        let name = self.expect_ident()?;
        self.expect_token(&Token::Assign)?;
        let init = self.parse_expr()?;
        self.expect_token(&Token::Semicolon)?;
        Ok(Stmt::Let { name, init })
    }

    fn parse_qbit_decl(&mut self) -> Result<Stmt> {
        self.bump();
        let name = self.expect_ident()?;
        self.expect_token(&Token::Semicolon)?;
        Ok(Stmt::QbitDecl { name })
    }

    fn parse_qop_stmt(&mut self) -> Result<Stmt> {
        let tok = self.bump();
        if let Token::QOp { gate, target } = tok {
            Ok(Stmt::QOp { gate, target })
        } else {
            bail!("expected quantum operation, found {:?}", tok);
        }
    }

    fn parse_measure_stmt(&mut self) -> Result<Stmt> {
        self.bump();
        let target = self.expect_ident()?;

        let mut classical = None;
        if matches!(self.current(), Token::Arrow) {
            self.bump();
            let cname = self.expect_ident()?;
            classical = Some(cname);
        }

        self.expect_token(&Token::Semicolon)?;
        Ok(Stmt::Measure { target, classical })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.bump();
        if matches!(self.current(), Token::Semicolon) {
            self.bump();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr()?;
            self.expect_token(&Token::Semicolon)?;
            Ok(Stmt::Return(Some(expr)))
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;
        self.expect_token(&Token::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr> {
        let mut expr = self.parse_mul_div()?;

        loop {
            match self.current() {
                Token::Plus => {
                    self.bump();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary {
                        op: BinOp::Add,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                Token::Minus => {
                    self.bump();
                    let rhs = self.parse_mul_div()?;
                    expr = Expr::Binary {
                        op: BinOp::Sub,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_mul_div(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current() {
                Token::Star => {
                    self.bump();
                    let rhs = self.parse_primary()?;
                    expr = Expr::Binary {
                        op: BinOp::Mul,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                Token::Slash => {
                    self.bump();
                    let rhs = self.parse_primary()?;
                    expr = Expr::Binary {
                        op: BinOp::Div,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let tok = self.bump();
        match tok {
            Token::Number(s) => {
                let v: i64 = s.parse().map_err(|e| anyhow::anyhow!("invalid integer literal {}: {}", s, e))?;
                Ok(Expr::Number(v))
            }
            Token::Ident(name) => {
                if matches!(self.current(), Token::LParen) {
                    self.bump();
                    let mut args = Vec::new();

                    if !matches!(self.current(), Token::RParen) {
                        loop {
                            let arg = self.parse_expr()?;
                            args.push(arg);

                            if matches!(self.current(), Token::Comma) {
                                self.bump();
                                continue;
                            } else {
                                break;
                            }
                        }
                    }

                    self.expect_token(&Token::RParen)?;
                    Ok(Expr::Call { callee: name, args })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let inner = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(inner)
            }
            other => bail!("unexpected token in expression: {:?}", other),
        }
    }
}
