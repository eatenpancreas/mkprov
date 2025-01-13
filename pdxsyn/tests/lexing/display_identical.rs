use pdxsyn::{Lexer, LexerError};

fn stringify(lexer: Lexer) -> Result<String, LexerError> {
    Ok(lexer
        .map(|res| res.map(|token| token.to_string()))
        .collect::<Result<Vec<_>, LexerError>>()?
        .join(""))
}

#[test]
fn simple_output_identical() {
    use test_files::simple::*;
    assert_eq!(KEYVAL, stringify(Lexer::new(KEYVAL)).unwrap());
    assert_eq!(
        COMMENTS_OBJECT,
        stringify(Lexer::new(COMMENTS_OBJECT)).unwrap()
    );
}
