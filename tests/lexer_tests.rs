use qxad::lexer::{Lexer, Token};

fn collect_tokens(mut lex: Lexer) -> Vec<Token> {
    let mut toks = Vec::new();
    loop {
        let t = lex.next_token();
        let is_eof = t == Token::EOF;
        toks.push(t);
        if is_eof {
            break;
        }
    }
    toks
}

#[test]
fn test_let_binding_lexing() {
    let src = "let x = 42;";
    let lex = Lexer::new(src);
    let toks = collect_tokens(lex);

    assert_eq!(toks[0], Token::Let);
    assert_eq!(toks[1], Token::Ident("x".into()));
    assert_eq!(toks[2], Token::Assign);
    assert_eq!(toks[3], Token::Number("42".into()));
    assert_eq!(toks[4], Token::Semicolon);
    assert_eq!(toks[5], Token::EOF);
}

#[test]
fn test_simple_fn_like_rust() {
    let src = "fn add(x, y) { return x + y; }";
    let lex = Lexer::new(src);
    let toks = collect_tokens(lex);

    assert!(toks.contains(&Token::Fn));
    assert!(toks.contains(&Token::Ident("add".into())));
    assert!(toks.contains(&Token::Return));
    assert!(toks.contains(&Token::Plus));
    assert!(toks.contains(&Token::LBrace));
    assert!(toks.contains(&Token::RBrace));
    assert!(toks.contains(&Token::EOF));
}

#[test]
fn test_hybrid_classical_and_quantum() {
    let src = r#"
        let steps = 2;
        qbit q;
        #[pe]
        H(q);
        H(q);
        X(q);
        X(q);
        #[nope]
        X(q);
    "#;

    let lex = Lexer::new(src);
    let toks = collect_tokens(lex);

    assert!(toks.contains(&Token::Let));
    assert!(toks.contains(&Token::Ident("steps".into())));
    assert!(toks.contains(&Token::Qbit));

    let qops: Vec<_> = toks
        .iter()
        .filter_map(|t| {
            if let Token::QOp { gate, target } = t {
                Some((gate.clone(), target.clone()))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(qops.len(), 1);
    assert_eq!(qops[0].0, "X");
    assert_eq!(qops[0].1, "q");
}
