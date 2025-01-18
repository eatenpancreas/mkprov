use pdxsyn::{Document, Lexer, Token, syntax::ObjectLike};

fn lex(str: &str) -> Vec<Token> {
    let lexer_output: Result<Vec<_>, _> = Lexer::new(str).collect();
    lexer_output.unwrap()
}

#[test]
fn ages_parse_test() {
    use test_files::massive::AGES as FILE;
    let tokens = lex(FILE);
    let doc = Document::create(tokens);

    let root = doc.parse().unwrap();
    let disc = root.get_first(&doc, "age_of_discovery").unwrap();
    let refo = root.get_first(&doc, "age_of_reformation").unwrap();
    let disc = disc.as_object().unwrap();
    let refo = refo.as_object().unwrap();

    println!("{}", refo.debug_fmt(&doc));

    assert!(disc.has_same_keys_as(&doc, refo));
    assert_eq!(FILE, doc.into_string());
}
