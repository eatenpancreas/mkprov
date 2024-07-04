

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


#[macro_export]
/// returns false to the scope if the value is [`Option::None`]
macro_rules! if_none {
    ( $x:expr ) => {
        if let Some(v) = $x {
            v
        } else {
            return false;
        }
    };
}