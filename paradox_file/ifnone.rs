

#[macro_export]
/// returns false to the scope if the value is [`Result::Err`]
macro_rules! if_err {
    ( $x:expr ) => {
        if let Ok(v) = $x {
            v
        } else {
            return false;
        }
    };
}