#[test]
fn invalid_prefix_test() {
    use pdxsyn::{Date, ParseDateError};
    let result = Date::parse(["2023", "010", "05"]);
    assert!(matches!(result, Err(ParseDateError::TooManyCharacters(_))));
}
