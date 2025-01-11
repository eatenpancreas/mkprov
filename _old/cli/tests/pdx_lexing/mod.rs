use crate::pdx_parsing::files::{COMMENTS_ETC, FULL_FILE_ADAL, OBJECTS, PROBLEMATIC_LEX, SIMPLE};
use paradox_file::{Lexer, Token};

#[test]
fn simple_lex() {
    let idents = Lexer::new(SIMPLE).unwrap_all();
    assert_eq!(
        idents,
        vec![
            Token::new(1, "width", false),
            Token::new(7, "=", false),
            Token::new(9, "6400", false),
            Token::new(14, "height", false),
            Token::new(21, "=", false),
            Token::new(23, "2560", false),
        ]
    )
}

#[test]
fn problematic_lex_lex() {
    let idents = Lexer::new(PROBLEMATIC_LEX).unwrap_all();
    assert_eq!(
        idents,
        vec![
            Token::new(1, "bahama_channel_area", false),
            Token::new(21, "=", false),
            Token::new(23, "{", false),
            Token::new(26, "1503", false),
            Token::new(31, "1505", false),
            Token::new(36, "1524", false),
            Token::new(41, "1525", false),
            Token::new(46, "}", false),
        ]
    )
}

#[test]
fn objects_lex() {
    let idents = Lexer::new(OBJECTS).unwrap_all();
    assert_eq!(
        idents,
        vec![
            Token::new(1, "2024.6.24", false),
            Token::new(12, "=", false),
            Token::new(14, "{", false),
            Token::new(20, "event", false),
            Token::new(26, "=", false),
            Token::new(29, "making of the mod", true),
            Token::new(52, "height", false),
            Token::new(59, "=", false),
            Token::new(61, "2560", false),
            Token::new(70, "width", false),
            Token::new(76, "=", false),
            Token::new(78, "12.142", false),
            Token::new(85, "}", false),
        ]
    )
}

#[test]
fn comments_etc_lex() {
    let idents = Lexer::new(COMMENTS_ETC).unwrap_all();
    assert_eq!(
        idents,
        vec![
            Token::new(15, "width", false),
            Token::new(21, "=", false),
            Token::new(23, "6400", false),
            Token::new(28, "stars", false),
            Token::new(34, "=", false),
            Token::new(37, "In the #sky", true),
        ]
    )
}

#[test]
fn full_file_adal_lex() {
    for ident in Lexer::new(FULL_FILE_ADAL) {
        println!("{ident:#?}");
    }
}
