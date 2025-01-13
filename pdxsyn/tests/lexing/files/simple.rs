use pdxsyn::*;

#[test]
fn keyval_lexing_test() {
    use test_files::simple::KEYVAL;
    let lexer_output: Result<Vec<_>, _> = Lexer::new(KEYVAL).collect();
    let lexer_output = lexer_output.unwrap();

    // width = 6400
    assert_eq!(
        lexer_output[0..6],
        [
            Token::Literal("width".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::I64(6400)),
            Token::Whitespace("\n".to_owned()),
        ]
    );

    // height = 2560
    assert_eq!(
        lexer_output[6..12],
        [
            Token::Literal("height".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::I64(2560)),
            Token::Whitespace("\n".to_owned()),
        ]
    );
}

#[test]
fn comments_object_lexing_test() {
    use test_files::simple::COMMENTS_OBJECT;
    let lexer_output: Result<Vec<_>, _> = Lexer::new(COMMENTS_OBJECT).collect();
    let lexer_output = lexer_output.unwrap();

    // without comments
    assert_eq!(
        lexer_output[0..16],
        [
            Token::Literal("bahama_channel_area".to_owned().into_literal()),
            Token::Whitespace(" ".to_owned()),
            Token::Equals,
            Token::Whitespace(" ".to_owned()),
            Token::BracketL,
            Token::Whitespace("\n\t".to_owned()),
            Token::Literal(Literal::I64(1503)),
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::I64(1505)),
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::I64(1524)),
            Token::Whitespace(" ".to_owned()),
            Token::Literal(Literal::I64(1525)),
            Token::Whitespace("\n".to_owned()),
            Token::BracketR,
            Token::Whitespace("\n\n".to_owned()),
        ]
    );
    // with comments

    assert_eq!(
        lexer_output[16..20],
        [
            Token::Comment("\n".to_owned()),
            Token::Comment("central_asian_lakes_area = {\n".to_owned()),
            Token::Comment("	1326 1327 1653 1654 1888\n".to_owned()),
            Token::Comment("}\n".to_owned()),
        ]
    );
}
