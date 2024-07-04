

#[macro_export]
macro_rules! if_err {
    ( $( $x:expr ),* ) => {
        if let Err(x) = $x {
            
        }
    };
}