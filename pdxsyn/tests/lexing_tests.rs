use pdxsyn::{IntoLiteral, Lexer, Literal, Token};

#[test]
fn keyval_lexing_test() {
    use test_files::simple::KEYVAL;
    let lexer_output: Result<Vec<_>, _> = Lexer::new(KEYVAL).collect();
    let lexer_output = lexer_output.unwrap();

    assert_eq!(
        lexer_output[0..6],
        [
            // width = 6400
            Token::Literal("width".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::U32(6400)),
            Token::Whitespace("\n".to_owned()),
        ]
    );

    assert_eq!(
        lexer_output[6..12],
        [
            // height = 2560
            Token::Literal("height".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::U32(2560)),
            Token::Whitespace("\n".to_owned()),
        ]
    );
}

#[test]
fn comments_object_lexing_test() {
    use test_files::simple::COMMENTS_OBJECT;
    let lexer_output: Result<Vec<_>, _> = Lexer::new(COMMENTS_OBJECT).collect();
    let lexer_output = lexer_output.unwrap();

    assert_eq!(
        lexer_output[0..6],
        [
            // width = 6400
            Token::Literal("bahama_channel_area".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::BracketR,
            Token::Whitespace("\n".to_owned()),
        ]
    );

    assert_eq!(
        lexer_output[6..10],
        [
            // height = 2560
            Token::Literal("height".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::U32(2560)),
            Token::Whitespace("\n".to_owned()),
        ]
    );
}
