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

#[macro_export]
/// Wrap variables into `Cow`s
///
/// "Why not just use .into()?": `.into()` will work for *some* borrowed types, not all.
/// For owned types, there are no `From` implementation at all  
///
/// This macro works for all types, borrowed or owned
///
/// Example:
/// ```
/// let str1 = "hello";
/// let str2 = "world";
///
/// // Wrap the two `&str`s into `Cow<'static, str>`s
/// wrap_cow!(Borrowed; str1, str2);
/// ```
macro_rules! wrap_cow {
    ($cow_type:ident; $($var:ident),+) => {
        $(
            let $var = std::borrow::Cow::$cow_type($var);
        )+
    }
}

#[macro_export]
/// Wrap the content of `Option`s in `Cow`s
///
/// Example:
/// ```
/// let maybe_str1: Option<&str> = Some("hello");
/// let maybe_str2: Option<&str> = None;
///
/// // Wrap the two `Option<&str>` s into `Option<Cow<'static, str>>`s
/// wrap_cow_option!(Borrowed; maybe_str1, maybe_str2);
/// ```
macro_rules! wrap_cow_option {
    ($cow_type:ident; $($var:ident),+) => {
        $(
            let $var = $var.map(std::borrow::Cow::$cow_type);
        )+
    }
}
