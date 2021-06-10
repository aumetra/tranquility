// Otherwise the compiler will complain when the `activitypub` feature is deactivated
// Since the only code using this function is the ActivityPub code
#![allow(dead_code)]

#[inline]
/// A replacement for `<array>.contains(<value>)` because, for example, the `.contains()` of `Vec<String>` can't be used with an `&str`  
pub fn contains(vec: &[String], value: &str) -> bool {
    vec.iter().any(|entry| entry == value)
}
