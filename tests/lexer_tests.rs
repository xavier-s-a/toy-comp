use toy_comp::lexer::{Lexer, Token};

#[test]
fn test_quantum_gate_cancellation() {
    let code = r#"
        #[pe]
        H(q);
        H(q);
        X(q);
        X(q);
        #[nope]
        X(q);
    "#;

    let mut lex = Lexer::new(code);

    let mut tokens = vec![];
    loop {
        let t = lex.next_token();
        if t == Token::EOF { break; }
        tokens.push(t);
    }

    assert!(!tokens.contains(&Token::Gate("H".into())));
    assert!(!tokens.contains(&Token::Gate("X".into())));

    assert!(tokens.contains(&Token::Gate("X".into()))); // the #[nope] part
}
