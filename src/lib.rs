#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![no_std]

// This one is the magic feature flag to allow yeeting in the first place
#![feature(yeet_expr)]


/// Yeet your error case up to the caller.
/// 
/// This is essentially a direct translation to the `do yeet` statement, 
/// but a bit funnier because of being able to type `yeet!` in your code.
/// 
/// ## Example
/// 
/// ```
/// #![feature(yeet_expr)]
/// use yeet_ops::yeet;
/// 
/// /// A function that yeets `None`
/// fn test() -> Option<i32> {
///     yeet!();
/// }
/// 
/// # fn main() {
/// // Did it yeet?
/// assert_eq!(test(), None);
/// # }
/// ```
#[macro_export]
macro_rules! yeet {
    // Expressions can be yote
    ($expr:expr) => {
        do yeet $expr
    };
    // If nothing is yote, the `do yeet` should return the default value? Basically just `None` afaik
    () => {
        do yeet
    };
}
