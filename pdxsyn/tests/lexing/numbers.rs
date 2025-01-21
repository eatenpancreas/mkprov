use pdxsyn::*;

#[test]
fn int_lexing_test() {
    let lexer_output = Lexer::new("0").next().unwrap().unwrap();
    assert_eq!(lexer_output, Token::Literal(Literal::I64(0)));
}

#[test]
fn float_lexing_test() {
    let lexer_output = Lexer::new("0.000").next().unwrap().unwrap();
    assert_eq!(lexer_output, Token::Literal(Literal::F32(0.0, Precision::new(3))));
}
#[test]
fn float_neg_lexing_test() {
    let lexer_output = Lexer::new("-0.110").next().unwrap().unwrap();
    assert_eq!(lexer_output, Token::Literal(Literal::F32(-0.11, Precision::new(3))));
}

#[test]
fn date_lexing_test() {
    let lexer_output = Lexer::new("0000.02.01").next().unwrap().unwrap();
    assert_eq!(lexer_output, Token::Literal(Literal::Date(Date::unchecked(0, 2, 1, true))));
}

// TODO: remove
// #[test]
// fn massive_files_output_print() {
//     let string = test_files::massive::ACHIEVEMENTS;
//     let mut lex_output = Lexer::new(string).collect::<Result<Vec<_>, _>>().unwrap();
//     for token in lex_output {
//         print!("<{token}>");
//     }
// }
