use pdxsyn::*;

#[test]
fn int_lexing_test() {
    let lexer_output = Lexer::new("0").next().unwrap().unwrap();
    assert_eq!(lexer_output, Token::Literal(Literal::I64(0)));
}

#[test]
fn float_lexing_test() {
    let lexer_output = Lexer::new("0.000").next().unwrap().unwrap();
    assert_eq!(
        lexer_output,
        Token::Literal(Literal::F32(0.0, Precision::new(3)))
    );
}
#[test]
fn float_neg_lexing_test() {
    let lexer_output = Lexer::new("-0.110").next().unwrap().unwrap();
    assert_eq!(
        lexer_output,
        Token::Literal(Literal::F32(-0.11, Precision::new(3)))
    );
}

#[test]
fn date_lexing_test() {
    let lexer_output = Lexer::new("0000.02.01").next().unwrap().unwrap();
    assert_eq!(
        lexer_output,
        Token::Literal(Literal::Date(Date::unchecked(0, 2, 1)))
    );
}
