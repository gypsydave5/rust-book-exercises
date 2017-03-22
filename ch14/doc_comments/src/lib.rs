//! # Documentation Comments
//! The doc_comments crate is a demonstration of all the forms of documentation comments in Rust
//! This, for example, is a 'containing item' comment, designed to describe the module or crate
//! that the items exist in.
/// Adds one to the number given.
///
/// # Example
/// ```
/// let five = 5;
/// assert_eq!(6, add_one(five));
/// # fn add_one(x: i32) -> i32 {
/// # x + 1
/// # }
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
