#[macro_export]
#[doc(hidden)]
/// Invoke `.into()` on all given idents and shadow the variables with the result  
macro_rules! __into {
    ($($var:ident),+) => {
        $(
            let $var = $var.into();
        )+
    }
}
