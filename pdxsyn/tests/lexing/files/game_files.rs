use pdxsyn::Lexer;

#[test]
fn adal_lexing_test() {
    use test_files::game_files::ADAL;
    let lexer_output: Result<Vec<_>, _> = Lexer::new(ADAL).collect();
    let lexer_output = lexer_output.unwrap();
    println!("{lexer_output:?}");
}
