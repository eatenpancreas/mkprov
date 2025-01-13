use pdxsyn::{Lexer, LexerError};

fn stringify(lexer: Lexer) -> Result<String, LexerError> {
    Ok(lexer
        .map(|res| res.map(|token| token.to_string()))
        .collect::<Result<Vec<_>, LexerError>>()?
        .join(""))
}

#[test]
fn numbers_output_identical() {
    assert_eq!("0", stringify(Lexer::new("0")).unwrap());
    assert_eq!("0.000", stringify(Lexer::new("0.000")).unwrap());
    assert_eq!("-0.110", stringify(Lexer::new("-0.110")).unwrap());
    assert_eq!("0000.02.01", stringify(Lexer::new("0000.02.01")).unwrap());
}

#[test]
fn simple_output_identical() {
    let string = test_files::simple::KEYVAL;
    assert_eq!(string, stringify(Lexer::new(string)).unwrap());
    let string = test_files::simple::COMMENTS_OBJECT;
    assert_eq!(string, stringify(Lexer::new(string)).unwrap());
}
