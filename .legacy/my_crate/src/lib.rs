//! # My Crate
//! 
//! 'my_crate' is a collection of utilities to make perfoming certain
//! calculations more convenient.

/// Adds one to the number given
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let my_answer = my_crate::add_one(arg);
///
/// assert_eq!(6, my_answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}