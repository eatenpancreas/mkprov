use pdxsyn::{Date, Document, Lexer, Token, syntax::ObjectLike};

fn lex(str: &str) -> Vec<Token> {
    let lexer_output: Result<Vec<_>, _> = Lexer::new(str).collect();
    lexer_output.unwrap()
}

#[test]
fn keyval_removal_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    assert_eq!(doc.clone().into_string().as_str(), "width = 6400\nheight = 2560\n");
    let mut root = doc.parse().unwrap();
    root.remove_key(&mut doc, "height");
    assert_eq!(doc.into_string().as_str(), "width = 6400\n");
}

#[test]
fn date_object_removal_test() {
    use test_files::simple::DATE_OBJECT as FILE;
    let mut doc = Document::create(lex(FILE));

    let mut root = doc.parse().unwrap();
    let s = root.get_first_mut(&mut doc, Date::parse_string_unwrapped("2024.06.24"));
    let obj = s.unwrap().as_object_mut().unwrap();
    obj.remove_key(&mut doc, "height");
    obj.remove_key(&mut doc, "event");

    assert_eq!("2024.06.24 = {\n    width = 12.142\n}\n", doc.into_string().as_str());
}

// #[test]
// fn date_object_alter_test() {
//     use test_files::simple::DATE_OBJECT as FILE;
//     let mut doc = Document::create(lex(FILE));

//     let mut root = doc.parse().unwrap();
// }

// #[test]
// fn keyval_parsing_test() {
//     use test_files::simple::WONKY_OBJECT as FILE;
//     let tokens = lex(FILE);
//     let doc = Document::create(tokens);
//     let root = doc.parse().unwrap();
//     println!("{:?}", root.keys(&doc).collect_vec());

//     println!("{}", structure.debug_fmt(&doc));

//     assert_eq!(FILE, doc.into_string().as_str());
// }
