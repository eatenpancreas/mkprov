use pdxsyn::{Document, Lexer, Token, syntax::ObjectLike};

fn lex(str: &str) -> Vec<Token> {
    let lexer_output: Result<Vec<_>, _> = Lexer::new(str).collect();
    lexer_output.unwrap()
}

#[test]
fn keyval_insertion_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    let mut root = doc.parse().unwrap();
    root.insert(&mut doc, 0, "event", 100);

    println!("{}", doc.into_string())
}

// #[test]
// fn keyval_insertion_test() {
//     use test_files::simple::KEYVAL as FILE;
//     let tokens = lex(FILE);
//     let mut doc = Document::create(tokens);

//     let mut root = doc.parse().unwrap();
//     let literals: &[Box<dyn IntoLiteral>] = &[Box::new("100"), Box::new(10)];
//     root.insert(&mut doc, 0, "event", literals);

//     println!("{}", doc.into_string())
// }
