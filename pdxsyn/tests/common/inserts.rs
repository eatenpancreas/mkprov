use pdxsyn::{
    Document, Lexer, Token,
    syntax::{ArrayBuilder, DebugFmt, ObjectBuilder, ObjectLike},
};

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

    assert!(root.has_key(&doc, "event"));
    assert_eq!(root.iter_key_indices(&doc, "event").next().unwrap(), 0);
}

#[test]
fn keyval_insertion_array_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    let mut root = doc.parse().unwrap();
    let mut arr = ArrayBuilder::new();
    arr.push(100);
    arr.push("zamn");
    root.insert(&mut doc, 0, "event", arr);

    println!("{}", doc.into_string())
}

#[test]
fn keyval_insertion_complex_array_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    let mut root = doc.parse().unwrap();
    let mut arr = ArrayBuilder::new();
    arr.push(100);
    arr.push(Lexer::lex_literal("2024.1.1").unwrap());
    root.insert(&mut doc, 0, "event", arr);

    let arr = root
        .get_first_mut(&mut doc, "event")
        .unwrap()
        .as_array_mut()
        .unwrap();

    println!("{}", doc.into_string())
}

#[test]
fn keyval_insertion_object_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    let mut root = doc.parse().unwrap();
    let mut obj = ObjectBuilder::new();
    obj.push(1, 100);
    obj.push("aaaa", 100);
    root.insert(&mut doc, 1, "event", obj);
    println!("{}", doc.into_string())
}

#[test]
fn keyval_insertion_complex_object_test() {
    use test_files::simple::KEYVAL as FILE;
    let tokens = lex(FILE);
    let mut doc = Document::create(tokens);

    let mut root = doc.parse().unwrap();
    let mut obj = ObjectBuilder::new();
    obj.push(1, 100);
    obj.push(
        "aaaa",
        ObjectBuilder::new()
            .with("key", "value")
            .with("key", ObjectBuilder::new().with("x", "b")),
    );
    root.insert(&mut doc, 1, "event", obj);

    let event = root.get_first_mut(&mut doc, "event").unwrap();
    let event_obj = event.as_object_mut().unwrap();
    event_obj.insert(&mut doc, 0, "2", 300);

    println!("{}", root.debug_fmt(&doc));
    println!("---\n{}", doc.into_string());
}
